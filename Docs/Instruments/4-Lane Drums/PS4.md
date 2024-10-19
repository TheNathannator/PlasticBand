# PS4 Rock Band Drums

## Device Info

- Vendor/product ID:
  - MadCatz: `0738:8262`
  - PDP: `0E6F:????` (product ID not known yet)
- Revision:
  - MadCatz:
  - PDP:
- Device name:
  - MadCatz:
  - PDP:

## Input Info

This device sends a report modelled after the one [documented here](../../Controller%20Communication%20Basics/PS4%20Instruments.md). Refer to that document for the base layout.

Report ID: `0x01`

Face buttons and d-pad function as normal.

Pads/cymbals:

| Pad/cymbal    | Button + velocity axis |
| :--------:    | :------                |
| Red pad       | ○ + byte offset 43     |
| Yellow pad    | Δ + byte offset 45     |
| Blue pad      | □ + byte offset 44     |
| Green pad     | × + byte offset 46     |
| Yellow cymbal | Δ + byte offset 47     |
| Blue cymbal   | □ + byte offset 48     |
| Green cymbal  | × + byte offset 49     |

Since the pads and cymbals of the same color use the same button inputs, only the velocity axes can be used to truly determine what was hit.

Pedals:

| Pedal  | Button |
| :---:  | :----: |
| Kick 1 | L1     |
| Kick 2 | R1     |

Any additional details are listed in the state struct.

### As A Struct

```cpp
struct PS4FourLaneDrumsState
{
    uint8_t reportId = 0x01;

    uint8_t unused1[4];

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad : 4;
    bool square_blue : 1;
    bool cross_green : 1;
    bool circle_red : 1;
    bool triangle_yellow : 1;

    bool kick1 : 1;
    bool kick2 : 1;
    uint8_t : 2;
    bool share : 1;
    bool options : 1;
    uint8_t : 2;

    bool ps : 1;
    uint8_t : 7;

    uint8_t unused2[22];

    uint8_t powerLevel : 4; // seems to range differently, packets listed in Minatsuki have 0x0C
    uint8_t : 4;

    uint8_t unused3[12];

    uint8_t redDrumVelocity;
    uint8_t blueDrumVelocity;
    uint8_t yellowDrumVelocity;
    uint8_t greenDrumVelocity;

    uint8_t yellowCymbalVelocity;
    uint8_t blueCymbalVelocity;
    uint8_t greenCymbalVelocity;

    uint8_t unused4[24];
    uint32le_t crc32;
} __attribute__((__packed__));
```

## References

- https://github.com/yanagiragi/Minatsuki
