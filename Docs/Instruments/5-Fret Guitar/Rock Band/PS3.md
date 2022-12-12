# PS3 Guitar Hero Guitars

## Device Info

- Vendor ID: `0x12BA` ("Licensed by Sony Computer Entertainment America")
- Product ID: `0x0200` ("Harmonix Guitar for PlayStation(R)3")

## Input Info

Start/select, PS button, and d-pad work as normal.

Upper frets:

| Fret   | Button | Bit |
| :--:   | :----: | :-: |
| Green  | ×      | 1   |
| Red    | ○      | 2   |
| Yellow | Δ      | 3   |
| Blue   | □      | 0   |
| Orange | L1     | 4   |

Lower frets:

| Fret   | Buttons | Bits  |
| :--:   | :------ | :---  |
| Green  | × + L2  | 1 + 6 |
| Red    | ○ + L2  | 2 + 6 |
| Yellow | Δ + L2  | 3 + 6 |
| Blue   | □ + L2  | 0 + 6 |
| Orange | L1 + L2 | 4 + 6 |

Or, as flags:

| Flag      | Button | Bit |
| :--:      | :----: | :-: |
| Green     | ×      | 1   |
| Red       | ○      | 2   |
| Yellow    | Δ      | 3   |
| Blue      | □      | 0   |
| Orange    | L1     | 4   |
| Solo flag | L2     | 6   |

Strumbar: D-pad up/down

Whammy: Right stick X

Tilt: R1 (bit 5)

Pickup switch: Right stick Y

- TODO: Define ranges for each of the notches 

### As A Struct

```c
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
    bool r2 : 1;

    bool select : 1;
    bool start : 1;
    bool l3 : 1;
    bool r3 : 1;

    bool ps : 1;
    uint8_t padding : 3;

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
