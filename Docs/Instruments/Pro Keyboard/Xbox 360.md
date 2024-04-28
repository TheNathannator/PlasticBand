# Xbox 360 Rock Band 3 Keyboard

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: 15, not part of XInput standards
- Stick hardware IDs:
  - Vendor ID (left stick X): `0x1BAD`
  - Product ID (left stick Y): `0x1330`
  - Revision (right stick X): `0x0004`

## Input Info

Face buttons work like a standard Xbox 360 controller.

Keys:

- C1 is the leftmost key, C3 is the rightmost key.

| Key | Axis          | Bits                     |
| :-- | :---          | :--:                     |
| C1  | Left trigger  | `0b_1xxx_xxxx`           |
| Db1 | Left trigger  | `0b_x1xx_xxxx`           |
| D1  | Left trigger  | `0b_xx1x_xxxx`           |
| Eb1 | Left trigger  | `0b_xxx1_xxxx`           |
| E1  | Left trigger  | `0b_xxxx_1xxx`           |
| F1  | Left trigger  | `0b_xxxx_x1xx`           |
| Gb1 | Left trigger  | `0b_xxxx_xx1x`           |
| G1  | Left trigger  | `0b_xxxx_xxx1`           |
|     |               |                          |
| Ab1 | Right trigger | `0b_1xxx_xxxx`           |
| A1  | Right trigger | `0b_x1xx_xxxx`           |
| Bb1 | Right trigger | `0b_xx1x_xxxx`           |
| B1  | Right trigger | `0b_xxx1_xxxx`           |
| C2  | Right trigger | `0b_xxxx_1xxx`           |
| Db2 | Right trigger | `0b_xxxx_x1xx`           |
| D2  | Right trigger | `0b_xxxx_xx1x`           |
| Eb2 | Right trigger | `0b_xxxx_xxx1`           |
|     |               |                          |
| E2  | Left stick X  | `0b_xxxx_xxxx_1xxx_xxxx` |
| F2  | Left stick X  | `0b_xxxx_xxxx_x1xx_xxxx` |
| Gb2 | Left stick X  | `0b_xxxx_xxxx_xx1x_xxxx` |
| G2  | Left stick X  | `0b_xxxx_xxxx_xxx1_xxxx` |
| Ab2 | Left stick X  | `0b_xxxx_xxxx_xxxx_1xxx` |
| A2  | Left stick X  | `0b_xxxx_xxxx_xxxx_x1xx` |
| Bb2 | Left stick X  | `0b_xxxx_xxxx_xxxx_xx1x` |
| B2  | Left stick X  | `0b_xxxx_xxxx_xxxx_xxx1` |
|     |               |                          |
| C3  | Left stick X  | `0b_1xxx_xxxx_xxxx_xxxx` |

Velocities:

| Key     | Axis          | Bits                     |
| :--     | :---          | :--:                     |
| 1st key | Left stick X  | `0b_x111_1111_xxxx_xxxx` |
| 2nd key | Left stick Y  | `0b_xxxx_xxxx_x111_1111` |
| 3rd key | Left stick Y  | `0b_x111_1111_xxxx_xxxx` |
| 4th key | Right stick X | `0b_xxxx_xxxx_x111_1111` |
| 5th key | Right stick X | `0b_x111_1111_xxxx_xxxx` |

- See the [general notes doc](General%20Notes.md) for details on [velocity behavior](General%20Notes.md#key-velocities) and [pairing velocities with keys](General%20Notes.md#pairing-keys-and-velocities).

Overdrive button: Right Stick Y, `0b_xxxx_xxxx_1xxx_xxxx`

Pedal port digital input: Right Stick Y, `0b_1xxx_xxxx_xxxx_xxxx`

Pedal port analog input: Right Stick Y, `0b_x111_1111_xxxx_xxxx`

- This value appears to have an inverted range (0 = max, 127 = min). Not 100% sure on that though as this wasn't tested with an actual analog pedal, instead I used a headphone/microphone splitter on a pair of headphones that have a built-in mic and connected the mic part into the port.
- These bits are all 1 when nothing is plugged in.

### [XUSB-Only Info](../../_Templates/Xbox%20360%20Base.md#xusb-only-info)

- Effects touchpad: `reserved[0]` (Byte offet 12)
- Pedal connection: `reserved[5]` (byte offset 17), `0b_0000_0001`

### As A Struct

```cpp
struct XInputKeytarGamepad
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

    bool key8 : 1;
    bool key7 : 1;
    bool key6 : 1;
    bool key5 : 1;
    bool key4 : 1;
    bool key3 : 1;
    bool key2 : 1;
    bool key1 : 1;

    bool key16 : 1;
    bool key15 : 1;
    bool key14 : 1;
    bool key13 : 1;
    bool key12 : 1;
    bool key11 : 1;
    bool key10 : 1;
    bool key9 : 1;

    bool key24 : 1;
    bool key23 : 1;
    bool key22 : 1;
    bool key21 : 1;
    bool key20 : 1;
    bool key19 : 1;
    bool key18 : 1;
    bool key17 : 1;

    uint8_t velocity1 : 7;
    bool key25 : 1;
    uint8_t velocity2 : 7;
    bool : 1;
    uint8_t velocity3 : 7;
    bool : 1;
    uint8_t velocity4 : 7;
    bool : 1;
    uint8_t velocity5 : 7;
    bool : 1;

    uint8_t : 7;
    bool overdrive : 1;
    uint8_t pedalAnalog : 7;
    bool pedalDigital : 1;

#ifdef USING_XUSB
    uint8_t touchPad : 7;
    bool : 1;

    uint8_t unused2[4];

    bool pedalConnection : 1; // If this matches PS3 MPA behavior, always 0 with the MIDI Pro Adapter
    uint8_t : 7;
#endif
} __attribute__((__packed__));
```

## References

- Observations from my own hardware
- https://github.com/bearzly/RockBandPiano
- https://github.com/TheNathannator/RB3_X360_Keyboard
