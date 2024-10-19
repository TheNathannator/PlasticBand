# PS3 Guitar Hero Drums

## Device Info

- Vendor/product ID: `12BA:0120`
- Revision:
  - World Tour: `0x0108`
  - GH5/Band Hero/WoR:
- Device name:
  - The Phase Shift device list has like 3 different names for these lol
  - `GuitarHero for Playstation (R) 3`
  - World Tour: `Guitar Hero4 for PlayStation (R) 3`
  - GH5/Band Hero/WoR: `Guitar Hero5 for PlayStation (R) 3`
- PS3 ID: `0x07`

## Input Info

This device sends a report modelled after the one [documented here](../../Controller%20Communication%20Basics/PS3%20Instruments.md). Refer to that document for the base layout.

Face buttons and d-pad function as normal.

Pads:

| Pad    | Button |
| :-:    | :----: |
| Red    | ○      |
| Yellow | Δ      |
| Blue   | □      |
| Orange | R1     |
| Green  | ×      |
| Kick   | L1     |

Velocities:

| Pad    | Axis        | Byte offset |
| :-:    | :--:        | :---------: |
| Red    | R2 Pressure | 12          |
| Yellow | L2 Pressure | 11          |
| Blue   | R1 Pressure | 14          |
| Orange | ○ Pressure  | 16          |
| Green  | L1 Pressure | 13          |
| Kick   | Δ Pressure  | 15          |

### As A Struct

```cpp
struct PS3FiveLaneDrumsState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool square_blue : 1;
    bool cross_green : 1;
    bool circle_red : 1;
    bool triangle_yellow : 1;

    bool kick : 1;
    bool orange : 1;
    bool : 1;
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
    uint8_t dpad;

    uint8_t unused1[8];

    uint8_t velocity_yellow;
    uint8_t velocity_red;
    uint8_t velocity_green;
    uint8_t velocity_blue;
    uint8_t velocity_pedal;
    uint8_t velocity_orange;

    uint8_t unused2[2];
    int16_t unused3[4];
} __attribute__((__packed__));
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-gh-drums.html
- [Phase Shift mappings file](../../Other/device_list.json)
