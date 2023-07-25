# PS4 Rock Band Guitars

## Device Info

- Vendor/product ID:
  - Stratocaster (MadCatz): `0738:8261`
  - Jaguar (PDP): `0E6F:0173`
- Revision:
  - Stratocaster:
  - Jaguar:
- Device name:
  - Stratocaster:
  - Jaguar:

## Input Info

Options, Share, PS button, and d-pad work as normal.

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
| Green  | × + L3  |
| Red    | ○ + L3  |
| Yellow | Δ + L3  |
| Blue   | □ + L3  |
| Orange | L1 + L3 |

Or, as flags:

| Flag      | Button |
| :--:      | :----: |
| Green     | ×      |
| Red       | ○      |
| Yellow    | Δ      |
| Blue      | □      |
| Orange    | L1     |
| Solo flag | L3     |

Strumbar: D-pad up/down

Whammy: L2 Axis

- Assuming it works the same as the PS3/Wii guitars, ranges from `0x00` when not pressed to `0xFF` when fully pressed, and resets to `0x7F` after a second or two of no movement.

Tilt: R2 Axis

Pickup switch: (Unknown)

### As A Struct

```cpp
struct PS4RockBandGuitarState
{
    uint8_t reportId;

    uint8_t unused1[4];

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum : 4;
    bool blue : 1;
    bool green : 1;
    bool red : 1;
    bool yellow : 1;

    bool orange : 1
    uint8_t : 3;
    bool share : 1;
    bool options : 1;
    bool solo : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 7;

    uint8_t whammy;
    uint8_t tilt;

    uint8_t unused2[20];

    uint8_t powerLevel : 4;
    uint8_t : 4;

    uint8_t unused4[43];
    uint32_t crc32;
} __attribute__((__packed__));
```

## References

- https://rb4.app/js/app.js
