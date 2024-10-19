# Santroller HID Rock Band Guitars

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0400`
- Device name: `Santroller`

## Input Info

Length: 8 bytes

- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 1, bit 0 (`0x01`) - Green
  - Byte 1, bit 1 (`0x02`) - Red
  - Byte 1, bit 2 (`0x04`) - Yellow
  - Byte 1, bit 3 (`0x08`) - Blue
  - Byte 1, bit 4 (`0x10`) - Orange
  - Byte 1, bit 5 (`0x20`) - Solo Green
  - Byte 1, bit 6 (`0x40`) - Solo Red
  - Byte 1, bit 7 (`0x80`) - Solo Yellow
  - Byte 2, bit 0 (`0x01`) - Solo Blue
  - Byte 2, bit 1 (`0x02`) - Solo Orange
  - Byte 2, bit 2 (`0x04`) - Select button
  - Byte 2, bit 3 (`0x08`) - Start button
  - Byte 2, bit 4 (`0x10`) - Home button
  - Byte 2, bit 5-7 - Unused
- Byte 3 - D-pad
  - Same format as the PS3 guitars. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
    Visual representation:

    ```
        0
      7   1
    6   8   2
      5   3
        4
    ```

- Byte 4 - Whammy
  - Full range: resting at 0, fully pressed at 255.
- Byte 5 - Pickup
  - See the [General Notes](General%20Notes.md) doc for more details.
- Byte 6 - Tilt
  - Full range: centered at `0x80`, tilted up all the way at 255, tilted all the way down at 0.

### As A Struct

```cpp
struct SantrollerRockBandGuitarState
{
    uint8_t reportId = 0x01;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;

    bool orange : 1;
    bool soloGreen : 1;
    bool soloRed : 1;
    bool soloYellow : 1;

    bool soloBlue : 1;
    bool soloOrange : 1;
    bool select : 1;
    bool start : 1;

    bool home : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum;

    uint8_t whammy;
    uint8_t pickup;
    uint8_t tilt;
} __attribute__((packed)); // 7 bytes
```
