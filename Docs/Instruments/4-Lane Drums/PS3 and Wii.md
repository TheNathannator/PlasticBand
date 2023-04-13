# PS3 and Wii Rock Band Drums

## Device Info

PS3:

- Vendor/product ID:
  - Drumkit: `12BA:0210`
  - MIDI Pro Adapter in drums mode: `12BA:0218`
- PS3 ID: 

Wii:

- Vendor/product ID:
  - RB1: `1BAD:0005`
  - RB2 and later: `1BAD:3110`
  - MIDI Pro Adapter in drums mode: `1BAD:3118`

## Input Info

Face buttons and d-pad function as normal.

Pads/cymbals:

| Pad/cymbal    | Buttons             |
| :--------:    | :------             |
| Red pad       | ○ + L3              |
| Yellow pad    | Δ + L3              |
| Blue pad      | □ + L3              |
| Green pad     | × + L3              |
| Yellow cymbal | Δ + R3 + d-pad up   |
| Blue cymbal   | □ + R3 + d-pad down |
| Green cymbal  | × + R3              |

Or, as flags:

| Flag   | Button |
| :--:   | :----: |
| Red    | ○      |
| Yellow | Δ      |
| Blue   | □      |
| Green  | ×      |
| Pad    | L3     |
| Cymbal | R3     |

with the yellow and blue cymbals doing d-pad up and down, respectively.

Pedals:

| Pedal  | Button |
| :---:  | :----: |
| Kick 1 | L1     |
| Kick 2 | R1     |

Velocities:

| Pad    | Axis        | Byte offset |
| :-:    | :--:        | :---------: |
| Red    | R2 Pressure | 13          |
| Yellow | L2 Pressure | 12          |
| Blue   | R1 Pressure | 15          |
| Green  | L1 Pressure | 14          |

### As A Struct

```cpp
struct PS3FourLaneDrumsState
{
    uint8_t reportId;

    bool square_blue : 1;
    bool cross_green : 1;
    bool circle_red : 1;
    bool triangle_yellow : 1;

    bool kick1 : 1;
    bool kick2 : 1;
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool pad : 1;
    bool cymbal : 1;

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

    uint8_t unused2[4];
    int16_t unused3[4];
}
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-rockband-drums.html
- [Phase Shift mappings file](../../Other/device_list.json)
