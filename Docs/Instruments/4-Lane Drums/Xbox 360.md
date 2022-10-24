# Xbox 360 Rock Band Drums

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: Drumkit (8)

## Input Info

Face buttons work like a standard Xbox 360 controller.

Pads and cymbals:

| Action        | Input                         |
| :-----        | :----                         |
| Red pad       | B + Right stick click         |
| Yellow pad    | Y + Right stick click         |
| Blue pad      | X + Right stick click         |
| Green pad     | A + Right stick click         |
| Yellow cymbal | Y + Right bumper + D-pad up   |
| Blue cymbal   | X + Right bumper + D-pad down |
| Green cymbal  | A + Right bumper              |

Or, as flags:

| Flag        | Input             |
| :-----      | :----             |
| Red         | B                 |
| Yellow      | Y                 |
| Blue        | X                 |
| Green       | A                 |
| Pad flag    | Right stick click |
| Cymbal flag | Right bumper      |

with the yellow and blue cymbals doing d-pad up and down, respectively.

Pedals:

| Action               | Input            |
| :-----               | :----            |
| Kick 1 (orange port) | Left bumper      |
| Kick 2 (black port)  | Left stick click |

Velocities:

- `1` = This bit is always 1 when the other bits are active.
- `0` = This bit is always 0.
- `B` = This bit is used but not constant.

  | Action | Input         | Bits                     |
  | :----- | :----         | :--:                     |
  | Red    | Left stick X  | `0b_0BBB_BBBB_BBBB_BBBB` |
  | Yellow | Left stick Y  | `0b_1BBB_BBBB_BBBB_BBBB` |
  | Blue   | Right stick X | `0b_0BBB_BBBB_BBBB_BBBB` |
  | Green  | Right stick Y | `0b_1BBB_BBBB_BBBB_BBBB` |

- The range is inverted: excluding the top-most bit, `0x0000` for the hardest hit, and `0x7FFF` for the softest measurable hit. This is done for compatibility with Rock Band 1 kits, which don't report velocity.
- The top-most bit probably doesn't matter, but it is observed as being different between pads.

### As A Struct

```c
struct XInput4LaneDrumsGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool pedal2 : 1;
    bool padFlag : 1;

    bool pedal1 : 1;
    bool cymbalFlag : 1;
    bool guide : 1;
    bool unused : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    uint16_t unused;
    int16_t redVelocity;
    int16_t yellowVelocity;
    int16_t blueVelocity;
    int16_t greenVelocity;
}
```
