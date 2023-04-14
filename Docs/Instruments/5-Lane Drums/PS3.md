# PS3 Guitar Hero Drums

## Device Info

- Vendor/product ID: `12BA:0120`
- Revision:
  - World Tour:
  - GH5/Band Hero/WoR:
- Device name:
- PS3 ID:

## Input Info

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
| Red    | R2 Pressure | 13          |
| Yellow | L2 Pressure | 12          |
| Blue   | R1 Pressure | 15          |
| Orange | ○ Pressure  | 17          |
| Green  | L1 Pressure | 14          |
| Kick   | Δ Pressure  | 16          |

### As A Struct

```cpp
struct PS3FiveLaneDrumsState
{
    uint8_t reportId;

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
}
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-gh-drums.html
- [Phase Shift mappings file](../../Other/device_list.json)
