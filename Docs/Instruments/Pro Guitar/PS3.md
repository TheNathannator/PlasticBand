# PS3 Rock Band 3 Pro Guitar

## Controller Info

- Vendor ID: `0x12BA` ("Licensed by Sony Computer Entertainment America")
- Product ID:
  - Mustang: `0x2430`
  - Squire: `0x2530` (assumed based on patterns with other RB device PIDs)
  - MIDI Pro Adapter: `0x2538`
- PS3 ID: 

## Input Info

Face buttons and d-pad function as normal.

Fret numbers:

- For simplicity and clarity, some single-byte values are combined here little-endian-wise into a single 16-bit integer.

| String | Input                     | Byte Offset | Bits                     |
| :----- | :----                     | :---------- | :---                     |
| Low E  | Right Thumb X + Y         | 6 + 7       | `0b_xxxx_xxxx_xxx1_1111` |
| A      | Right Thumb X + Y         | 6 + 7       | `0b_xxxx_xx11_111x_xxxx` |
| D      | Right Thumb X + Y         | 6 + 7       | `0b_x111_11xx_xxxx_xxxx` |
| G      | Pressure D-pad Up + Right | 8 + 9       | `0b_xxxx_xxxx_xxx1_1111` |
| B      | Pressure D-pad Up + Right | 8 + 9       | `0b_xxxx_xx11_111x_xxxx` |
| High E | Pressure D-pad Up + Right | 8 + 9       | `0b_x111_11xx_xxxx_xxxx` |

String velocity:

| String | Input               | Byte Offset | Bits           |
| :----- | :----               | :---------- | :---           |
| Low E  | Pressure D-pad Left | 10          | `0b_x111_1111` |
| A      | Pressure D-pad Down | 11          | `0b_x111_1111` |
| D      | Pressure L2         | 12          | `0b_x111_1111` |
| G      | Pressure R2         | 13          | `0b_x111_1111` |
| B      | Pressure L1         | 14          | `0b_x111_1111` |
| High E | Pressure R1         | 15          | `0b_x111_1111` |

Standard 5-fret color flags:

| Fret Color | Input                | Byte Offset | Bits           |
| :--------- | :----                | :---------- | :---           |
| Solo flag  | Pressure D-pad Right | 9           | `0b_1xxx_xxxx` |
| Green      | Pressure D-pad Left  | 10          | `0b_1xxx_xxxx` |
| Red        | Pressure D-pad Down  | 11          | `0b_1xxx_xxxx` |
| Yellow     | Pressure L2          | 12          | `0b_1xxx_xxxx` |
| Blue       | Pressure R2          | 13          | `0b_1xxx_xxxx` |
| Orange     | Pressure L1          | 14          | `0b_1xxx_xxxx` |

Tilt and Auto-Calibration:

- Tilt is duplicated across 3 bytes, 2 of which are used for auto-calibration sensors when enabled. When no sensors are enabled, all 3 of the inputs listed below are tilt.

- Microphone: Pressure Δ (byte offset 16)
  - When enabled, neutral at `0x7F`, and decreases when sound is picked up.
- Light sensor: Pressure ○ (byte offset 17)
  - When enabled, neutral at `0x00`, and increases when light is detected.
- Tilt: Pressure × (byte offset 18)
  - `0x7F` when tilted, `0x40` when not tilted. (TODO: needs verification) 

Pedal port:

- Pedal press: Pressure □ (byte offset 19),  `0b_1000_0000`
- Pedal connection: Accel X high byte (byte offset 21), `0b_0000_0001`

### As A Struct

```cpp
struct Ps3Report
{
    uint8_t reportId;

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
  
    uint8_t autoCal_Microphone; // When the sensor isn't activated, this
    uint8_t autoCal_Light; // and this just duplicate the tilt axis
    uint8_t tilt;

    uint8_t : 7;
    bool pedal : 1;

    uint8_t unused2;

    bool pedalConnection : 1;
    uint8_t : 7;

    int16_t unused3[2];

    uint8_t counter; // Unsure what this is, but this is what it's defined as in the spreadsheet linked below.
                     // No description is provided for it until more investigation can be done.

    uint8_t unused4;
};
```

## References

- https://jasonharley2o.com/wiki/doku.php?id=rb3mustang
- https://docs.google.com/spreadsheets/d/1Y3QM1tEcf0bGiUTjT7R-3mwEAKrCL0qYoySmk3RLo8c/edit?usp=sharing
