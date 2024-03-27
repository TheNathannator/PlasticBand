# PS3 and Wii Rock Band 3 Keyboard

## Controller Info

PS3:

- Vendor/product ID: `12BA:2330`
- Revision: `0x0005`
- Device name: `Harmonix RB3 Keyboard for PlayStation®3`
- PS3 ID: `0x06`

Wii:

- Vendor/product ID: `1BAD:3330`
- Revision: `0x0005`
- Device name: `Harmonix RB3 Keyboard for Nintendo Wii`

MIDI Pro Adapter information may be found in [its own document](../../Other/Rock%20Band%20Adapters/MIDI%20Pro%20Adapters.md).

## Input Info

Face buttons and d-pad function as normal.

### Keys

C1 is the leftmost key, C3 is the rightmost key.

| Key | Input                | Byte Offset | Bits           |
| :-- | :----                | :---------- | :--:           |
| C1  | Right stick X        | 5           | `0b_1xxx_xxxx` |
| Db1 | Right stick X        | 5           | `0b_x1xx_xxxx` |
| D1  | Right stick X        | 5           | `0b_xx1x_xxxx` |
| Eb1 | Right stick X        | 5           | `0b_xxx1_xxxx` |
| E1  | Right stick X        | 5           | `0b_xxxx_1xxx` |
| F1  | Right stick X        | 5           | `0b_xxxx_x1xx` |
| Gb1 | Right stick X        | 5           | `0b_xxxx_xx1x` |
| G1  | Right stick X        | 5           | `0b_xxxx_xxx1` |
|     |                      |             |                |
| Ab1 | Right stick Y        | 6           | `0b_1xxx_xxxx` |
| A1  | Right stick Y        | 6           | `0b_x1xx_xxxx` |
| Bb1 | Right stick Y        | 6           | `0b_xx1x_xxxx` |
| B1  | Right stick Y        | 6           | `0b_xxx1_xxxx` |
| C2  | Right stick Y        | 6           | `0b_xxxx_1xxx` |
| Db2 | Right stick Y        | 6           | `0b_xxxx_x1xx` |
| D2  | Right stick Y        | 6           | `0b_xxxx_xx1x` |
| Eb2 | Right stick Y        | 6           | `0b_xxxx_xxx1` |
|     |                      |             |                |
| E2  | Pressure d-pad up    | 7           | `0b_1xxx_xxxx` |
| F2  | Pressure d-pad up    | 7           | `0b_x1xx_xxxx` |
| Gb2 | Pressure d-pad up    | 7           | `0b_xx1x_xxxx` |
| G2  | Pressure d-pad up    | 7           | `0b_xxx1_xxxx` |
| Ab2 | Pressure d-pad up    | 7           | `0b_xxxx_1xxx` |
| A2  | Pressure d-pad up    | 7           | `0b_xxxx_x1xx` |
| Bb2 | Pressure d-pad up    | 7           | `0b_xxxx_xx1x` |
| B2  | Pressure d-pad up    | 7           | `0b_xxxx_xxx1` |
|     |                      |             |                |
| C3  | Pressure d-pad right | 8           | `0b_1xxx_xxxx` |

### Velocity

| Key     | Input                | Byte Offset | Bits           |
| :--     | :----                | :---------- | :--:           |
| 1st key | Pressure d-pad right | 8           | `0b_x111_1111` |
| 2nd key | Pressure d-pad left  | 9           | `0b_x111_1111` |
| 3rd key | Pressure d-pad down  | 10          | `0b_x111_1111` |
| 4th key | Pressure L2          | 11          | `0b_x111_1111` |
| 5th key | Pressure R2          | 12          | `0b_x111_1111` |

See the [general notes doc](General%20Notes.md) for details on [velocity behavior](General%20Notes.md#key-velocities) and [pairing velocities with keys](General%20Notes.md#pairing-keys-and-velocities).

### Other

Overdrive button: Pressure L1 (byte offset 13), `0b_1xxx_xxxx`

Pedal port:

- Digital input: Pressure R1 (byte offset 14), `0b_1xxx_xxxx`
- Analog input: Pressure R1 (byte offset 14), `0b_x111_1111`
- Pedal connection: Accel X high byte (byte offset 20), `0b_0000_0001`

- This value may have an inverted range (0 = max, 127 = min). Needs confirmation though, as this is taken from observatons on the Xbox 360 keyboard.
- These bits are all 1 when nothing is plugged in.

Effects touchpad: Pressure Δ (byte offset 15), `0b_x111_1111`

### As A Struct

```cpp
struct PS3KeytarState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

    bool : 1;
    bool : 1;
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t unused1[2];

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
    uint8_t touchPad : 7;
    bool : 1;

    uint8_t unused2[4];

    bool pedalConnection : 1; // Always 0 with the MIDI Pro Adapter
    uint8_t : 7;

    int16_t unused3[2];

    uint8_t pressCount; // Unsure what this is, but this is what it's defined as in the spreadsheet linked below.
                        // No description is provided for it until more investigation can be done.

    uint8_t unused4;
} __attribute__((__packed__));
```

## Feature Reports

### Enable Keyboard Data (PS3)

The PS3 keyboard requires a feature report to be sent before it will output information for keys and such:

```cpp
struct PS3KeyboardEnable
{
    uint8_t reportId = 0x00;

    uint8_t data[40] = {
        0xE9, 0x00, 0x89, 0x1B, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    };
} __attribute__((__packed__));
```

The Wii Keyboard does not require this report, it sends the data regardless.

**TODO:** Is this supposed to be sent as 5 separate reports like the Pro Guitar auto-calibration reports are, or does it work either way?

## References

- https://jasonharley2o.com/wiki/doku.php?id=rb3keyboard
- https://docs.google.com/spreadsheets/d/1Y3QM1tEcf0bGiUTjT7R-3mwEAKrCL0qYoySmk3RLo8c/edit?usp=sharing
