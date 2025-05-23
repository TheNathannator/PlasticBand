# Xbox One Rock Band Guitars

## Controller Info

- Vendor/product ID:
  - Stratocaster (MadCatz): `0738:4161`
  - Jaguar (PDP): `0E6F:0170`
  - Riffmaster (PDP): `0E6F:0248`
- Interface GUIDs:
  - Stratocaster (MadCatz):
    - `0D2AE438-7F7D-4933-8693-30FC55018E77` (Primary)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
  - Jaguar (PDP):
    - `1A266AF6-3A46-45E3-B9B6-0F2C0B2C1EBE` (Primary)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
  - Riffmaster (PDP):
    - No unique interface GUID, must be distinguished by vendor/product ID.
    - `1A266AF6-3A46-45E3-B9B6-0F2C0B2C1EBE` (Primary; same as Jaguar)
    - `ECDDD2FE-D387-4294-BD96-1A712E3DC77D` (Console function map)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
- Class strings:
  - Stratocaster (MadCatz):
    - `MadCatz.Xbox.Guitar.Stratocaster` (Primary)
    - `Windows.Xbox.Input.NavigationController`
  - Jaguar (PDP):
    - `PDP.Xbox.Guitar.Jaguar` (Primary)
    - `Windows.Xbox.Input.NavigationController`
  - Riffmaster (PDP):
    - `PDP.Xbox.Guitar.Jaguar` (Primary)
    - `Windows.Xbox.Input.NavigationController`

## Input Command Info

### Command ID `0x20`: Input State

Length: 10 bytes

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu button
  - Byte 0, bit 3 (`0x08`) - View button
  - Byte 0, bit 4 (`0x10`) - Green fret flag
  - Byte 0, bit 5 (`0x20`) - Red fret flag
  - Byte 0, bit 6 (`0x40`) - Blue fret flag
  - Byte 0, bit 7 (`0x80`) - Yellow fret flag
  - Byte 1, bit 0 (`0x01`) - D-pad/strum up
  - Byte 1, bit 1 (`0x02`) - D-pad/strum down
  - Byte 1, bit 2 (`0x04`) - D-pad left
  - Byte 1, bit 3 (`0x08`) - D-pad right
  - Byte 1, bit 4 (`0x10`) - Orange fret flag
  - Byte 1, bit 5 (`0x20`) - Unused
  - Byte 1, bit 6 (`0x40`) - Solo fret flag / Riffmaster joystick click
    - Due to the overlap between fret flags and the joystick, using the fret flags for fret inputs is not recommended. Additionally, the joystick click should only be recognized if none of the solo frets are pressed.
  - Byte 1, bit 7 (`0x80`) - Unused
- Byte 2: Tilt
  - Nominally, `0x00` when parallel, `0xFF` when straight up.
- Byte 3: Whammy bar
  - Ranges from `0x00` when not pressed to `0xFF` when fully pressed.
- Byte 4: Pickup switch
  - Ranges from 0 to 4, with each number being a discrete notch of the switch.
  - Uses the top 4 bytes instead of the bottom 4, values from the Guitar Sniffer logs are `0x00`, `0x10`, `0x20`, `0x30`, and `0x40`.
- Byte 5: 8-bit upper fret bitmask
  - Bit 0 (`0x01`) - Green
  - Bit 1 (`0x02`) - Red
  - Bit 2 (`0x04`) - Yellow
  - Bit 3 (`0x08`) - Blue
  - Bit 4 (`0x10`) - Orange
- Byte 6: 8-bit lower (solo) fret bitmask
  - Same as previous
- Byte 7: Auto-calibration light sensor
  - Must be enabled first using the [`0x21` output report](#command-id-0x21-auto-calibration-mode).
- Bytes 8-9: Auto-calibration audio sensor (little-endian, unsigned)
  - Must be enabled first using the [`0x21` output report](#command-id-0x21-auto-calibration-mode).

#### Riffmaster Additions

The Riffmaster expands this input report to 28 bytes long, and adds a joystick and the Share button.

- Bytes 10-11: Joystick X (little-endian, signed)
  - Left is negative, right is positive.
- Bytes 12-13: Joystick Y (little-endian, signed)
  - Up is positive, down is negative.
- Byte 14: Console function buttons
  - Byte 14, bit 0 (`0x01`): Share button
- Bytes 15-27: Unknown

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

    uint8_t unknown[3];
} __attribute__((packed));

struct GipRiffmasterGuitarState : GipGuitarState
{
    int16le_t joystickX;
    int16le_t joystickY;

    bool share : 1;
    bool : 7;

    uint8_t unknown2[13];

    bool joystickClick() { return soloFlag && !(soloGreen | soloRed | soloYellow | soloBlue | soloOrange); }
} __attribute__((packed));
```

## Output Command Info

### Command ID `0x21`: Auto-Calibration Mode

- Length: 1 byte

This request is sent to change which auto-calibration sensors are enabled on the guitar.

- Byte 0: Sensor mode
  - 0: Disabled
  - 1: Light sensor
  - 2: Audio sensor
  - 254: Both light and audio

## References

- [RB4InstrumentMapper](https://github.com/TheNathannator/RB4InstrumentMapper)
- [GuitarSniffer](https://github.com/artman41/guitarsniffer)
  - [Guitar packet logs](https://1drv.ms/f/s!AgQGk0OeTMLwhA-uDO9IQHEHqGhv)
  - Guitar packet spreadsheets: [New](https://docs.google.com/spreadsheets/d/1ITZUvRniGpfS_HV_rBpSwlDdGukc3GC1CeOe7SavQBo/edit?usp=sharing), [Old](https://1drv.ms/x/s!AgQGk0OeTMLwg3GBDXFUC3Erj4Wb)
