/**
 * random-tone.js
 *
 * Pure-JavaScript implementation of YM2151 random tone register generation.
 *
 * This module is a drop-in alternative to the WASM export
 * `generate_random_tone_registers`.  It produces byte-for-byte identical
 * output to the WASM version but has **no external dependencies** — it does
 * not load any `.wasm` binary and does not fetch from any external URL.
 *
 * ## Why this file exists
 *
 * The WASM version (`pkg/ym2151_wasm.js`) must be initialised with
 * `await init()` and, in some hosting setups such as Chrome extensions that
 * vendor the library, the `.wasm` binary cannot be loaded from an external
 * URL due to Content Security Policy restrictions.  This file solves that
 * problem: vendor `random-tone.js` locally and call
 * `generateRandomToneRegisters(seed, note)` synchronously.
 *
 * ## Usage
 *
 * ```js
 * import { generateRandomToneRegisters } from './random-tone.js';
 *
 * const seed = Date.now();   // any Number — different seeds → different tones
 * const note = 69;           // MIDI note number (69 = A4)
 * const registers = generateRandomToneRegisters(seed, note);
 * // e.g. "4000600080001F..." — same format as the WASM export
 * console.log(registers);
 * ```
 *
 * ## Algorithm
 *
 * Mirrors the Rust implementation in `core/src/lib.rs` exactly:
 *   - LCG with multiplier 6364136223846793005 and addend 1442695040888963407
 *   - Parameter ranges matching `getDefaultConfig()` in web-ym2151
 *   - Register encoding identical to `editor_rows_to_registers()`
 *
 * BigInt arithmetic is used for the 64-bit LCG to avoid floating-point
 * precision loss.
 */

// ---------------------------------------------------------------------------
// LCG constants (must match SimpleRng in core/src/lib.rs)
// ---------------------------------------------------------------------------

const LCG_MUL = 6364136223846793005n;
const LCG_ADD = 1442695040888963407n;
const U64_MASK = 0xFFFFFFFFFFFFFFFFn; // 2^64 − 1

// ---------------------------------------------------------------------------
// Parameter indices (must match PARAM_* constants in core/src/lib.rs)
// ---------------------------------------------------------------------------

const PARAM_SM  = 0;
const PARAM_TL  = 1;
const PARAM_MUL = 2;
const PARAM_AR  = 3;
const PARAM_D1R = 4;
const PARAM_D1L = 5;
const PARAM_D2R = 6;
const PARAM_RR  = 7;
const PARAM_DT  = 8;
const PARAM_DT2 = 9;
const PARAM_KS  = 10;
const PARAM_AMS = 11;
const GRID_WIDTH = 12;

// CH_PARAM_* and ROW_CH are used in the Rust 2D grid; the JS implementation
// tracks ALG/FB as plain variables instead, so those indices are not needed.
// ---------------------------------------------------------------------------
// Hardware layout constants (must match core/src/lib.rs)
// ---------------------------------------------------------------------------

/** Maps display operator index (0-3) to YM2151 hardware register slot. */
const REG_FROM_O1_O4 = [0, 2, 1, 3];

/** Which operators are carriers for each ALG value (0-7). */
const CARRIERS_PER_ALG = [
  [false, false, false, true],  // ALG=0
  [false, false, false, true],  // ALG=1
  [false, false, false, true],  // ALG=2
  [false, false, false, true],  // ALG=3
  [false, false, true,  true],  // ALG=4
  [false, true,  true,  true],  // ALG=5
  [false, true,  true,  true],  // ALG=6
  [true,  true,  true,  true],  // ALG=7
];

/** Modulator TL value per ALG (stage_count × 0x08). */
const MODULATOR_TL_PER_ALG = [0x20, 0x20, 0x20, 0x20, 0x18, 0x10, 0x10, 0x00];

/** YM2151 note encoding: 12 semitones mapped to 14 hardware values. */
const NOTE_MAP = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];

