# Santroller HID Stage Kit

While there was never a PS3 or Wii version of the stage kit, a Santroller version is available anyways since it doesn't make sense to limit it to only XInput mode.

## Device Info

- Vendor/product ID:
  - USB: `1209:2882`
  - Bluetooth:  `1209:2885`
- Revision: `0x0900`
- Device name: `Santroller`

## Input Info

Length: 8 bytes

- Byte 0: Report ID (always 1)
- Byte 1: 8-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - A / × button
  - Byte 0, bit 1 (`0x02`) - B / ○ button
  - Byte 0, bit 2 (`0x04`) - X / □ button
  - Byte 0, bit 3 (`0x08`) - Y / Δ button
  - Byte 0, bit 4 (`0x10`) - Select button
  - Byte 0, bit 5 (`0x20`) - Start button
  - Byte 0, bit 6 (`0x40`) - Home button
  - Byte 0, bit 7 (`0x80`) - Not used
- Byte 3 - D-pad
  - Same format as the PS3 instruments. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
    Visual representation:

    ```
        0
      7   1
    6   8   2
      5   3
        4
    ```

### As A Struct

```cpp
struct SantrollerStageKitState
{
    uint8_t reportId;

    bool a : 1; // cross
    bool b : 1; // circle
    bool x : 1; // square
    bool y : 1; // triangle

    bool select : 1;
    bool start : 1;
    bool home : 1;
    bool : 1;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;
} __attribute__((__packed__));
```
