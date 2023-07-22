# Santroller HID Guitar Hero Guitars

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0300`
- Device name: `Santroller`

## Input Info

Length: 7 bytes
- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Green
  - Byte 0, bit 1 (`0x02`) - Red
  - Byte 0, bit 2 (`0x04`) - Yellow
  - Byte 0, bit 3 (`0x08`) - Blue
  - Byte 0, bit 4 (`0x10`) - Orange
  - Byte 0, bit 5 (`0x20`) - Pedal
  - Byte 0, bit 6 (`0x40`) - Select button
  - Byte 0, bit 7 (`0x80`) - Start button
  - Byte 1, bit 0 (`0x01`) - Home button
  - Byte 1, bit 1-7 - Unused
- Byte 3 - Dpad
  - Same format as a ps3 controller
  - This value is not a bitmask, rather it encodes different possible states as individual numbers.
    Visual representation:
        0
      7   1
    6   8   2
      5   3
        4
- Byte 4 - Whammy
  - Full range, 0-255
- Byte 5 - Tap bar
  - Full range, 0-255
- Byte 6 - Tilt
  - Full range, 0-255

### As A Struct

```cpp
struct SantrollerGuitarHeroGuitarState
{
    uint8_t reportId;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;

    bool orange : 1;
    bool kick : 1;
    bool select: 1;
    bool start : 1;

    bool home: 1;
    uint8_t : 7;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum;

    uint8_t whammy;
    uint8_t tapBar;
    uint8_t tilt;
}
```
