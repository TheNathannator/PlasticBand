# Xbox 360 Rock Band 3 Pro Guitar

TODO: This document was written without actual hardware to test with. Everything here needs to be verified, and missing information needs to be filled out. 

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: 25, not part of XInput standards

## Input Info

Face buttons work like a standard Xbox 360 controller.

Binary representations key:

- `x` = This bit doesn't matter.
- `B` = This bit is used.

Fret numbers:

- For simplicity and clarity, the triggers here are combined little-endian-wise into a single 16-bit integer (that is, left trigger is the low 8 bytes, and right trigger is the high 8 bytes).

| String | Input         | Bits                     |
| :----- | :----         | :---                     |
| Low E  | Triggers      | `0b_xxxx_xxxx_xxxB_BBBB` |
| A      | Triggers      | `0b_xxxx_xxBB_BBBx_xxxx` |
| D      | Triggers      | `0b_xBBB_BBxx_xxxx_xxxx` |
| G      | Left Thumb X  | `0b_xxxx_xxxx_xxxB_BBBB` |
| B      | Left Thumb X  | `0b_xxxx_xxBB_BBBx_xxxx` |
| High E | Left Thumb X  | `0b_xBBB_BBxx_xxxx_xxxx` |

String velocity?:

| String | Input         | Bits                     |
| :----- | :----         | :---                     |
| Low E  | Left Thumb Y  | `0b_xxxx_xxxx_xBBB_BBBB` |
| A      | Left Thumb Y  | `0b_xBBB_BBBB_xxxx_xxxx` |
| D      | Right Thumb X | `0b_xxxx_xxxx_xBBB_BBBB` |
| G      | Right Thumb X | `0b_xBBB_BBBB_xxxx_xxxx` |
| B      | Right Thumb Y | `0b_xxxx_xxxx_xBBB_BBBB` |
| High E | Right Thumb Y | `0b_xBBB_BBBB_xxxx_xxxx` |

Standard 5-fret color flags:

| Fret Color | Input         | Bits                     |
| :--------- | :----         | :---                     |
| Green      | Left Thumb Y  | `0b_xxxx_xxxx_Bxxx_xxxx` |
| Red        | Left Thumb Y  | `0b_Bxxx_xxxx_xxxx_xxxx` |
| Yellow     | Right Thumb X | `0b_xxxx_xxxx_Bxxx_xxxx` |
| Blue       | Right Thumb X | `0b_Bxxx_xxxx_xxxx_xxxx` |
| Orange     | Right Thumb Y | `0b_xxxx_xxxx_Bxxx_xxxx` |

Tilt is not reported through the standard XInput data, according to the referenced forum post. It's possible it does the same thing as the RB3 keyboard's touchpad.

TODO: There are probably more inputs present on the guitar than just these, such as a pedal port 

### As A Struct

```c
struct XInputProGuitarState
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    bool leftShoulder : 1;
    bool rightShoulder : 1;
    bool guide : 1;
    bool reserved : 1;

    bool a : 1;
    bool b : 1;
    bool x : 1;
    bool y : 1;

    uint16_t frets[2];
    uint8_t velocities[6];
}

// Indexes into velocities[] where the flag for a specific 5-fret color can be found at the top-most bit
#define VelocityColorIndex_Green   0
#define VelocityColorIndex_Red     1
#define VelocityColorIndex_Yellow  2
#define VelocityColorIndex_Blue    3
#define VelocityColorIndex_Orange  4
```

## References

- https://dwsk.proboards.com/thread/404/song-standard-advancements
- https://github.com/maxton/gamepad-midi/blob/master/LibPlasticInstrument/Native/Xinput.cs#L110 for subtype
