# Xbox One Guitar Hero Live Guitars

This one is interesting, it's basically just a PS3/Wii U GHL guitar masquerading as an Xbox One peripheral.

## Controller Info

- Vendor ID: `0x1430`
- Product ID: `0x079B`
- Interface GUIDs:
  - Primary: `FD12FDD9-8E73-47C7-A231-96268C38009A`
  - Secondary:
    - `082E402C-07DF-45E1-A5AB-A3127AF197B5` (Gamepad)
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Unknown)
- Class strings:
  - Primary: `Activision.Xbox.Input.GH7`
  - Secondary:
    - `Windows.Xbox.Input.Gamepad`
    - `Windows.Xbox.Input.NavigationController`

## Input Command Info

### Command ID `0x20`: Standard Input State

This section needs to be verified with additional hardware observations.

This command is the standard input report that the Xbox expects and uses for menu navigation. Presumably, it defaults to this when the keep-alive packet is not sent.

Based on the interface GUIDs, class strings, and command length listed in the descriptor, this report will likely match that of the standard gamepad, so that is what's listed here.

Length: 14 bytes

Although the readme for these docs says that little-endian is assumed, it's recommended to specify big-endian or little-endian for values spanning across multiple bytes. Signedness should also be specified.

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu button
  - Byte 0, bit 3 (`0x08`) - View button (Hero Power)
  - Byte 0, bit 4 (`0x10`) - A button (Black 1)
  - Byte 0, bit 5 (`0x20`) - B button (Black 2)
  - Byte 0, bit 6 (`0x40`) - X button (White 1)
  - Byte 0, bit 7 (`0x80`) - Y button (Black 3)
  - Byte 1, bit 0 (`0x01`) - D-pad up
  - Byte 1, bit 1 (`0x02`) - D-pad down
  - Byte 1, bit 2 (`0x04`) - D-pad left
  - Byte 1, bit 3 (`0x08`) - D-pad right
  - Byte 1, bit 4 (`0x10`) - Left bumper
  - Byte 1, bit 5 (`0x20`) - Right bumper
  - Byte 1, bit 6 (`0x40`) - Left stick press
  - Byte 1, bit 7 (`0x80`) - Right stick press
- Bytes 2-3: Left trigger (little-endian, unsigned)
  - Unpressed: 0, fully pressed: roughly `0x03FF`
- Bytes 4-5: Right trigger (little-endian, unsigned)
  - Same as above
- Bytes 6-7: Left stick X (little-endian, signed)
- Bytes 8-9: Left stick Y (little-endian, signed)
- Bytes 10-11: Right stick X (little-endian, signed)
- Bytes 12-13: Right stick Y (little-endian, signed)

```cpp
struct GipGamepadState
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

    bool leftShoulder : 1;
    bool rightShoulder : 1;
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    uint16_t leftTrigger;
    uint16_t rightTrigger;

    int16_t leftStickX;
    int16_t leftStickY;
    int16_t rightStickX;
    int16_t rightStickY;
}
```

### Command ID `0x21`: Guitar Input State

This command is used to send the actual guitar input reports.

The report format here is identical to that of the PS3/Wii U guitar.

Length: 27

Bytes:

- Byte 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - White 1
  - Byte 0, bit 1 (`0x02`) - Black 1
  - Byte 0, bit 2 (`0x04`) - Black 2
  - Byte 0, bit 3 (`0x08`) - Black 3
  - Byte 0, bit 4 (`0x10`) - White 2
  - Byte 0, bit 5 (`0x20`) - White 3
  - Byte 0, bit 6 (`0x40`) - Unused
  - Byte 0, bit 7 (`0x80`) - Unused
  - Byte 1, bit 0 (`0x01`) - Hero Power
  - Byte 1, bit 1 (`0x02`) - Pause
  - Byte 1, bit 2 (`0x04`) - GHTV
  - Byte 1, bit 3 (`0x08`) - Unused
  - Byte 1, bit 4 (`0x10`) - D-pad center
  - Byte 1, bit 5 (`0x20`) - Unused
  - Byte 1, bit 6 (`0x40`) - Unused
  - Byte 1, bit 7 (`0x80`) - Unused
- Byte 2: D-pad
  - Unlike other Xbox One peripherals, the d-pad here is reported using individual states instead of flags. Here's a visual representation of each state:

    ```
         0
      7     1
    6    15   2
      5     3
         4
    ```

- Byte 4: Strumbar
  - `0x80` when not strumming, `0x00` when strumming up, and `0xFF` when strumming down.
- Byte 5: Tilt extremes
  - `0xFF` when tilt is at its maximum, `0x00` when tilt is at its minimum, `0x80` otherwise.
- Byte 6: Whammy
  - `0x80` when not pressed, `0xFF` when fully pressed.
- Byte 7-18: Unused
- Bytes 19-20: Tilt (little-endian, unsigned)
  - While this only uses the bottom byte (byte 19), it's defined as a 16-bit little-endian number for consistency with the rest of the PS3 devices.
  - Maxes out at `0xFF`, bottoms out at `0x00`.
- Bytes 21-22: Tilt extreme indicator (little-endian, unsigned)
  - `0x180` when tilt is not at max or min, `0x200` when it is.
- Bytes 23-26: Unused
  - These bytes are two more 16-bit little-endian values which rest at `0x200`.

```cpp
struct GipGHLGuitarState
{
    bool white1 : 1;
    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool : 1;
    bool : 1;

    bool heroPower : 1;
    bool pause : 1;
    bool ghtv : 1;
    bool : 1;

    bool dpad_center : 1;
    uint8_t : 3;

    //      0
    //   7     1
    // 6    15   2
    //   5     3
    //      4
    uint8_t dpad;

    uint8_t unused1;
    uint8_t strumBar;
    uint8_t tiltActive;
    uint8_t whammy;

    uint8_t unused3[12];

    // Reminder that these values are 10-bit in range
    int16_t tilt;
    int16_t tiltActive2;

    int16_t unused4[2];
}
```

## Output Command Info

### Command ID `0x22`: PS3-Style Output Reports

This command is used to send similar output reports to what is seen on PS3 controllers.

Length: 8

Bytes:

- Byte 0: Sub-command
- Bytes 1-7: Command data

```cpp
struct GipGhlGuitarGenericCommand
{
    uint8_t subCommand;
    uint8_t data[7];
}
```

#### Sub-command ID `0x01`: LED control

Data bytes:

- Byte 0: `0x08`
- Byte 1: Player LED bitmask
  - Bit 0: LED 1
  - Bit 1: LED 2
  - Bit 2: LED 3
  - Bit 3: LED 4
- Bytes 2-6: Padding

```cpp
struct GipGhlGuitarPlayerLeds
{
    uint8_t subCommand = 0x01;
    uint8_t unk1 = 0x08;
    bool player1 : 1;
    bool player2 : 1;
    bool player3 : 1;
    bool player4 : 1;
    uint8_t : 4;
    uint8_t padding[5];
}
```

#### Sub-command ID `0x02`: Keep-Alive Packet

This command must be sent every 8 seconds for inputs to work correctly.

Bytes: `0x08, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00`

```cpp
struct GipGhlGuitarKeepAlive
{
    uint8_t subCommand = 0x02;
    uint8_t data[7] = { 0x08, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00 };
}
```

## References

- https://github.com/paroj/xpad
- Packet captures provided by [dynamix1337](https://github.com/dynamix1337)