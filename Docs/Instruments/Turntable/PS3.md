# PS3 DJ Hero Turntable

## Device Info

- Vendor/product ID: `12BA:0140`
- Revision: `0x0005`
- Manufacturer string: `RedOctane DJ`
- Device name: `Guitar Hero5 for PlayStation (R) 3`
- PS3 flags: `0x06`

## Input Info

This device sends a report modelled after the one [documented here](../../Base%20Reports/PS3.md). Refer to that document for the base layout.

Face buttons and d-pad work as normal, with the exception that the d-pad is neutral at 15 (`0x0F`) instead of 8.

Tables:

- Left table:

  | Action  | Input                                        |
  | :-----  | :---:                                        |
  | Scratch | Right stick X                                |
  | Green   | × + Accelerometer Y `0b_xxxx_xxxx_xxx1_xxxx` |
  | Red     | ○ + Accelerometer Y `0b_xxxx_xxxx_xx1x_xxxx` |
  | Blue    | □ + Accelerometer Y `0b_xxxx_xxxx_x1xx_xxxx` |

- Right table:

  | Action  | Input                                        |
  | :-----  | :---:                                        |
  | Scratch | Right stick Y                                |
  | Green   | × + Accelerometer Y `0b_xxxx_xxxx_xxxx_xxx1` |
  | Red     | ○ + Accelerometer Y `0b_xxxx_xxxx_xxxx_xx1x` |
  | Blue    | □ + Accelerometer Y `0b_xxxx_xxxx_xxxx_x1xx` |

- The accelerometer Y axis is still neutral at `0x0200` in addition to the bitmask used by the platter buttons.

- Scratching:
  - Positive is clockwise, negative is counter-clockwise.
  - The full range is used here, unlike Xbox 360 turntables.

Euphoria button: Δ button

Effects knob: Accelerometer X

- Clockwise increments, counter-clockwise decrements.
- Wraps around when reaching maximum or minimum (remember that this is a 10-bit value, so it'll wrap around from `0x03FF` to 0).

Crossfader: Accelerometer Z

- Unlike the Xbox 360 turntable, the range is *not* inverted: left is minimum (`0x000`), right is maximum (`0x3FF`).

Pressure axes:

- Some inputs trigger the pressure axes along with their regular inputs. Details can be found in the struct below.
- These axes are just 0 or 255, so they're not particularly useful.
- Some of these axes are triggered by two buttons. With these inputs, pressing both of the buttons will set it to 0 instead of leaving it at 255.

### As A Struct

```cpp
struct PS3TurntableState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle_euphoria : 1;

    bool : 1;
    bool : 1;
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
    // 6    15   2
    //   5     3
    //      4
    uint8_t dpad;

    uint8_t unused_leftStick[2];
    uint8_t leftTableVelocity; // rightStickX
    uint8_t rightTableVelocity; // rightStickY

    uint8_t pressure_dpadRight_square; // pressure[0]
    uint8_t pressure_dpadLeft; // pressure[1]
    uint8_t pressure_dpadUp_cross; // pressure[2]
    uint8_t pressure_dpadDown; // pressure[3]
    uint8_t pressure_triangle; // pressure[4]
    uint8_t pressure_circle; // pressure[5]
    uint8_t unused_pressure[6]; // pressure[6-11]

    // Reminder that these values are 10-bit in range
    uint16le_t effectsKnob; // accelX
    uint16le_t crossfader; // accelZ
    uint16le_t platterButtons; // accelY
    uint16le_t unused_gyro;
} __attribute__((packed));
```

## Output Info

### Output Type `0x91`: Euphoria Light

This report controls the Euphoria button's light.

Unlike the Xbox 360 turntable, the light is not controlled using an absolute brightness value. Rather, it's a simple on-off value, and the LED fades in and out on its own.

Length: 8 bytes

```cpp
struct PS3TurntableEuphoria
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    uint8_t outputType = 0x91;
    uint8_t unknown1 = 0x01;
    uint8_t enable; // 1 to enable, 0 to disable
    uint8_t padding[5];
} __attribute__((packed));
```

## References

- https://github.com/RPCS3/rpcs3/blob/master/rpcs3/Emu/Io/Turntable.cpp
- https://github.com/ghlre/GHLtarUtility
- Observations by [sanjay900](https://github.com/sanjay900) and [shockdude](https://github.com/shockdude)
