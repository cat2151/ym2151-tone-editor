use super::*;

// ------------------------------------------------------------------
// SimpleRng
// ------------------------------------------------------------------

#[test]
fn test_simple_rng_deterministic() {
    let mut a = SimpleRng::from_seed(42);
    let mut b = SimpleRng::from_seed(42);
    assert_eq!(a.next_u64(), b.next_u64());
    assert_eq!(a.next_u64(), b.next_u64());
}

#[test]
fn test_simple_rng_range_in_bounds() {
    let mut rng = SimpleRng::from_seed(999);
    for _ in 0..100 {
        let v = rng.range(5, 31);
        assert!((5..=31).contains(&v));
    }
}

#[test]
fn test_simple_rng_range_equal_min_max() {
    let mut rng = SimpleRng::from_seed(1);
    assert_eq!(rng.range(7, 7), 7);
}

// ------------------------------------------------------------------
// midi_to_kc_kf
// ------------------------------------------------------------------

#[test]
fn test_midi_to_kc_kf_kf_always_zero() {
    for note in 0u8..=127 {
        let (_, kf) = midi_to_kc_kf(note);
        assert_eq!(kf, 0, "KF must always be 0 for MIDI note {}", note);
    }
}

#[test]
fn test_midi_to_kc_kf_middle_c() {
    let (kc, _) = midi_to_kc_kf(60);
    // Middle C (MIDI 60) → adjusted=59 → octave=4, note=11 → YM2151 octave=(4-2)=2, kc_note=NOTE_MAP[11]=14
    assert_eq!(
        kc,
        (2 << 4) | 14,
        "Unexpected KC for middle C: 0x{:02X}",
        kc
    );
}

#[test]
fn test_midi_to_kc_kf_a4() {
    let (kc, _) = midi_to_kc_kf(69);
    // A4 (MIDI 69) → adjusted=68 → octave=5, note=8 → YM2151 octave=(5-2)=3, kc_note=NOTE_MAP[8]=10
    assert_eq!(kc, (3 << 4) | 10, "Unexpected KC for A4: 0x{:02X}", kc);
}

#[test]
fn test_midi_to_kc_kf_kc_within_valid_range() {
    for note in 0u8..=127 {
        let (kc, _) = midi_to_kc_kf(note);
        // YM2151 KC max: octave=7 (bits 6-4), note=14 (bits 3-0) → 0x7E
        assert!(
            kc <= 0x7E,
            "KC out of valid YM2151 range for MIDI {}: 0x{:02X}",
            note,
            kc
        );
    }
}

// ------------------------------------------------------------------
// generate_random_tone_with_seed
// ------------------------------------------------------------------

#[test]
fn test_generate_random_tone_with_seed_deterministic() {
    let a = generate_random_tone_with_seed(42, 69);
    let b = generate_random_tone_with_seed(42, 69);
    assert_eq!(a, b);
}

#[test]
fn test_generate_random_tone_with_seed_different_seeds() {
    let a = generate_random_tone_with_seed(1, 69);
    let b = generate_random_tone_with_seed(2, 69);
    assert_ne!(a, b);
}

#[test]
fn test_generate_random_tone_with_seed_note_preserved() {
    let tone = generate_random_tone_with_seed(0, 60);
    assert_eq!(tone[ROW_CH][CH_PARAM_NOTE], 60);
}

#[test]
fn test_generate_random_tone_with_seed_alg_in_range() {
    let tone = generate_random_tone_with_seed(12345, 69);
    assert!(tone[ROW_CH][CH_PARAM_ALG] <= 7);
    assert!(tone[ROW_CH][CH_PARAM_FB] <= 7);
}

#[test]
fn test_generate_random_tone_with_seed_carrier_tl_zero() {
    let tone = generate_random_tone_with_seed(7777, 69);
    let alg = tone[ROW_CH][CH_PARAM_ALG] as usize;
    for (op, row) in tone.iter().take(4).enumerate() {
        if CARRIERS_PER_ALG[alg][op] {
            assert_eq!(
                row[PARAM_TL],
                0,
                "Carrier OP{} TL must be 0 for ALG={}",
                op + 1,
                alg
            );
        } else {
            let expected = MODULATOR_TL_PER_ALG[alg];
            assert_eq!(
                row[PARAM_TL],
                expected,
                "Modulator OP{} TL mismatch for ALG={}",
                op + 1,
                alg
            );
        }
    }
}

#[test]
fn test_generate_random_tone_with_seed_ar_in_range() {
    let tone = generate_random_tone_with_seed(42, 60);
    for (op, row) in tone.iter().take(4).enumerate() {
        assert!(
            (5..=31).contains(&row[PARAM_AR]),
            "AR out of range for OP{}: {}",
            op + 1,
            row[PARAM_AR]
        );
    }
}

// ------------------------------------------------------------------
// editor_rows_to_registers
// ------------------------------------------------------------------

#[test]
fn test_editor_rows_to_registers_hex_format() {
    let tone = generate_random_tone_with_seed(12345, 69);
    let regs = editor_rows_to_registers(&tone);
    assert!(!regs.is_empty());
    assert_eq!(
        regs.len() % 4,
        0,
        "Register string must be multiple of 4 chars"
    );
    assert!(
        regs.chars().all(|c| c.is_ascii_hexdigit()),
        "All chars must be hex digits: {}",
        regs
    );
}

#[test]
fn test_editor_rows_to_registers_includes_kc_and_kf() {
    let tone = generate_random_tone_with_seed(1234, 69);
    let regs = editor_rows_to_registers(&tone);
    let chars: Vec<char> = regs.chars().collect();
    let mut found_kc = false;
    let mut found_kf = false;
    for chunk in chars.chunks(4) {
        let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
        let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
        if (0x28..=0x2F).contains(&addr) {
            found_kc = true;
            assert!(data <= 0x7E, "KC out of range: 0x{:02X}", data);
        }
        if (0x30..=0x37).contains(&addr) {
            found_kf = true;
            assert_eq!(data, 0, "KF must be 0, got 0x{:02X}", data);
        }
    }
    assert!(found_kc, "KC register (0x28) missing from output");
    assert!(found_kf, "KF register (0x30) missing from output");
}

#[test]
fn test_editor_rows_to_registers_deterministic() {
    let tone = generate_random_tone_with_seed(999, 60);
    let r1 = editor_rows_to_registers(&tone);
    let r2 = editor_rows_to_registers(&tone);
    assert_eq!(r1, r2);
}
