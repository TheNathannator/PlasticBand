# Xbox One Rock Band Guitars

## Controller Info

- Vendor ID:
  - Stratocaster (MadCatz): `0x0738`
  - Jaguar (PDP): (Unknown)
- Product ID:
  - Stratocaster: `0x4161`
  - Jaguar: (Unknown)
- Interface GUID:
  - Stratocaster: `0D2AE438-7F7D-4933-8693-30FC55018E77`
  - Jaguar: (Unknown, likely the same)
- Class string:
  - Stratocaster: `MadCatz.Xbox.Guitar.Stratocaster`
  - Jaguar: (Unknown)

## Input Command Info

Command ID: `0x20`

- Length:
  - Typically 10 bytes
  - Descriptor reports length as 14 bytes
- General format: `<buttons> <tilt> <whammy> <pickup> <upper frets> <lower/solo frets> <unused[3]>`
  - `buttons`: 16-bit button bitmask
    - Bit 0 (`0x0001`) - Sync button
    - Bit 1 (`0x0002`) - Unused
    - Bit 2 (`0x0004`) - Menu Button
    - Bit 3 (`0x0008`) - View Button
    - Bit 4 (`0x0010`) - Green Fret Flag (equivalent to A Button)
    - Bit 5 (`0x0020`) - Red Fret Flag (equivalent to B Button)
    - Bit 6 (`0x0040`) - Blue Fret Flag (equivalent to X Button)
    - Bit 7 (`0x0080`) - Yellow Fret Flag (equivalent to Y Button)
    - Bit 8 (`0x0100`) - D-pad Up/Strum Up
    - Bit 9 (`0x0200`) - D-pad Down/Strum Down
    - Bit 10 (`0x0400`) - D-pad Left
    - Bit 11 (`0x0800`) - D-pad Right
    - Bit 12 (`0x1000`) - Orange Fret Flag (equivalent to Left Bumper)
    - Bit 13 (`0x2000`) - Unused (equivalent to Right Bumper)
    - Bit 14 (`0x4000`) - Solo Fret Flag (equivalent to Left Stick Press)
    - Bit 15 (`0x8000`) - Unused (equivalent to Right Stick Press)
  - `tilt`: 8-bit tilt axis
    - Has a minimum value of `0x70`, angles below this point register as just `0x00`
  - `whammy`: 8-bit whammy bar axis
  - `pickup`: 8-bit pickup switch axis
    - Seems to use top 4 bytes, values from the Guitar Sniffer logs are `0x00`, `0x10`, `0x20`, `0x30`, and `0x40`
  - `upper frets`, `lower/solo frets`: 8-bit fret bitmasks
    - Bit 0 (`0x01`) - Green
    - Bit 1 (`0x02`) - Red
    - Bit 2 (`0x04`) - Yellow
    - Bit 3 (`0x08`) - Blue
    - Bit 4 (`0x10`) - Orange
    - Bits 5-7 - Unused
  - `unused[3]`: unknown data values
    - These are most likely used for the auto-calibration sensors once those are activated.

  ```c
  struct GipGuitarState
  {
      bool sync : 1;
      bool reserved : 1;
      bool menu : 1;
      bool view : 1;

      bool greenFlag : 1;
      bool redFlag : 1;
      bool blueFlag : 1;
      bool yellowFlag : 1;

      bool dpadStrumUp : 1;
      bool dpadStrumDown : 1;
      bool dpadLeft : 1;
      bool dpadRight : 1;

      bool orangeFlag : 1;
      bool unused1 : 1;
      bool soloFlag : 1;
      bool unused2 : 1;

      uint8_t tilt;
      uint8_t whammy;
      uint8_t pickup;

      bool upperGreen : 1;
      bool upperRed : 1;
      bool upperYellow : 1;
      bool upperBlue : 1;
      bool upperOrange : 1;
      bool upperPadding : 3;

      bool lowerGreen : 1;
      bool lowerRed : 1;
      bool lowerYellow : 1;
      bool lowerBlue : 1;
      bool lowerOrange : 1;
      bool lowerPadding : 3;

      uint8_t unknown[3];
  }
  ```

## Output Command Info

Command ID: `0x21`

- Length: 5 bytes
- Data is unknown. This info comes from the guitar's descriptor.
- This is most likely used for enabling/disabling the auto-calibration sensors.

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [GuitarSniffer](https://github.com/artman41/guitarsniffer)
  - [Guitar packet logs](https://1drv.ms/f/s!AgQGk0OeTMLwhA-uDO9IQHEHqGhv)
  - Guitar packet spreadsheets: [New](https://docs.google.com/spreadsheets/d/1ITZUvRniGpfS_HV_rBpSwlDdGukc3GC1CeOe7SavQBo/edit?usp=sharing), [Old](https://1drv.ms/x/s!AgQGk0OeTMLwg3GBDXFUC3Erj4Wb)
