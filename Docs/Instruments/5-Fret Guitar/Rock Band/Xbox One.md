# Xbox One Rock Band Guitars

## Controller Info

- Vendor ID:
  - Stratocaster (MadCatz): `0x0738`
  - Jaguar (PDP): (Unknown)
- Product ID:
  - Stratocaster: `0x4161`
  - Jaguar: (Unknown)
- Interface GUID:
  - Stratocaster:
    - Primary: `0D2AE438-7F7D-4933-8693-30FC55018E77`
    - Secondary:
      - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
      - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Unknown)
  - Jaguar: (Unknown, possibly the same)
- Class string:
  - Stratocaster:
    - Primary: `MadCatz.Xbox.Guitar.Stratocaster`
    - Secondary: `Windows.Xbox.Input.NavigationController`
  - Jaguar: (Unknown)

## Input Command Info

### Command ID `0x20`: Input State

Length: 10 bytes

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync Button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu Button
  - Byte 0, bit 3 (`0x08`) - View Button
  - Byte 0, bit 4 (`0x10`) - Green Fret Flag (equivalent to A Button)
  - Byte 0, bit 5 (`0x20`) - Red Fret Flag (equivalent to B Button)
  - Byte 0, bit 6 (`0x40`) - Blue Fret Flag (equivalent to X Button)
  - Byte 0, bit 7 (`0x80`) - Yellow Fret Flag (equivalent to Y Button)
  - Byte 1, bit 0 (`0x01`) - D-pad Up/Strum Up
  - Byte 1, bit 1 (`0x02`) - D-pad Down/Strum Down
  - Byte 1, bit 2 (`0x04`) - D-pad Left
  - Byte 1, bit 3 (`0x08`) - D-pad Right
  - Byte 1, bit 4 (`0x10`) - Orange Fret Flag (equivalent to Left Bumper)
  - Byte 1, bit 5 (`0x20`) - Unused (equivalent to Right Bumper)
  - Byte 1, bit 6 (`0x40`) - Solo Fret Flag (equivalent to Left Stick Press)
  - Byte 1, bit 7 (`0x80`) - Unused (equivalent to Right Stick Press)
- Byte 2: Tilt
  - Has a minimum value of `0x70`, angles below this point register as just `0x00`
- Byte 3: Whammy bar
- Byte 4: Pickup switch
  - Seems to use top 4 bytes, values from the Guitar Sniffer logs are `0x00`, `0x10`, `0x20`, `0x30`, and `0x40`
- Byte 5: 8-bit upper fret bitmask
  - Bit 0 (`0x01`) - Green
  - Bit 1 (`0x02`) - Red
  - Bit 2 (`0x04`) - Yellow
  - Bit 3 (`0x08`) - Blue
  - Bit 4 (`0x10`) - Orange
- Byte 6: 8-bit lower fret bitmask
  - Same as previous
- Bytes 7-9: unknown
  - These are most likely used for the auto-calibration sensors once those are activated.

```cpp
struct GipGuitarState
{
    bool sync : 1;
    bool : 1;
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
    bool : 1;
    bool soloFlag : 1;
    bool : 1;

    uint8_t tilt;
    uint8_t whammy;
    uint8_t pickup;

    bool upperGreen : 1;
    bool upperRed : 1;
    bool upperYellow : 1;
    bool upperBlue : 1;
    bool upperOrange : 1;
    bool : 3;

    bool lowerGreen : 1;
    bool lowerRed : 1;
    bool lowerYellow : 1;
    bool lowerBlue : 1;
    bool lowerOrange : 1;
    bool : 3;

    uint8_t unknown[3];
}
```

## Output Command Info

### Command ID `0x21`

- Length: 1 byte

The data for this one is unknown, it's reported in the descriptor but unfortunately it doesn't provide any important info outside of the ID, length, and that it's an output command. It's probably used for enabling/disabling the auto-calibration sensors.

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [GuitarSniffer](https://github.com/artman41/guitarsniffer)
  - [Guitar packet logs](https://1drv.ms/f/s!AgQGk0OeTMLwhA-uDO9IQHEHqGhv)
  - Guitar packet spreadsheets: [New](https://docs.google.com/spreadsheets/d/1ITZUvRniGpfS_HV_rBpSwlDdGukc3GC1CeOe7SavQBo/edit?usp=sharing), [Old](https://1drv.ms/x/s!AgQGk0OeTMLwg3GBDXFUC3Erj4Wb)
