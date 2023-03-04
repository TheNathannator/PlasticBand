# PS4 Rock Band Drums

## Device Info

TODO 

- Vendor ID: 
- Product ID: 

## Input Info

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
    uint8_t reportId;

    uint8_t unused1[4];

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4 
    uint8_t dpad : 4;
    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

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
    uint32_t crc32;
};
```

## References

- https://github.com/yanagiragi/Minatsuki
