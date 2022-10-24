# Xbox 360 Guitar Hero Drums

TODO: This document was written without actual hardware to test with. Everything here needs to be verified, and missing information needs to be filled out. 

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: Drumkit (8)

Guitar Hero kits differentiate themselves from Rock Band kits by holding the left stick click input. This conflicts with the second kick pedal input on Rock Band kits, but should be easy to detect when first finding a drumkit. The only issue will be if a Rock Band kit user happens to be holding the second pedal as the kit gets detected, which isn't likely but is entirely possible.

## Input Info

Face buttons work like a standard Xbox 360 controller.

Pads, cymbals, and kick:

| Action | Input        |
| :----- | :---:        |
| Red    | B            |
| Yellow | Y            |
| Blue   | X            |
| Orange | Right bumper |
| Green  | A            |
| Kick   | Left bumper  |

Velocities:

**TODO:** These are speculated, need to be verified with actual hardware 

- Key:
  - `B` = This bit is used.
  - `x` = This bit doesn't matter.

| Action | Input         | Bits                     |
| :----- | :----         | :--:                     |
| Red    | Left stick Y  | `0b_BBBB_BBBB_xxxx_xxxx` |
| Yellow | Right stick X | `0b_xxxx_xxxx_BBBB_BBBB` |
| Blue   | Right stick X | `0b_BBBB_BBBB_xxxx_xxxx` |
| Orange | Right stick Y | `0b_xxxx_xxxx_BBBB_BBBB` |
| Green  | Left stick Y  | `0b_xxxx_xxxx_BBBB_BBBB` |
| Kick   | Right stick Y | `0b_BBBB_BBBB_xxxx_xxxx` |

Hi-hat pedal port: TODO 

### As A Struct

```c
struct XInput5LaneDrumsGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool isFiveLane : 1;
    bool rightThumbClick : 1;

    bool kick : 1;
    bool orange : 1;
    bool guide : 1;
    bool reserved : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    int32_t unused; // TODO: The hi-hat pedal data is probably here somewhere
    uint8_t greenVelocity;
    uint8_t redVelocity;
    uint8_t yellowVelocity;
    uint8_t blueVelocity;
    uint8_t orangeVelocity;
    uint8_t kickVelocity;
}
```
