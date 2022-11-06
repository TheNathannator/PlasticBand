# Xbox 360 Rock Band Guitars

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: Guitar (6)

## Input Info

Face buttons work like a standard Xbox 360 controller.

Upper frets:

| Action | Input       |
| :----- | :---:       |
| Green  | A           |
| Red    | B           |
| Yellow | X           |
| Blue   | Y           |
| Orange | Left bumper |

Lower (solo) frets:

| Action | Input                 |
| :----- | :---:                 |
| Green  | A + Left stick click  |
| Red    | B + Left stick click  |
| Yellow | X + Left stick click  |
| Blue   | Y + Left stick click  |
| Orange | LB + Left stick click |

Or, as flags:

| Flag      | Input            |
| :---      | :---:            |
| Green     | A                |
| Red       | B                |
| Yellow    | X                |
| Blue      | Y                |
| Orange    | Left bumper      |
| Solo flag | Left stick click |

Strumbar: D-pad up/down

Whammy: Right Stick X

- Resting state is negative, pressed state is positive.

Tilt: Right Stick Y

- Acts like a button press: not tilted = 0, tilted = 32767 (the values might not be exact, use a threshold when checking the value).
- Seems to flicker somewhat between pressed and not pressed when tilting.

Pickup switch: Left trigger

- TODO: Define ranges for each of the notches 

Pedal port (3.5mm port): Right Stick Y

- Acts exactly the same as tilt, minus the flickering.

Auto-calibration sensors (RB2/3 only(?)): TODO, may require low-level XUSB stuff 

### As A Struct

```c
struct XInputRBGuitarGamepad
{
    bool dpadStrumUp : 1;
    bool dpadStrumDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool soloFlag : 1;
    bool rightThumbClick : 1;

    bool orange : 1;
    bool rightShoulder : 1;
    bool guide : 1;
    bool reserved : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    uint8_t pickupSwitch;
    uint8_t unused1;
    int32_t unused2;
    int16_t whammy;
    int16_t tilt;
}
```
