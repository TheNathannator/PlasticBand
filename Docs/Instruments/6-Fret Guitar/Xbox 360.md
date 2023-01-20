# Xbox 360 Guitar Hero Live Guitars

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: Guitar Alternate (7)
  - Annoyingly, the subtype is not unique, so additional capabilities info is required.
- XInput Flags: Voice supported, plug-in modules supported, no navigation (`0x1C`)
  - Interestingly, the no navigation flag is set, but clearly this shouldn't be the case, it has equivalents for d-pad, ABXY, start, back, and the guide button. This flag is probably how Guitar Hero Live distinguishes between 5-fret and 6-fret guitars.

## Input Info

Frets:

| Action  | Input        |
| :-----  | :---:        |
| Black 1 | A            |
| Black 2 | B            |
| Black 3 | Y            |
| White 1 | X            |
| White 2 | Left bumper  |
| White 3 | Right bumper |

Strumbar: D-pad up/down + left stick up/down

- It seems that Guitar Hero Live only uses the left stick input when checking for strumming during gameplay.

Tilt and whammy are swapped compared to the other Xbox 360 guitars.

Whammy: Right stick Y

- Ranges from 0 (not pressed) to 32767 (pressed). This also differs from other Xbox 360 guitars.

Tilt: Right stick X

- Positive is tilted up, negative is tilted down.

D-pad up/down/left/right are as expected.

D-pad center button: Guide button

Pause buttons: Start button

Hero Power button: Back button

GHTV button: Left stick click

### As A Struct

```cpp
struct XInputSixFretGuitarGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool pause : 1;
    bool heroPower : 1;
    bool ghtv : 1;
    bool : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool guide : 1;
    bool : 1;

    bool black1 : 1;
    bool black2 : 1;
    bool white1 : 1;
    bool black3 : 1;

    uint8_t unused1[2];
    int16_t unused2;

    int16_t strumBar;
    int16_t tilt;
    int16_t whammy;
}
```

## References

- Observations sent by [Ahriana](https://github.com/Ahriana) via DMs
