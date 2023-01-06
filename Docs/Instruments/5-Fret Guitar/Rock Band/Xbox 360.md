# Xbox 360 Rock Band Guitars

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: Guitar (6)

TODO: Rock Band is able to differentiate between RB guitars with auto-calibration and those without, most likely either a flag or reported vibration capabilities

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

- Ranges from -32768 (not pressed) to 32767 (fully pressed). Note that the actual max may vary.

Tilt: Right Stick Y

- Acts like a button press: not tilted = 0, tilted = 32767 (the values might not be exact, use a threshold when checking the value).
- Seems to flicker somewhat between pressed and not pressed when tilting.

Pickup switch: Left trigger

- TODO: Define ranges for each of the notches 

Pedal port (3.5mm port): Right Stick Y

- Acts exactly the same as tilt, minus the flickering.

Auto-calibration sensors: Left Stick X

- Only one sensor is ever active at a time, and they both use this axis to report their state.
- Microphone:
  - Ranges from `0x0000` (no noise) to `0x7FFF` (roughly, doesn't actually reach this but it's so close it might as well).
  - Has a limited frequency response range: it starts barely picking up at around 3640 Hz, starts maxing out at around 4350 hz, and starts dropping off at around 11400 Hz. Rock Band uses a tone at around B7 (3951 Hz).
- Light sensor:
  - Ranges from `0x7FFF` (no light) to `0x0000`.

### As A Struct

```c
struct XInputGuitarGamepad
{
    bool dpadStrumUp : 1;
    bool dpadStrumDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool soloFlag : 1;
    bool : 1;

    bool orange : 1;
    bool : 1;
    bool guide : 1;
    bool : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    uint8_t pickupSwitch;
    uint8_t unused1;
    int16_t calibrationSensor;
    int16_t unused2;
    int16_t whammy;
    int16_t tilt;
}
```

## Vibration Info

Toggling auto-calibration sensors:

- Microphone: Right vibration `0x6000`
- Light sensor: Right vibration `0xFFFF`
- Off: Right vibration `0x0000` (or any other non-specified value)
