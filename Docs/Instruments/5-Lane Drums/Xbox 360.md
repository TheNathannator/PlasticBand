# Xbox 360 Guitar Hero Drums

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: Drumkit (8)
- Band Hero Drum stick hardware IDs:
  - Vendor ID (left stick X): `0x1430`
  - Product ID (left stick Y): `0x0805`
  - Revision (right stick X): `0x0001`

Note that Guitar Hero and Rock Band drums share the same subtype, and can't be distinguished off of that alone. The Force Feedback capability flag is used as the distinguisher instead: Rock Band drums set the Force Feedback flag in their capabilities, while Guitar Hero drum kits do not.

## Input Info

This device uses the standard Xbox 360 report format, [documented here](../../Base%20Reports/Xbox%20360.md). Refer to that document for the base layout.

Face buttons work like a standard Xbox 360 controller.

Pads, cymbals, and kick:

| Action         | Input        |
| :-----         | :---:        |
| Red            | B            |
| Yellow         | Y            |
| Blue           | X            |
| Orange         | Right bumper |
| Green          | A            |
| Kick + Hi-Hat  | Left bumper  |

Velocities:

The velocity values are standard MIDI, so they range from 0 to 0x7F.

| Action        | Input         | Bits                     |
| :-----        | :----         | :--:                     |
| Red           | Left stick Y  | `0b_1111_1111_xxxx_xxxx` |
| Yellow        | Right stick X | `0b_xxxx_xxxx_1111_1111` |
| Blue          | Right stick X | `0b_1111_1111_xxxx_xxxx` |
| Orange        | Right stick Y | `0b_xxxx_xxxx_1111_1111` |
| Green         | Left stick Y  | `0b_xxxx_xxxx_1111_1111` |
| Kick + Hi-Hat | Right stick Y | `0b_1111_1111_xxxx_xxxx` |

MIDI data:

The drums send any unrecognised MIDI data as-is, using the extra bytes at the end of the report.
The Hi-Hat pedal sends MIDI data here alongside its Kick input, which is how you can differenciate it from a standard kick input.

An example packet for the Hi-Hat pedal would look like the following:
`99 64 1C`: Note on (channel 10, note 100, velocity 28)

As an additional note, Guitar Hero drumkits constantly hold the left stick click input. This is used by the Guitar Hero games in addition to the Force Feedback capability flag for differentiation, however just checking the Force Feedback flag is enough for any new software.

### As A Struct

```cpp
struct XInputFiveLaneDrumsGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool isFiveLane : 1;
    bool : 1;

    bool kick : 1;
    bool orange : 1;
    bool guide : 1;
    bool : 1;

    bool green : 1;
    bool red : 1;
    bool blue : 1;
    bool yellow : 1;

    uint8_t unused1[2];
    int16le_t unused2;
    uint8_t greenVelocity;
    uint8_t redVelocity;
    uint8_t yellowVelocity;
    uint8_t blueVelocity;
    uint8_t orangeVelocity;
    uint8_t kickVelocity;

    #ifdef USING_XUSB
        uint8_t midiPacket[6];
    #endif
} __attribute__((packed));
```

## References

- https://github.com/raphaelgoulart/360GHDrums2Midi
- https://github.com/mcclymont/ps360prodrummer
  - Note: This program uses HID to interact with the drumkit instead of XInput. Xbox 360 devices via HID have different layouts compared to XInput, and the inputs behave differently. While you can compare byte offsets and figure out what's equivalent to what, the actual values do not work the same (sticks are centered at `0x8000` instead of 0, triggers are mashed into one value, d-pad uses states instead of flags).
