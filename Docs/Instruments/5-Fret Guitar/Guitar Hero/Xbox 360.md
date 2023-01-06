# Xbox 360 Guitar Hero Guitars

## Controller Info

TODO: Check World Tour, GH5, and Warriors of Rock guitars

- XInput Type: Gamepad (1)
- XInput Subtype: Guitar Alternate (7)

## Input Info

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

- Ranges from -32768 (not pressed) to 32767 (fully pressed).

Tilt: Right stick Y

- Tilted up is positive, tilted down is negative.
- Values vary; on some guitars the range might max out at a fairly high angle, on others you might hardly be able to hold it regularly without it maxing out.

Other accelerometer data: Left and right triggers

- Unsure if these are present on guitars other than the Xplorer and Les Paul

Touch/slider bar: Left stick X

- Only the bottom byte of the value matters. The top byte is the same as the bottom, except for values with the bottom byte above `0x80`, where it is one less. (When viewing those values in signed hexadecimal, they still appear the same.)
- See the [General Notes](General%20Notes.md) document for more info.

RJ14 (pedal) port: Right bumper

### As Data Types

```c
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

    uint8_t accel1;
    uint8_t accel2;
    int16_t touchBar;
    int16_t unused;
    int16_t whammy;
    int16_t tilt;
}

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
}
```

## References

- https://github.com/spitfirex86/SliderBarThing for touchbar info
