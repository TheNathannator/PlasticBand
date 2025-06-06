# PS3 and Wii Rock Band Drums

## Device Info

PS3:

- Vendor/product ID:
  - Drumkit: `12BA:0210`
- Revision:
  - RB1: `0x1000`
  - RB2:
  - RB3:
- Device name:
  - The Phase Shift device list has 2 different names listed
  - `Harmonix Drum kit for PlayStation®3` (sic)
  - `Harmonix Drum Kit for PlayStation(R)3`
- PS3 flags:
  - RB1: `0x00`
  - RB2:
  - RB3:

Wii:

- Vendor/product ID:
  - RB1: `1BAD:0005`
  - RB2 and later: `1BAD:3110`
- Revision:
  - RB1:
  - RB2:
  - RB3:
- Device name:
  - `Harmonix Drum Controller for Nintendo Wii` 

MIDI Pro Adapter information may be found in [its own document](../../Other/Rock%20Band%20Adapters/MIDI%20Pro%20Adapters.md).

## Input Info

This device sends a report modelled after the one [documented here](../../Base%20Reports/PS3.md). Refer to that document for the base layout.

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

with the yellow and blue cymbals doing d-pad up and down, respectively. Refer to the [General Notes](General%20Notes.md#deciphering-pads-and-cymbals) doc for an example of how to distinguish things.

Pedals:

| Pedal  | Button |
| :---:  | :----: |
| Kick 1 | L1     |
| Kick 2 | R1     |

Velocities:

| Pad    | Axis        | Byte offset |
| :-:    | :--:        | :---------: |
| Red    | R2 Pressure | 12          |
| Yellow | L2 Pressure | 11          |
| Blue   | R1 Pressure | 14          |
| Green  | L1 Pressure | 13          |

If you hit both the pad and cymbal of the same colour simultaneously, the pads velocity will be in its respective velocity byte, but the cymbals velocity will be placed into the red velocity byte.
In this scenario you end up with a velocity in the red velocity byte but the red button bit won't be set.

### As A Struct

```cpp
struct PS3FourLaneDrumsState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool square_blue : 1;
    bool cross_green : 1;
    bool circle_red : 1;
    bool triangle_yellow : 1;

    bool kick1 : 1; // l1
    bool kick2 : 1; // r1
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool pad : 1; // l3
    bool cymbal : 1; // r3

    bool ps : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t unused_sticks[4];

    uint8_t unused_pressure1[4]; // pressure[0-3]
    uint8_t velocity_yellow; // pressure[4]
    uint8_t velocity_red; // pressure[5]
    uint8_t velocity_green; // pressure[6]
    uint8_t velocity_blue; // pressure[7]
    uint8_t unused_pressure2[4]; // pressure[8-11]

    uint16le_t unused_accelerometer[4];
} __attribute__((packed));
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-rockband-drums.html
- [Phase Shift mappings file](../../Other/device_list.json)
