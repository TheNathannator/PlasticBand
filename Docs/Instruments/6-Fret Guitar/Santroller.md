# Santroller HID Guitar Hero Live Guitars

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0700`
- Device name: `Santroller`

## Input Info

Length: 6 bytes

- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 1, bit 0 (`0x01`) - Black 1
  - Byte 1, bit 1 (`0x02`) - Black 2
  - Byte 1, bit 2 (`0x04`) - Black 3
  - Byte 1, bit 3 (`0x08`) - White 1
  - Byte 1, bit 4 (`0x10`) - White 2
  - Byte 1, bit 5 (`0x20`) - White 3
  - Byte 1, bit 6 (`0x40`) - Select button / Hero Power
  - Byte 1, bit 7 (`0x80`) - Start button / Pause Button
  - Byte 2, bit 0 (`0x01`) - GHTV button
  - Byte 2, bit 1 (`0x02`) - Home button
  - Byte 2, bit 2-7 - Unused
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
- Byte 5 - Tilt
  - Full range: centered at `0x80`, tilted up all the way at 255, tilted all the way down at 0.

### As A Struct

```cpp
struct SantrollerSixFretGuitarState
{
    uint8_t reportId = 0x01;

    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;
    bool white1 : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool heroPower : 1;
    bool pause : 1;

    bool ghtv : 1;
    bool home: 1;
    uint8_t : 6;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum;

    uint8_t whammy;
    uint8_t tilt;
} __attribute__((__packed__)); // 6 bytes
```
