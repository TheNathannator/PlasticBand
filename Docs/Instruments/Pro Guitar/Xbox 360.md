# Xbox 360 Rock Band 3 Pro Guitar

TODO: This document was written without actual hardware to test with. Everything here needs to be verified, and missing information needs to be filled out. 

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: 25, not part of XInput standards

## Input Info

Face buttons work like a standard Xbox 360 controller.

Fret numbers:

- For simplicity and clarity, the triggers here are combined little-endian-wise into a single 16-bit integer (that is, left trigger is the low 8 bytes, and right trigger is the high 8 bytes).

| String | Input         | Bits                     |
| :----- | :----         | :---                     |
| Low E  | Triggers      | `0b_xxxx_xxxx_xxx1_1111` |
| A      | Triggers      | `0b_xxxx_xx11_111x_xxxx` |
| D      | Triggers      | `0b_x111_11xx_xxxx_xxxx` |
| G      | Left Thumb X  | `0b_xxxx_xxxx_xxx1_1111` |
| B      | Left Thumb X  | `0b_xxxx_xx11_111x_xxxx` |
| High E | Left Thumb X  | `0b_x111_11xx_xxxx_xxxx` |

String velocity:

| String | Input         | Bits                     |
| :----- | :----         | :---                     |
| Low E  | Left Thumb Y  | `0b_xxxx_xxxx_x111_1111` |
| A      | Left Thumb Y  | `0b_x111_1111_xxxx_xxxx` |
| D      | Right Thumb X | `0b_xxxx_xxxx_x111_1111` |
| G      | Right Thumb X | `0b_x111_1111_xxxx_xxxx` |
| B      | Right Thumb Y | `0b_xxxx_xxxx_x111_1111` |
| High E | Right Thumb Y | `0b_x111_1111_xxxx_xxxx` |

Standard 5-fret color flags:

| Fret Color | Input         | Bits                     |
| :--------- | :----         | :---                     |
| Green      | Left Thumb Y  | `0b_xxxx_xxxx_1xxx_xxxx` |
| Red        | Left Thumb Y  | `0b_1xxx_xxxx_xxxx_xxxx` |
| Yellow     | Right Thumb X | `0b_xxxx_xxxx_1xxx_xxxx` |
| Blue       | Right Thumb X | `0b_1xxx_xxxx_xxxx_xxxx` |
| Orange     | Right Thumb Y | `0b_xxxx_xxxx_1xxx_xxxx` |

Tilt is not reported through the standard XInput data, according to the referenced forum post. It's possible it does the same thing as the RB3 keyboard's touchpad.

TODO: There are probably more inputs present on the guitar than just these, such as a pedal port 

### As A Struct

```cpp
struct XInputProGuitarState
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
    bool y : 1;

    uint8_t lowEFret : 5;
    uint8_t aFret : 5;
    uint8_t dFret : 5;
    bool : 1;
    uint8_t gFret : 5;
    uint8_t bFret : 5;
    uint8_t highEFret : 5;
    bool soloFlag : 1;

    uint8_t lowEVelocity : 7;
    bool greenFret: 1;
    uint8_t aVelocity : 7;
    bool redFret: 1;
    uint8_t dVelocity : 7;
    bool yellowFret: 1;
    uint8_t gVelocity : 7;
    bool blueFret: 1;
    uint8_t bVelocity : 7;
    bool orangeFret: 1;
    uint8_t highEVelocity : 7;
    bool : 1;
}
```

## References

- https://dwsk.proboards.com/thread/404/song-standard-advancements
- https://github.com/maxton/gamepad-midi/blob/master/LibPlasticInstrument/Native/Xinput.cs#L110 for subtype
