# PS3 and Wii Rock Band Guitars

## Device Info

PS3:

- Vendor/product ID: `12BA:0200`
- PS3 ID: `0x00`

Wii:

- Vendor/product ID:
  - RB1: `1BAD:0004`
  - RB2 and later: `1BAD:3010`

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

Whammy: Right stick X

Tilt: R1

Pickup switch: Right stick Y

- TODO: Define ranges for each of the notches 

### As A Struct

```cpp
struct PS3RockBandGuitarState
{
    uint8_t reportId;

    bool blue : 1;
    bool green : 1;
    bool red : 1;
    bool yellow : 1;

    bool orange : 1;
    bool tilt : 1;
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

    uint8_t unused1[2];
    uint8_t whammy;
    uint8_t pickup;

    uint8_t unused2[12];
    int16_t unused3[4];
}
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-rockband-guitar.html
- https://sites.google.com/site/infnorm/rbguitartechnicaldetails
- https://docs.google.com/spreadsheets/d/1Y3QM1tEcf0bGiUTjT7R-3mwEAKrCL0qYoySmk3RLo8c/edit?usp=sharing
- [Phase Shift mappings file](../../Other/device_list.json)
