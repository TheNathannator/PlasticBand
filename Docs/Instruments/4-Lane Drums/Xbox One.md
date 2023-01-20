# Xbox One Rock Band Drums

## Controller Info

- Vendor ID:
  - MadCatz: (Unknown)
  - PDP: `0x0E6F`
- Product ID:
  - MadCatz: (Unknown)
  - PDP: `0x0171`
- Interface GUID:
  - MadCatz: (Unknown, possibly the same as the PDP one)
  - PDP:
    - Primary: `A503F9B0-955E-47C4-A2ED-B1336FA7703E`
    - Secondary:
      - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
      - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Unknown)
- Class string:
  - MadCatz: (Unknown)
  - PDP:
    - Primary: `PDP.Xbox.Drums.Tablah`
    - Secondary: `Windows.Xbox.Input.NavigationController`

NOTE: These drumkits typically send every packet twice! Respecting the packet count is vital!

## Input Command Info

### Command ID `0x20`: Input State

Length: Typically 6 bytes (descriptor reports max length as 10 bytes)

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync Button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu Button
  - Byte 0, bit 3 (`0x08`) - View Button
  - Byte 0, bit 4 (`0x10`) - A Button/Green Pad
  - Byte 0, bit 5 (`0x20`) - B Button/Red Pad
  - Byte 0, bit 6 (`0x40`) - X Button
  - Byte 0, bit 7 (`0x80`) - Y Button
  - Byte 1, bit 0 (`0x01`) - D-pad Up
  - Byte 1, bit 1 (`0x02`) - D-pad Down
  - Byte 1, bit 2 (`0x04`) - D-pad Left
  - Byte 1, bit 3 (`0x08`) - D-pad Right
  - Byte 1, bit 4 (`0x10`) - 1st Kick Pedal (equivalent to Left Bumper)
  - Byte 1, bit 5 (`0x20`) - 2nd Kick Pedal (equivalent to Right Bumper)
  - Byte 1, bit 6 (`0x40`) - Unused (equivalent to Left Stick Press)
  - Byte 1, bit 7 (`0x80`) - Unused (equivalent to Right Stick Press)
- Bytes 2-5 - Velocities
  - Byte 2, bits 0-3 (`0x0F`) - Yellow Pad
  - Byte 2, bits 4-7 (`0xF0`) - Red Pad
  - Byte 3, bits 0-3 (`0x0F`) - Green Pad
  - Byte 3, bits 4-7 (`0xF0`) - Blue Pad
  - Byte 4, bits 0-3 (`0x0F`) - Blue Cymbal
  - Byte 4, bits 4-7 (`0xF0`) - Yellow Cymbal
  - Byte 5, bits 4-7 (`0xF0`) - Green Cymbal
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
}
```

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [DrumSniffer](https://github.com/Dunkalunk/guitarsniffer)
