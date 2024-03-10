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
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - A / × button
  - Byte 0, bit 1 (`0x02`) - B / ○ button
  - Byte 0, bit 2 (`0x04`) - X / □ button
  - Byte 0, bit 3 (`0x08`) - Y / Δ button
  - Byte 0, bit 4 (`0x10`) - Left bumper / L1
  - Byte 0, bit 5 (`0x20`) - Right bumper / R1
  - Byte 0, bit 6 (`0x40`) - Left trigger press / L2 press
  - Byte 0, bit 7 (`0x80`) - Right trigger press / R2 press

  - Byte 1, bit 0 (`0x01`) - Select button
  - Byte 1, bit 1 (`0x02`) - Start button
  - Byte 1, bit 2 (`0x04`) - Left stick click / L3
  - Byte 1, bit 3 (`0x08`) - Right stick click / R3
  - Byte 1, bit 4 (`0x10`) - Guide button
  - Byte 1, bit 5 (`0x20`) - Capture button
  - Byte 1, bit 6-7 - Not used
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
- Byte 4: Left stick X
  - Neutral at 0x80, left at 0x00, right at 0xFF
- Byte 5: Left stick Y
  - Neutral at 0x80, top at 0x00, bottom at 0xFF
- Byte 6: Right stick X
- Byte 7: Right stick Y
- Byte 8: Left trigger / L2
- Byte 9: Right trigger / R2

### As A Struct

```cpp
struct SantrollerStageKitState
{
    uint8_t reportId = 0x01;

    bool a : 1; // cross
    bool b : 1; // circle
    bool x : 1; // square
    bool y : 1; // triangle

    bool l1 : 1; // left bumper
    bool r1 : 1; // right bumper
    bool l2 : 1; // left trigger pressed
    bool r2 : 1; // right trigger pressed

    bool select : 1;
    bool start : 1;
    bool l3 : 1; // left stick click
    bool r3 : 1; // right stick click

    bool guide : 1;
    bool capture : 1;
    uint8_t : 2;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    // Stick axes
    // Neutral state is 0x7F/0x80
    // X axis is left at 0x00, right at 0xFF
    // Y axis is top at 0x00, bottom at 0xFF
    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;

    // Trigger axes
    // Neutral state is 0x00, maxes out at 0xFF
    uint8_t leftTrigger;
    uint8_t rightTrigger;
} __attribute__((__packed__)); // 10 bytes
```
