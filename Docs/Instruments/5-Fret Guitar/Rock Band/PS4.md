# PS4 Rock Band Guitars

## Device Info

- Vendor/product ID:
  - Stratocaster (MadCatz): `0738:8261`
  - Jaguar (PDP): `0E6F:0173`
  - Riffmaster, PS4 mode (PDP): `0E6F:024A`
- Revision:
  - Stratocaster:
  - Jaguar:
  - Riffmaster: `0x0101`
- Device name:
  - Stratocaster:
  - Jaguar:
  - Riffmaster: `PDP RiffMaster Guitar for PS4`

## Input Info

This device sends a report modelled after the one [documented here](../../../Controller%20Communication%20Basics/PS4%20Instruments.md). Refer to that document for the base layout.

Report ID: `0x01`

Options, Share, PS button, and d-pad work as normal.

Upper frets:

| Fret   | Button | Additional
| :--:   | :----: | :---------
| Green  | ×      | Byte offset 46 bit 0
| Red    | ○      | Byte offset 46 bit 1
| Yellow | Δ      | Byte offset 46 bit 2
| Blue   | □      | Byte offset 46 bit 3
| Orange | L1     | Byte offset 46 bit 4

Lower frets:

| Fret   | Buttons | Additional
| :--:   | :------ | :---------
| Green  | × + L3  | Byte offset 47 bit 0
| Red    | ○ + L3  | Byte offset 47 bit 1
| Yellow | Δ + L3  | Byte offset 47 bit 2
| Blue   | □ + L3  | Byte offset 47 bit 3
| Orange | L1 + L3 | Byte offset 47 bit 4

Or, as flags:

| Flag      | Button |
| :--:      | :----: |
| Green     | ×      |
| Red       | ○      |
| Yellow    | Δ      |
| Blue      | □      |
| Orange    | L1     |
| Solo flag | L3     |

- Note: Using these flags is not recommended, use the values in byte offsets 46-47 instead.

Strumbar: D-pad up/down

Whammy: Byte offset 44

- Ranges from `0x00` when not pressed to `0xFF` when fully pressed.

Tilt: Byte offset 45

- Nominally, `0x00` when parallel, `0xFF` when straight up.

Pickup switch: Byte offset 43

- Ranges from 0 to 4, with each number being a discrete notch of the switch.

### Riffmaster Additions

The Riffmaster features a joystick on the back of its headstock and a programmable P1 button.

Joystick:

| Input          | Report
| :---:          | :----:
| Joystick X     | Left stick X (byte offset 1)
| Joystick Y     | Left stick Y (byte offset 2)
| Joystick click | L3 button

- Reminder that the stick inputs are top-left oriented: 0 on the X axis is left, 0 on the Y axis is up.
- Note that the joystick click overlaps with the solo fret flag input. For this reason, it's recommended to rely only on the values in byte offsets 46-47 for reading fret inputs. Additionally, the stick click should only be respected if none of the solo fret bits in byte 47 are active.

P1 button: R3 by default

- This button is programmable and can be remapped to any other input available on the guitar. This is simply its default, which does not correspond to any other input on the guitar.

### As A Struct

```cpp
struct PS4RockBandGuitarState
{
    uint8_t reportId = 0x01;

#ifdef RIFFMASTER
    uint8_t joystickX;
    uint8_t joystickY;
    uint8_t unused1[2];
#else
    uint8_t unused1[4];
#endif

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum : 4;
    bool blueFlag : 1;
    bool greenFlag : 1;
    bool redFlag : 1;
    bool yellowFlag : 1;

    bool orangeFlag : 1;
    uint8_t : 3;
    bool share : 1;
    bool options : 1;
    bool soloFlag : 1;
#ifdef RIFFMASTER
    bool p1 : 1;
#else
    bool : 1;
#endif

    bool ps : 1;
    uint8_t : 7;

    uint8_t unused2[22];

    uint8_t powerLevel : 4;
    uint8_t : 4;

    uint8_t unused4[12];

    uint8_t pickup;
    uint8_t whammy;
    uint8_t tilt;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;
    bool orange : 1;
    bool : 3;

    bool soloGreen : 1;
    bool soloRed : 1;
    bool soloYellow : 1;
    bool soloBlue : 1;
    bool soloOrange : 1;
    bool : 3;

    uint8_t unused4[26];
    uint32_t crc32;

#ifdef RIFFMASTER
    bool joystickClick() { return soloFlag && !(soloGreen | soloRed | soloYellow | soloBlue | soloOrange); }
#endif
} __attribute__((__packed__));
```

## References

- https://rb4.app/js/app.js
