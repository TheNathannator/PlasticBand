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
- PS3 flags: `0x07`

## Input Info

This device sends a report modelled after the one [documented here](../../Base%20Reports/PS3.md). Refer to that document for the base layout.

Face buttons and d-pad function as normal.

Pads:

| Pad            | Button |
| :-:            | :----: |
| Red            | ○      |
| Yellow         | Δ      |
| Blue           | □      |
| Orange         | R1     |
| Green          | ×      |
| Kick + Hi-Hat  | L1     |

Velocities:

The velocity values are standard MIDI, so they range from 0 to 0x7F.

| Pad           | Axis        | Byte offset |
| :-:           | :--:        | :---------: |
| Red           | R2 Pressure | 12          |
| Yellow        | L2 Pressure | 11          |
| Blue          | R1 Pressure | 14          |
| Orange        | ○ Pressure  | 16          |
| Green         | L1 Pressure | 13          |
| Kick + Hi-Hat | Δ Pressure  | 15          |

MIDI data:

The drums send any unrecognised MIDI data as-is, using the following bytes.
The Hi-Hat pedal sends MIDI data here alongside its Kick input, which is how you can differenciate it from a standard kick input.

| Byte          | Axis          | Byte offset |
| :-----------: | :-----------: | :---------: |
| 0             | Right Stick X | 5           |
| 1             | Accel X       | 19          |
| 2             | Accel Y       | 23          |


An example packet for the Hi-Hat pedal would look like the following:
`99 64 1C`: Note on (channel 10, note 100, velocity 28)

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

    bool kick : 1; // l1
    bool orange : 1; // r1
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

    uint8_t unused_leftStick[2];
    uint8_t midi_byte0; // rightStickX
    uint8_t unused_rightStickY;

    uint8_t unused_pressure1[4]; // pressure[0-3]
    uint8_t velocity_yellow; // pressure[4]
    uint8_t velocity_red; // pressure[5]
    uint8_t velocity_green; // pressure[6]
    uint8_t velocity_blue; // pressure[7]
    uint8_t velocity_pedal; // pressure[8]
    uint8_t velocity_orange; // pressure[9]
    uint8_t unused_pressure2[2]; // pressure[10-11]

    uint8_t midi_byte1; // accelerometer X first byte
    uint8_t unused_accelX;
    uint16le_t unused_accelZ;
    uint8_t midi_byte2; // accelerometer Y first byte
    uint8_t unused_accelY;
    uint16le_t unused_gyro;
} __attribute__((packed));
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-gh-drums.html
- [Phase Shift mappings file](../../Other/device_list.json)
