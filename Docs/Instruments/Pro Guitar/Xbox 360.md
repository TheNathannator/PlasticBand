# Xbox 360 Rock Band 3 Pro Guitar

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: 25, not part of XInput standards

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

### [XUSB-Only Info](../../_Templates/Xbox%20360%20Base.md#xusb-only-info)

The information here is speculated based on the PS3/Wii Pro guitar report, more research is needed to confirm it. The speculation is based on the Pro Keyboards having bitwise identical report data beyond a certain point of each report type; the Pro Guitars appear to have the same behavior.

Tilt and Auto-Calibration:

- Tilt is duplicated across 3 bytes, 2 of which are used for auto-calibration sensors when enabled. When no sensors are enabled, all 3 of the inputs listed below are tilt.

- Microphone: `reserved[0]` (byte offset 12)
  - When enabled, neutral at `0x7F`, and decreases when sound is picked up.
- Light sensor: `reserved[1]` (byte offset 13)
  - When enabled, neutral at `0x00`, and increases when light is detected.
- Tilt: `reserved[2]` (byte offset 14)
  - `0x7F` when tilted, `0x40` when not tilted. (TODO: needs verification) 

Pedal port:

- Pedal press: `reserved[3]` (byte offset 15),  `0b_1000_0000`
- Pedal connection: `reserved[5]` (byte offset 17), `0b_0000_0001`

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

#ifdef USING_XUSB
    uint8_t autoCal_Microphone; // When the sensor isn't activated, this
    uint8_t autoCal_Light; // and this just duplicate the tilt axis
    uint8_t tilt;

    uint8_t : 7;
    bool pedal : 1;

    uint8_t unused2;

    bool pedalConnection : 1;
    uint8_t : 7;

    // There's additional data in the PS3/Wii report, but the XUSB report
    // only has 6 additional bytes after the end of the XInput report.
#endif
} __attribute__((__packed__));
```

## References

- https://dwsk.proboards.com/thread/404/song-standard-advancements
- https://github.com/maxton/gamepad-midi/blob/master/LibPlasticInstrument/Native/Xinput.cs#L110 for subtype
