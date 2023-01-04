# PS3 Guitar Hero Guitars

## Device Info

- Vendor ID: `0x12BA` ("Licensed by Sony Computer Entertainment America")
- Product ID: `0x0210` ("Harmonix Drum Kit for PlayStation(R)3")
  - MIDI Pro Adapter in drums mode: `0x0218`

## Input Info

Face buttons and d-pad function as normal.

Pads/cymbals:

| Pad/cymbal    | Buttons             | Bits                         |
| :--------:    | :------             | :---                         |
| Red pad       | ○ + L3              | 2 + 10                       |
| Yellow pad    | Δ + L3              | 3 + 10                       |
| Blue pad      | □ + L3              | 0 + 10                       |
| Green pad     | × + L3              | 1 + 10                       |
| Yellow cymbal | Δ + R3 + d-pad up   | 3 + 11 + (byte offset 3 = 0) |
| Blue cymbal   | □ + R3 + d-pad down | 0 + 11 + (byte offset 3 = 4) |
| Green cymbal  | × + R3              | 1 + 11                       |

Or, as flags:

| Flag   | Button | Bit |
| :--:   | :----: | :-: |
| Red    | ○      | 2   |
| Yellow | Δ      | 3   |
| Blue   | □      | 0   |
| Green  | ×      | 1   |
| Pad    | L3     | 10  |
| Cymbal | R3     | 11  |

with the yellow and blue cymbals doing d-pad up and down, respectively.

Pedals:

| Pedal  | Button | Bit |
| :---:  | :----: | :-: |
| Kick 1 | L1     | 4   |
| Kick 2 | R1     | 5   |

Velocities:

| Pad    | Axis        | Byte offset |
| :-:    | :--:        | :---------: |
| Red    | R2 Pressure | 13          |
| Yellow | L2 Pressure | 12          |
| Blue   | R1 Pressure | 15          |
| Green  | L1 Pressure | 14          |

### As A Struct

```c
struct PS3FourLaneDrumsState
{
    uint8_t reportId;

    bool square_blue : 1;
    bool cross_green : 1;
    bool circle_red : 1;
    bool triangle_yellow : 1;

    bool kick1 : 1;
    bool kick2 : 1;
    bool l2 : 1;
    bool r2 : 1;

    bool select : 1;
    bool start : 1;
    bool pad : 1;
    bool cymbal : 1;

    bool ps : 1;
    uint8_t padding : 3;

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