// ---------------------------------------------------------------------------
// LCG pseudo-random number generator (mirrors SimpleRng in core/src/lib.rs)
// ---------------------------------------------------------------------------

/**
 * Seed-based LCG state initialisation.
 *
 * Mirrors `seed.abs() as u64` in the WASM wrapper, which uses Rust's
 * saturating float-to-integer cast semantics:
 *   - NaN              → 0
 *   - ±Infinity or any value ≥ 2^64  → u64::MAX (2^64 − 1)
 *   - finite negative  → trunc(abs(seed))   (same as positive mirror)
 *   - finite positive  → trunc(seed)
 *
 * @param {number} seed - Any JS Number.
 * @returns {bigint} Initial LCG state.
 */
function lcgInit(seed) {
  const abs = Math.abs(seed);
  let s;
  if (isNaN(abs)) {
    s = 0n;
  } else if (!isFinite(abs) || abs >= 18446744073709551616) {
    // 18446744073709551616 === 2**64, exactly representable as float64
    s = U64_MASK; // saturate to u64::MAX
  } else {
    s = BigInt(Math.trunc(abs));
  }
  return (s * LCG_MUL + LCG_ADD) & U64_MASK;
}

/**
 * Advance the LCG by one step.
 *
 * @param {bigint} state
 * @returns {bigint} Next state.
 */
function lcgNext(state) {
  return (state * LCG_MUL + LCG_ADD) & U64_MASK;
}

/**
 * Draw a uniform integer in [min, max] from the LCG.
 *
 * When `min >= max` the function returns `min` **without advancing the LCG
 * state** — exactly mirroring `SimpleRng::range` in `core/src/lib.rs`, which
 * performs an early return before calling `next_u64`.
 *
 * @param {bigint} state   Current LCG state.
 * @param {number} min     Inclusive lower bound.
 * @param {number} max     Inclusive upper bound (must be ≥ min in normal use).
 * @returns {{ state: bigint, value: number }}
 */
function lcgRange(state, min, max) {
  if (min >= max) return { state, value: min };
  const next = lcgNext(state);
  const span = BigInt(max - min + 1);
  return { state: next, value: min + Number(next % span) };
}

// ---------------------------------------------------------------------------
// MIDI → YM2151 pitch conversion (mirrors midi_to_kc_kf in core/src/lib.rs)
// ---------------------------------------------------------------------------

/**
 * Convert a MIDI note number to a YM2151 (KC, KF) pair.
 *
 * @param {number} midiNote - MIDI note number 0-127.
 * @returns {[number, number]} [KC, KF] — KF is always 0.
 */
