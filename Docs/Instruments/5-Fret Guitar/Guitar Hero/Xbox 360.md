# Xbox 360 Guitar Hero Guitars

## Controller Info

TODO: Check World Tour, GH5, and Warriors of Rock guitars

- XInput Type: Gamepad (1)
- XInput Subtype: Guitar Alternate (6)

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

- Resting state is negative, pressed state is positive.

Tilt: Right stick Y

Other accelerometer data: Left and right triggers

- Unsure if these are present on guitars other than the Xplorer and Les Paul

Touch/slider bar: Left stick X

- This one's complicated:

  ```
  None = 0

  G = 0x9495 (-0x6B6B)
  R = 0xCCCD (-0x3333)
  Y = 0x1A1A
  B = 0x4949
  O = 0x7F7F

  GR = 0xAFB0 (-0x5050)
  GY = 0x1919
  GB = 0x4747
  GO = 0x7B7B
  RY = 0xE5E6 (-0x1A1A)
  RB = 0x4848
  RO = 0x7D7D
  YB = 0x2F2F
  YO = 0x7E7E
  BO = 0x6666

  GRY = 0xE4E5 (-0x1B1B)
  GRB = 0x4646
  GRO = 0x7979
  GYB = 0x2D2D
  GYO = 0x7A7A
  GBO = 0x6262
  RYB = 0x2E2E
  RYO = 0x7C7C
  RBO = 0x6464
  YBO = 0x6565

  GRYB = 0x2C2C
  GRYO = 0x7878
  GRBO = 0x6060
  GYBO = 0x6161
  RYBO = 0x6363

  GRYBO = 0x5F5F
  ```

- TODO: There's gotta be a pattern here, more thorough analysis is required.\
  Some observations by others from a discussion where I brought this up:
  - The top byte of each value can be ignored.
  - It helps to order them by signed value rather than by frets:

    ```
    G     = -0x6B (0x95)
    GR    = -0x50 (0xB0)
    R     = -0x33 (0xCD)
    GRY   = -0x1B (0xE5)
    RY    = -0x1A (0xE6)
    GY    = 0x19
    Y     = 0x1A
    GRYB  = 0x2C
    GYB   = 0x2D
    RYB   = 0x2E
    YB    = 0x2F
    GRB   = 0x46
    GB    = 0x47
    RB    = 0x48
    B     = 0x49
    GRYBO = 0x5F
    GRBO  = 0x60
    GYBO  = 0x61
    GBO   = 0x62
    RYBO  = 0x63
    RBO   = 0x64
    YBO   = 0x65
    BO    = 0x66
    GRYO  = 0x78
    GRO   = 0x79
    GYO   = 0x7A
    GO    = 0x7B
    RYO   = 0x7C
    RO    = 0x7D
    YO    = 0x7E
    O     = 0x7F
    ```

  - The top nibble of each value seems to correspond with which fret(s) are the top one(s) being held, and the bottom nibble seems to start from a particular number for those top frets:

    | Top frets | First nibble | Starting number for second nibble |
    | :-------- | :----------- | :-------------------------------- |
    | G         | `0x9`        | `0x5`                             |
    | RG        | `0xB`        | `0x0`                             |
    | R         | `0xC`        | `0xD`                             |
    | YR        | `0xE`        | `0x6`                             |
    | Y         | `0x1`        | `0xA`                             |
    | BY        | `0x2`        | `0xF`                             |
    | B         | `0x4`        | `0x9`                             |
    | OB        | `0x6`        | `0x6`                             |
    | O         | `0x7`        | `0xF`                             |

  - For frets below the top fret combo, a certain amount is subtracted from the bottom nibble's starting value: 1 for G, 2 for R, and 4 for Y.

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
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    bool orange : 1;
    bool pedal : 1;
    bool guide : 1;
    bool reserved : 1;

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
