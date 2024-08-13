# Xbox One Rock Band Drums

## Controller Info

- Vendor/product ID:
  - MadCatz: `0738:4262`
  - PDP: `0E6F:0171`
- Interface GUIDs:
  - MadCatz:
    - `06182893-CCE0-4B85-9271-0A10DBAB7E07` (Primary)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
  - PDP:
    - `A503F9B0-955E-47C4-A2ED-B1336FA7703E` (Primary)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
- Class strings:
  - MadCatz:
    - `MadCatz.Xbox.Drums.Glam` (Primary)
    - `Windows.Xbox.Input.NavigationController`
  - PDP:
    - `PDP.Xbox.Drums.Tablah` (Primary)
    - `Windows.Xbox.Input.NavigationController`

NOTE: These drumkits typically send every packet twice! Respecting the packet count is vital!

## Input Command Info

### Command ID `0x20`: Input State

Length: 6 bytes

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu button
  - Byte 0, bit 3 (`0x08`) - View button
  - Byte 0, bit 4 (`0x10`) - A button/green pad
  - Byte 0, bit 5 (`0x20`) - B button/red pad
  - Byte 0, bit 6 (`0x40`) - X button
  - Byte 0, bit 7 (`0x80`) - Y button
  - Byte 1, bit 0 (`0x01`) - D-pad up
  - Byte 1, bit 1 (`0x02`) - D-pad down
  - Byte 1, bit 2 (`0x04`) - D-pad left
  - Byte 1, bit 3 (`0x08`) - D-pad right
  - Byte 1, bit 4 (`0x10`) - 1st kick pedal
  - Byte 1, bit 5 (`0x20`) - 2nd kick pedal
  - Byte 1, bit 6 (`0x40`) - Unused
  - Byte 1, bit 7 (`0x80`) - Unused
- Bytes 2-5 - Velocities
  - Byte 2, bits 0-3 (`0x0F`) - Yellow pad
  - Byte 2, bits 4-7 (`0xF0`) - Red pad
  - Byte 3, bits 0-3 (`0x0F`) - Green pad
  - Byte 3, bits 4-7 (`0xF0`) - Blue pad
  - Byte 4, bits 0-3 (`0x0F`) - Blue cymbal
  - Byte 4, bits 4-7 (`0xF0`) - Yellow cymbal
  - Byte 5, bits 4-7 (`0xF0`) - Green cymbal
  - Range seems to be 0-7

```cpp
struct GipDrumkitState
{
    bool sync : 1;
    bool : 1;
    bool menu : 1;
    bool view : 1;

    bool a : 1;
    bool b : 1;
    bool x : 1;
    bool y : 1;

    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool kick1 : 1;
    bool kick2 : 1;
    bool : 1;
    bool : 1;

    uint8_t yellowPad : 4;
    uint8_t redPad : 4;
    uint8_t greenPad : 4;
    uint8_t bluePad : 4;

    uint8_t blueCymbal : 4;
    uint8_t yellowCymbal : 4;
    uint8_t : 4;
    uint8_t greenCymbal : 4;
} __attribute__((__packed__));
```

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [DrumSniffer](https://github.com/Dunkalunk/guitarsniffer)
- https://rb4.app/js/app.js
