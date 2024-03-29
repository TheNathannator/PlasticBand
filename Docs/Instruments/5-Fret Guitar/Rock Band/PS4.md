# PS4 Rock Band Guitars

## Device Info

- Vendor/product ID:
  - Stratocaster (MadCatz): `0738:8261`
  - Jaguar (PDP): `0E6F:0173`
- Revision:
  - Stratocaster:
  - Jaguar:
- Device name:
  - Stratocaster:
  - Jaguar:

## Input Info

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

Strumbar: D-pad up/down

Whammy: Byte offset 44

- Ranges from `0x00` when not pressed to `0xFF` when fully pressed.

Tilt: Byte offset 45

- Nominally, `0x00` when parallel, `0xFF` when straight up.

Pickup switch: Byte offset 43

- Ranges from 0 to 4, with each number being a discrete notch of the switch.

### As A Struct

```cpp
struct PS4RockBandGuitarState
{
    uint8_t reportId = 0x01;

    uint8_t unused1[4];

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum : 4;
    bool blue_flag : 1;
    bool green_flag : 1;
    bool red_flag : 1;
    bool yellow_flag : 1;

    bool orange_flag : 1;
    uint8_t : 3;
    bool share : 1;
    bool options : 1;
    bool solo_flag : 1;
    bool : 1;

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
    bool : 1;
    bool : 1;
    bool : 1;

    bool soloGreen : 1;
    bool soloRed : 1;
    bool soloYellow : 1;
    bool soloBlue : 1;
    bool soloOrange : 1;
    bool : 1;
    bool : 1;
    bool : 1;

    uint8_t unused4[26];
    uint32_t crc32;
} __attribute__((__packed__));
```

## References

- https://rb4.app/js/app.js
