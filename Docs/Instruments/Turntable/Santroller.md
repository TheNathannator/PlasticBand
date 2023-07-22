# Santroller HID DJ Hero Turntable

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x1000`
- Device name: `Santroller`

## Input Info

Length: 8 bytes
- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - A Button
  - Byte 0, bit 1 (`0x02`) - B Button
  - Byte 0, bit 2 (`0x04`) - X Button
  - Byte 0, bit 3 (`0x08`) - Y Button
  - Byte 0, bit 4 (`0x10`) - Select button
  - Byte 0, bit 5 (`0x20`) - Start button
  - Byte 0, bit 6 (`0x40`) - Home button
  - Byte 0, bit 7 (`0x80`) - Right table green
  - Byte 1, bit 0 (`0x01`) - Right table red
  - Byte 1, bit 1 (`0x02`) - Right table blue
  - Byte 1, bit 2 (`0x04`) - Left table green
  - Byte 1, bit 3 (`0x08`) - Left table red
  - Byte 1, bit 4 (`0x10`) - Left table blue
  - Byte 1, bits 5-7 - Unused
- Byte 3 - Dpad
  - Same format as a ps3 controller
  - This value is not a bitmask, rather it encodes different possible states as individual numbers.
    Visual representation:
        0
      7   1
    6   8   2
      5   3
        4
- Byte 4 - Left table velocity
  - Full range, 0-255
- Byte 5 - Right table velocity
  - Full range, 0-255
- Byte 6 - Effects knob
  - Full range, 0-255
- Byte 7 - Crossfader
  - Full range, 0-255

### As A Struct

```cpp
struct SantrollerTurntableState
{
    uint8_t reportId;

    bool a : 1;
    bool b : 1;
    bool x : 1;
    bool y : 1;

    bool select : 1;
    bool start : 1;
    bool home: 1;
    bool rightGreen : 1;

    bool rightRed : 1;
    bool rightBlue: 1;
    bool leftGreen : 1;
    bool leftRed : 1;

    bool leftBlue: 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t leftTableVelocity;
    uint8_t rightTableVelocity;
    uint8_t effectsKnob;
    uint8_t crossfader;
}
```
