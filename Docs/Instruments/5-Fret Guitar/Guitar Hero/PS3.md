# PS3 Guitar Hero Guitars

## Device Info

- Vendor/product ID:
  - PS3 guitars: `12BA:0100`
  - World Tour PC guitar: `1430:474C`
- Revision:
  - Les Paul:
  - World Tour (PS3): `0x0108`
  - World Tour (PC):
  - GH5:
  - WoR:
- Device name:
  - Les Paul: `Guitar Hero3 for PlayStation (R) 3`
  - World Tour (PS3): `Guitar Hero4 for PlayStation (R) 3`
  - World Tour (PC): `Guitar Hero for PC/MAC`
  - GH5: `Guitar Hero5 for PlayStation (R) 3`
  - WoR:
- PS3 ID: `0x06`

## Input Info

This device sends a report modelled after the one [documented here](../../../Controller%20Communication%20Basics/PS3%20Instruments.md). Refer to that document for the base layout.

Start/select, PS button, and d-pad work as normal, with the exception that the d-pad is neutral at 31 (`0x1F`) instead of 8.

Frets:

| Fret   | Button |
| :--:   | :----: |
| Green  | ×      |
| Red    | ○      |
| Yellow | □      |
| Blue   | Δ      |
| Orange | L1     |

Strumbar: D-pad up/down

- Note again that the PoV hat byte is neutral at 31 (`0x1F`) instead of 8 on GH guitars.

Whammy: Right stick X

- Ranges from `0x7F`/`0x80` when not pressed to `0xFF` when fully pressed.

Accelerometers:

- These guitars feature a full 3-axis accelerometer which outputs values relative to gravity. The axis names used here match the fields that they're placed on compared to the normal PS3 controller report.
- X axis (used for standard tilt): Accelerometer X
  - Above `0x200` when the guitar is pointed up, below when the guitar is pointed down.
- Y axis: Accelerometer Y
  - Above `0x200` when the guitar is face down, below when the guitar is face up.
- Z axis: Accelerometer Z
  - Above `0x200` when the guitar is pointed right, below when the guitar is pointed left.
- Values and ranges may vary. All values listed are nominal, for proper handling calibration will need to be provided.

Some World Tour-specific(?) accelerometer quirks:

- X and Z are limited to a range of `0x180`-`0x280`.
- When reaching its minimum, X will snap to the maximum. After a certain point beyond that, it will somewhat jitter between max and min.

Touch/slider bar: Right stick Y

- See the [General Notes](General%20Notes.md) document for more info.

RJ14 (pedal) port: R1

Pressure axes:

- Some inputs trigger the pressure axes along with their regular inputs. Details can be found in the struct below.
- These axes are just 0 or 255, so they might as well not exist when you can just use the button fields.
- Some of these axes are triggered by two buttons. With these inputs, pressing both of the buttons will set it to 0 instead of leaving it at 255.

### As A Struct

```cpp
struct PS3GuitarHeroGuitarState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool yellow : 1;
    bool green : 1;
    bool red : 1;
    bool blue : 1;

    bool orange : 1;
    bool spPedal : 1;
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 3;

    //      0
    //   7     1
    // 6    31   2
    //   5     3
    //      4
    uint8_t dpad_strum;

    uint8_t unused1[2];
    uint8_t whammy;
    uint8_t slider;

    uint8_t pressure_dpadRight_yellow;
    uint8_t pressure_dpadLeft;
    uint8_t pressure_dpadUp_green;
    uint8_t pressure_dpadDown_orange;
    uint8_t pressure_blue;
    uint8_t pressure_red;
    uint8_t unused2[6];

    // Reminder that these values are 10-bit in range
    uint16le_t tilt_accelX;
    uint16le_t accelZ;
    uint16le_t accelY;
    uint16le_t unused3;
} __attribute__((packed));
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-gh-guitar.html
- [Phase Shift mappings file](../../Other/device_list.json)
