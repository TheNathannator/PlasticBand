# Santroller HID Stage Kit

While there was never a PS3 or Wii version of the stage kit, a Santroller version is available anyways since it doesn't make sense to limit it to only XInput mode.

## Device Info

- Vendor/product ID:
  - USB: `1209:2882`
  - Bluetooth:  `1209:2885`
- Revision: `0x0900`
- Device name: `Santroller`

## Input Info

The stage kit has no special inputs. It has face buttons, a d-pad, start, select, and PS button.

### As A Struct

```cpp
struct SantrollerStageKitState
{
    uint8_t reportId;

    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

    uint8_t : 4;

    bool select : 1;
    bool start : 1;
    uint8_t : 2;

    bool ps : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t unused2[16];
}
```
