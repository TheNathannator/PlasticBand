# Santroller HID Rock Band Guitars

## Device Info

- Vendor/product ID: `1209:2882`
- Revision: `0x071x`
  - The last nibble may vary, see [the main Santroller doc](../../Other/Santroller.md).
- Device name: `Santroller`

## Input Info

Most inputs are the same as the PS3 RB guitar, the ones that differ are listed here.

Tilt: Left stick X (instead of R1)

- Nominally centered at `0x80`, `0xFF` when fully tilted up, `0x00` when fully tilted down.

### As A Struct

```cpp
struct SantrollerRockBandGuitarState
{
    uint8_t reportId;

    bool blue : 1;
    bool green : 1;
    bool red : 1;
    bool yellow : 1;

    bool orange : 1;
    bool : 1;
    bool solo : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum;

    uint8_t tilt;
    uint8_t unused1;
    uint8_t whammy;
    uint8_t pickup;

    uint8_t unused2[12];
    int16_t unused3[4];
}
```
