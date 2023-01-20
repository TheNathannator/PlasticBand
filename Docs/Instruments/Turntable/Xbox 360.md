# Xbox 360 DJ Hero Turntable

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: 23, not part of XInput standards

## Input Info

Face buttons work like a standard Xbox 360 controller.

Tables:

- Left table:

  | Action  | Input                           |
  | :-----  | :---:                           |
  | Scratch | Left stick X                    |
  | Green   | A + Left trigger `0b_xxxx_xxx1` |
  | Red     | B + Left trigger `0b_xxxx_xx1x` |
  | Blue    | X + Left trigger `0b_xxxx_x1xx` |

- Right table:

  | Action  | Input                            |
  | :-----  | :---:                            |
  | Scratch | Left stick Y                     |
  | Green   | A + Right trigger `0b_xxxx_xxx1` |
  | Red     | B + Right trigger `0b_xxxx_xx1x` |
  | Blue    | X + Right trigger `0b_xxxx_x1xx` |

- Scratching:
  - Positive is clockwise, negative is counter-clockwise.
  - Only uses a tiny range (within -64 to +64), presumably so as to not register on the Xbox 360 menus.

Euphoria button: Y button

Effects knob: Right stick X

- Clockwise increments, counter-clockwise decrements.
- Wraps around when reaching maximum or minimum.
- Can be treated as an unsigned integer.

Crossfader: Right stick Y

- Range is inverted: left is positive, right is negative.

### As A Struct

```cpp
struct XInputTurntableState
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool : 1;
    bool : 1;

    bool : 1;
    bool : 1;
    bool guide : 1;
    bool : 1;

    bool a : 1;
    bool b : 1;
    bool x : 1;
    bool y_euphoria : 1;

    bool leftTableGreen : 1;
    bool leftTableRed : 1;
    bool leftTableBlue : 1;
    uint8_t : 5;

    bool rightTableGreen : 1;
    bool rightTableRed : 1;
    bool rightTableBlue : 1;
    uint8_t : 5;

    int16_t leftTableVelocity;
    int16_t rightTableVelocity;

    int16_t effectsKnob; // Whether or not this is signed doesn't really matter, as either way it's gonna loop over when it reaches min/max
    int16_t crossfader;
}
```

## Vibration Info

Euphoria button light: Right vibration

- First turns on at around 7936 (`0x1F00`). Maxes out at 65535 (`0xFFFF`).
- The low-level XUSB vibration report is only bytes instead of shorts, so on that end it turns on at `0x1F` and maxes out at `0xFF`.

## References

- Observations from my own hardware
