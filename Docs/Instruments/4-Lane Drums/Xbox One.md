# Xbox One Rock Band Drums

## Controller Info

- Vendor ID:
  - MadCatz: (Unknown)
  - PDP: `0x0E6F`
- Product ID:
  - MadCatz: (Unknown)
  - PDP: `0x0171`
- Interface GUID:
  - MadCatz: (Unknown, probably the same as the PDP one)
  - PDP: `A503F9B0-955E-47C4-A2ED-B1336FA7703E`
- Class string:
  - MadCatz: (Unknown)
  - PDP: `PDP.Xbox.Drums.Tablah`

## Input Command Info

Command ID: `0x20`

- Length:
  - Typically 6 bytes
  - Descriptor reports length as 10 bytes
- General format: `<buttons> <pad velocities> <cymbal velocities>`

Bytes:

- `buttons`: 16-bit button bitmask
  - Bit 0 (`0x0001`) - Sync button
  - Bit 1 (`0x0002`) - Unused
  - Bit 2 (`0x0004`) - Menu Button
  - Bit 3 (`0x0008`) - View Button
  - Bit 4 (`0x0010`) - A Button/Green Pad
  - Bit 5 (`0x0020`) - B Button/Red Pad
  - Bit 6 (`0x0040`) - X Button
  - Bit 7 (`0x0080`) - Y Button
  - Bit 8 (`0x0100`) - D-pad Up
  - Bit 9 (`0x0200`) - D-pad Down
  - Bit 10 (`0x0400`) - D-pad Left
  - Bit 11 (`0x0800`) - D-pad Right
  - Bit 12 (`0x1000`) - 1st Kick Pedal (equivalent to Left Bumper)
  - Bit 13 (`0x2000`) - 2nd Kick Pedal (equivalent to Right Bumper)
  - Bit 14 (`0x4000`) - Unused (equivalent to Left Stick Press)
  - Bit 15 (`0x8000`) - Unused (equivalent to Right Stick Press)
- `pad velocities` - 16 bits for the pad velocities (remember that this is little-endian)
  - Bits 0-3 (`0x000F`) - Yellow Pad
  - Bits 4-7 (`0x00F0`) - Red Pad
  - Bits 8-11 (`0x0F00`) - Green Pad
  - Bits 12-15 (`0xF000`) - Blue Pad
  - Range seems to be from 0-7
- `cymbal velocities` - 16 bits for the cymbal velocities (remember that this is little-endian)
  - Bits 0-3 (`0x000F`) - Blue Cymbal
  - Bits 4-7 (`0x00F0`) - Yellow Cymbal
  - Bits 8-11 (`0x0F00`) - Unused
  - Bits 12-15 (`0xF000`) - Green Cymbal
  - Range seems to be from 0-7

  ```c
  struct GipDrumkitState
  {
      bool sync : 1;
      bool reserved : 1;
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
      bool soloFlag : 1;
      bool unused1 : 1;

      uint8_t yellowPad : 4;
      uint8_t redPad : 4;
      uint8_t greenPad : 4;
      uint8_t bluePad : 4;

      uint8_t blueCymbal : 4;
      uint8_t yellowCymbal : 4;
      uint8_t unused2 : 4;
      uint8_t greenCymbal : 4;
  }
  ```

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [DrumSniffer](https://github.com/Dunkalunk/guitarsniffer)
