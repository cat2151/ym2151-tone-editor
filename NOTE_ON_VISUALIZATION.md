# YM2151 Tone Editor - Note On Event Visualization

## 🎵 Issue #11: Note On Events Now Included in JSON Output

This document visualizes the changes made to address the issue where changing tones didn't produce sound. The root cause was missing Note On events in the generated JSON.

## 📊 Before vs After Comparison

### Before (25 events)
The JSON only contained operator and channel configuration, but **no note trigger**:
- 24 operator register events (4 operators × 6 registers)
- 1 channel algorithm event
- **Missing**: Note On, Key Code, Key Fraction

### After (28 events)
The JSON now contains **complete YM2151 playback data**:
- 24 operator register events (4 operators × 6 registers)
- 1 channel algorithm event
- **✨ NEW**: 1 Key Code event (note pitch)
- **✨ NEW**: 1 Key Fraction event (fine tuning)
- **✨ NEW**: 1 Note On event (**triggers the sound!**)

## 🎹 Critical Note On Event

The most important addition is the **Note On event at register 0x08**:

```json
{
  "time": 0,
  "addr": "0x08",
  "data": "0x78"
}
```

### Breakdown of 0x78:
- Binary: `01111000`
- Bits 3-6 (value `0x78`): Enable all 4 operators (M1, M2, C1, C2)
- Bits 0-2 (value `0x00`): Channel 0

This event **triggers the YM2151 chip to start playing the note** with the configured tone parameters.

## 🔊 Supporting Events

### Key Code (Register 0x28)
```json
{
  "time": 0,
  "addr": "0x28",
  "data": "0x4C"
}
```
Sets the note frequency. Value `0x4C` corresponds to approximately **middle C**.

### Key Fraction (Register 0x30)
```json
{
  "time": 0,
  "addr": "0x30",
  "data": "0x00"
}
```
Fine-tunes the frequency. Set to `0x00` for no adjustment.

## 🎚️ Carrier TL Fixed to 0

As a provisional specification for clarity, **carrier operators now have TL=0** (maximum volume).

### Algorithm-Based Carrier Detection

Different algorithms use different operators as carriers:

| Algorithm | Carrier Operators | Pattern |
|-----------|-------------------|---------|
| 0, 1, 2, 3 | OP4 only | `[false, false, false, true]` |
| 4 | OP2, OP4 | `[false, true, false, true]` |
| 5, 6 | OP2, OP3, OP4 | `[false, true, true, true]` |
| 7 | All (OP1-OP4) | `[true, true, true, true]` |

### Example: Algorithm 4
With Algorithm 4, the TL registers will be:
- OP1 (0x60): Uses configured TL value (modulator)
- **OP2 (0x68): TL=0** (carrier - maximum volume)
- OP3 (0x70): Uses configured TL value (modulator)
- **OP4 (0x78): TL=0** (carrier - maximum volume)

## 📋 Complete JSON Example

Here's what a complete YM2151 tone now looks like:

```json
{
  "event_count": 28,
  "events": [
    // Operator 1
    { "time": 0, "addr": "0x40", "data": "0x01" },  // DT1/MUL
    { "time": 0, "addr": "0x60", "data": "0x14" },  // TL
    { "time": 0, "addr": "0x80", "data": "0x1F" },  // KS/AR
    { "time": 0, "addr": "0xA0", "data": "0x0A" },  // D1R
    { "time": 0, "addr": "0xC0", "data": "0x05" },  // D2R
    { "time": 0, "addr": "0xE0", "data": "0x57" },  // D1L/RR
    
    // Operator 2
    { "time": 0, "addr": "0x48", "data": "0x01" },  // DT1/MUL
    { "time": 0, "addr": "0x68", "data": "0x1E" },  // TL
    { "time": 0, "addr": "0x88", "data": "0x19" },  // KS/AR
    { "time": 0, "addr": "0xA8", "data": "0x08" },  // D1R
    { "time": 0, "addr": "0xC8", "data": "0x04" },  // D2R
    { "time": 0, "addr": "0xE8", "data": "0x66" },  // D1L/RR
    
    // Operator 3
    { "time": 0, "addr": "0x50", "data": "0x02" },  // DT1/MUL
    { "time": 0, "addr": "0x70", "data": "0x28" },  // TL
    { "time": 0, "addr": "0x90", "data": "0x14" },  // KS/AR
    { "time": 0, "addr": "0xB0", "data": "0x06" },  // D1R
    { "time": 0, "addr": "0xD0", "data": "0x03" },  // D2R
    { "time": 0, "addr": "0xF0", "data": "0x75" },  // D1L/RR
    
    // Operator 4 (Carrier in algorithm 4)
    { "time": 0, "addr": "0x58", "data": "0x01" },  // DT1/MUL
    { "time": 0, "addr": "0x78", "data": "0x00" },  // TL=0 (carrier!)
    { "time": 0, "addr": "0x98", "data": "0x16" },  // KS/AR
    { "time": 0, "addr": "0xB8", "data": "0x07" },  // D1R
    { "time": 0, "addr": "0xD8", "data": "0x04" },  // D2R
    { "time": 0, "addr": "0xF8", "data": "0x66" },  // D1L/RR
    
    // Channel Configuration
    { "time": 0, "addr": "0x20", "data": "0xC4" },  // RL/FB/CON (Algorithm 4)
    
    // Note Frequency
    { "time": 0, "addr": "0x28", "data": "0x4C" },  // ⭐ Key Code (middle C)
    { "time": 0, "addr": "0x30", "data": "0x00" },  // ⭐ Key Fraction
    
    // Sound Trigger
    { "time": 0, "addr": "0x08", "data": "0x78" }   // ⭐ NOTE ON (all ops enabled)
  ]
}
```

## ✅ Tests Added

Two comprehensive tests verify the implementation:

### 1. `test_to_ym2151_events()`
- ✅ Verifies 28 events are generated
- ✅ Confirms note on event (0x08) is present
- ✅ Validates event format

### 2. `test_carrier_tl_is_zero()`
- ✅ Tests all 8 algorithms
- ✅ Verifies carrier operators have TL=0
- ✅ Confirms modulator operators preserve their TL values

## 🎯 Summary

The JSON output now includes **everything needed for the YM2151 chip to play sound**:
1. ✅ Operator configuration (tone parameters)
2. ✅ Channel algorithm
3. ✅ Note frequency (Key Code + Key Fraction)
4. ✅ **Note On trigger** - This was missing before!
5. ✅ Carrier TL optimization (set to 0 for clarity)

**Result**: When the tone editor saves or plays tones, they will now actually produce sound! 🎉