function midiToKcKf(midiNote) {
  const adjusted = midiNote > 0 ? midiNote - 1 : 0;
  const noteInOctave = adjusted % 12;
  const ymOctave = Math.min(Math.max(Math.floor(adjusted / 12) - 2, 0), 7);
  return [(ymOctave << 4) | NOTE_MAP[noteInOctave], 0];
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/** Format a byte as a two-digit uppercase hex string. */
function hex2(n) {
  return (n & 0xFF).toString(16).toUpperCase().padStart(2, '0');
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Generate a random YM2151 tone and return it as a register hex string.
 *
 * The output is byte-for-byte identical to the WASM export
 * `generate_random_tone_registers(seed, currentNote)`.
 *
 * @param {number} seed        - Numeric seed (e.g. `Date.now()`). Different
 *   seeds produce different tones; the same seed always produces the same tone.
 * @param {number} currentNote - MIDI note number 0-127 (use 69 for A4).
 * @returns {string} Hex string of register address/data pairs, 4 chars each
 *   (e.g. `"4000600080001F..."`).
 */
export function generateRandomToneRegisters(seed, currentNote) {
  // --- Initialise LCG (mirrors SimpleRng::from_seed) ---
  let state = lcgInit(seed);

  // --- ALG (0-7) ---
  let res = lcgRange(state, 0, 7);
  state = res.state;
  const alg = res.value;
  const modulatorTl = MODULATOR_TL_PER_ALG[alg];
  // The min() mirrors `modulator_tl.min(PARAM_MAX[PARAM_TL])` in the Rust source.
  // All MODULATOR_TL_PER_ALG values are ≤ 0x20 (32), well below TL_MAX (99),
  // so this has no practical effect — it is kept for parity with the Rust code.

  // --- Operator parameters ---
  // Each row: [SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS]
  const ops = [];
  for (let op = 0; op < 4; op++) {
    const isCarrier = CARRIERS_PER_ALG[alg][op];
    const row = new Array(GRID_WIDTH).fill(0);

    row[PARAM_SM]  = 1;
    row[PARAM_TL]  = isCarrier ? 0 : modulatorTl;

    res = lcgRange(state,  0, 15); state = res.state; row[PARAM_MUL] = res.value;
    res = lcgRange(state,  5, 31); state = res.state; row[PARAM_AR]  = res.value;
    res = lcgRange(state,  0,  9); state = res.state; row[PARAM_D1R] = res.value;

    row[PARAM_D1L] = 15;
    row[PARAM_D2R] = 0;
    row[PARAM_RR]  = 0;

    res = lcgRange(state,  0,  7); state = res.state; row[PARAM_DT]  = res.value;

    row[PARAM_DT2] = 0;

    res = lcgRange(state,  0,  3); state = res.state; row[PARAM_KS]  = res.value;

    row[PARAM_AMS] = 0;

    ops.push(row);
  }

  // --- FB (0-7) ---
  res = lcgRange(state, 0, 7);
  const fb = res.value;

  // --- Encode registers (mirrors editor_rows_to_registers) ---
  let out = '';
  const channel = 0;

  for (let rowId = 0; rowId < 4; rowId++) {
    const opOff = REG_FROM_O1_O4[rowId] * 8 + channel;
    const row = ops[rowId];

    // DT1/MUL — 0x40
    out += hex2(0x40 + opOff) + hex2(((row[PARAM_DT] & 0x07) << 4) | (row[PARAM_MUL] & 0x0F));
    // TL — 0x60
    out += hex2(0x60 + opOff) + hex2(row[PARAM_TL] & 0x7F);
    // KS/AR — 0x80
    out += hex2(0x80 + opOff) + hex2(((row[PARAM_KS] & 0x03) << 6) | (row[PARAM_AR] & 0x1F));
    // AMS/D1R — 0xA0
    out += hex2(0xA0 + opOff) + hex2(((row[PARAM_AMS] & 0x03) << 6) | (row[PARAM_D1R] & 0x1F));
    // DT2/D2R — 0xC0
    out += hex2(0xC0 + opOff) + hex2(((row[PARAM_DT2] & 0x03) << 6) | (row[PARAM_D2R] & 0x0F));
    // D1L/RR — 0xE0
    out += hex2(0xE0 + opOff) + hex2(((row[PARAM_D1L] & 0x0F) << 4) | (row[PARAM_RR] & 0x0F));
  }

  // RL/FB/CON — 0x20
  out += hex2(0x20 + channel) + hex2(0xC0 | ((fb & 0x07) << 3) | (alg & 0x07));

  // KC — 0x28
  // `& 0xFF` mirrors Rust's `current_note: u8` — ensures values outside 0-255
  // are truncated the same way as Rust's unsigned byte cast.
  const [kc, kf] = midiToKcKf(currentNote & 0xFF);
  out += hex2(0x28 + channel) + hex2(kc);
  // KF — 0x30
  out += hex2(0x30 + channel) + hex2(kf);

  // Key On — 0x08
  const slotMask = (ops[0][PARAM_SM] ? 0x08 : 0)
                 | (ops[1][PARAM_SM] ? 0x10 : 0)
                 | (ops[2][PARAM_SM] ? 0x20 : 0)
                 | (ops[3][PARAM_SM] ? 0x40 : 0);
  out += hex2(0x08) + hex2(slotMask | channel);

  return out;
}
