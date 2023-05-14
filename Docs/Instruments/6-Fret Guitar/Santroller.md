# Santroller Guitar Hero Live Guitars

## Device Info

- Vendor/product ID: `1209:2882`
- Revision: `0x080x`
  - The last nibble may vary, see [the main Santroller doc](../../Other/Santroller.md).
- Device name: `Santroller`

## Input Info

Most inputs are the same as the PS3/Wii U GHL guitar, the ones that differ are listed here.

Tilt: Left stick X (instead of accelerometer X)

- Nominally centered at `0x80`, `0xFF` when fully tilted up, `0x00` when fully tilted down.

### As A Struct

```cpp
struct SantrollerSixFretGuitarState
{
    uint8_t reportId;

    bool white1 : 1;
    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool : 1;
    bool : 1;

    bool heroPower : 1;
    bool pause : 1;
    bool ghtv : 1;
    bool : 1;

    bool dpad_center : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t tilt;
    uint8_t strumBar;
    uint8_t unused2;
    uint8_t whammy;

    uint8_t unused3[12];
    int16_t unused4[4];
}
```
