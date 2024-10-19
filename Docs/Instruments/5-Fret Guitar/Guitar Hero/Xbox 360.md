# Xbox 360 Guitar Hero Guitars

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: Guitar Alternate (7)
- World Tour kiosk stick hardware IDs:
  - Vendor ID (left stick X): `0x1430`
  - Product ID (left stick Y): `0x4734`
  - Revision (right stick X): `0x3122`
- Guitar Hero 5 wireless/kiosk stick hardware IDs:
  - Vendor ID (left stick X): `0x1430`
  - Product ID (left stick Y): `0x0705`
  - Revision (right stick X): `0x0001`
- Warriors of Rock kiosk stick hardware IDs:
  - Vendor ID (left stick X): `0x1430`
  - Product ID (left stick Y): `0x0706`
  - Revision (right stick X): `0x0001`
  - Probably the same on wireless, but not confirmed yet.

## Input Info

This device uses the standard Xbox 360 report format, [documented here](../../../Controller%20Communication%20Basics/Xbox%20360.md). Refer to that document for the base layout.

Face buttons work like a standard Xbox 360 controller.

Frets:

| Fret   | Button      |
| :--:   | :----:      |
| Green  | A           |
| Red    | B           |
| Yellow | X           |
| Blue   | Y           |
| Orange | Left bumper |

Strumbar: D-pad up/down

Whammy: Right stick X

- Ranges from -32768 when not pressed to 32767 when fully pressed.

Accelerometers:

- These guitars feature a full 3-axis accelerometer which output values relative to gravity. The axis names used here are based on the equivalent axes on the PS3 guitars.
- X axis (used for standard tilt): Right stick Y
  - Positive when the guitar is pointed up, negative when the guitar is pointed down.
- Y axis: Left trigger (centered at `0x80`, not accounting for miscalibrations)
  - Above `0x80` when the guitar is face down, below when the guitar is face up.
- Z axis: Right trigger (centered at `0x80`, not accounting for miscalibrations)
  - Above `0x80` when the guitar is pointed right, below when the guitar is pointed left.
- Values and ranges may vary. On some guitars (notably the Les Paul) the accelerometer is miscalibrated and maxes out for a significant portion of the range, with no way to fix or calibrate for it.

Touch/slider bar: Left stick X

- See the [General Notes](General%20Notes.md) document for more info.

RJ14 (pedal) port: Right bumper

### As Data Types

```cpp
struct XInputGuitarAlternateGamepad
{
    bool dpadStrumUp : 1;
    bool dpadStrumDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool : 1;
    bool : 1;

    bool orange : 1;
    bool pedal : 1;
    bool guide : 1;
    bool : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    uint8_t accelZ;
    uint8_t accelX;
    int16le_t touchBar;
    int16le_t unused;
    int16le_t whammy;
    int16le_t tilt_accelY;
} __attribute__((packed));

enum XInputGuitarAlternateTouchBar
{
    TouchBar_None = 0,

    TouchBar_G = 0x9495, // -0x6B6B
    TouchBar_R = 0xCCCD, // -0x3333
    TouchBar_Y = 0x1A1A,
    TouchBar_B = 0x4949,
    TouchBar_O = 0x7F7F,

    TouchBar_GR = 0xAFB0, // -0x5050
    TouchBar_GY = 0x1919,
    TouchBar_GB = 0x4747,
    TouchBar_GO = 0x7B7B,
    TouchBar_RY = 0xE5E6, // -0x1A1A
    TouchBar_RB = 0x4848,
    TouchBar_RO = 0x7D7D,
    TouchBar_YB = 0x2F2F,
    TouchBar_YO = 0x7E7E,
    TouchBar_BO = 0x6666,

    TouchBar_GRY = 0xE4E5, // -0x1B1B
    TouchBar_GRB = 0x4646,
    TouchBar_GRO = 0x7979,
    TouchBar_GYB = 0x2D2D,
    TouchBar_GYO = 0x7A7A,
    TouchBar_GBO = 0x6262,
    TouchBar_RYB = 0x2E2E,
    TouchBar_RYO = 0x7C7C,
    TouchBar_RBO = 0x6464,
    TouchBar_YBO = 0x6565,

    TouchBar_GRYB = 0x2C2C,
    TouchBar_GRYO = 0x7878,
    TouchBar_GRBO = 0x6060,
    TouchBar_GYBO = 0x6161,
    TouchBar_RYBO = 0x6363,

    TouchBar_GRYBO = 0x5F5F
};
```

## References

- Observations from my own hardware
- https://github.com/spitfirex86/SliderBarThing for touchbar info
