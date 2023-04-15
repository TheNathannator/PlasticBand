# Santroller HID Rock Band Guitars

The only real difference between this and a PS3 Rock Band Guitar, is that we also map tilt to the left stick x, as that way games can take advantage of an analog tilt.

## Device Info

- Vendor/product ID:
  - `1209:2882`
- Revision:
  - `0x071x`
  - The last nibble may vary, see [the main Santroller doc](../../Other/Santroller.md).
- Device name:
  - Santroller

## Input Info

Start/select, PS button, and d-pad work as normal.

Upper frets:

| Fret   | Button |
| :--:   | :----: |
| Green  | ×      |
| Red    | ○      |
| Yellow | Δ      |
| Blue   | □      |
| Orange | L1     |

Lower frets:

| Fret   | Buttons |
| :--:   | :------ |
| Green  | × + L2  |
| Red    | ○ + L2  |
| Yellow | Δ + L2  |
| Blue   | □ + L2  |
| Orange | L1 + L2 |

Or, as flags:

| Flag      | Button |
| :--:      | :----: |
| Green     | ×      |
| Red       | ○      |
| Yellow    | Δ      |
| Blue      | □      |
| Orange    | L1     |
| Solo flag | L2     |

Strumbar: D-pad up/down

Tilt: Left Stick X

- This is centered at 128, and is above 128 if the guitar is tilted up and below 128 if it is tilted down.

Whammy: Right stick X

Tilt: R1

Pickup switch: Right stick Y

- TODO: Define ranges for each of the notches 

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
    bool tiltButton : 1;
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
