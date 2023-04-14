# Xbox One Controller Info Template

This is a template and guide for creating Xbox One controller data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

Resources to assist in the documentation process:

- The [GIP interface writeup](https://gist.github.com/TheNathannator/bcebc77e653f71e77634144940871596) has documentation on common messages that can be received or sent, along with references to other sources of information on the protocol.
- There's currently no programs to assist. TODO! 

## Controller Info

- Vendor/product ID: `<Hex vendor ID>:<Hex product ID>`
- Interface GUIDs:
  - Primary:
  - Secondary:
    - (List each)
- Class strings:
  - Primary:
  - Secondary:
    - (List each)

Vendor ID and product ID can be retrieved from the arrival message (`0x02`).

Interface GUIDs and class strings can be retrieved from the descriptor message (`0x04`). Multiple of each can be specified, if there's more than one include all of them but be sure to specify which is primary (unique) and which ones are secondary (non-unique). If there's only one, this is not required.

Common non-unique GUIDs include:

- `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7`: Navigation
- `9776FF56-9BFD-4581-AD45-B645BBA526D6`: Unknown

Common non-unique class strings include:

- `Windows.Xbox.Input.NavigationController`

## Input Command Info

Detail information about commands that may be received from the device, excluding common core ones such as status messages (command ID `0x03`) unless there's something important of note about them.

### Example: Command ID `0x20`: Input State

Typically, devices will report input data via the `0x20` command ID. As an example, this section details the standard gamepad's state info.

Length: 14 bytes

Although the readme for these docs says that little-endian is assumed, it's recommended to specify big-endian or little-endian for values spanning across multiple bytes. Signedness should also be specified.

- Bytes 0-1: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Sync button
  - Byte 0, bit 1 (`0x02`) - Unused
  - Byte 0, bit 2 (`0x04`) - Menu button
  - Byte 0, bit 3 (`0x08`) - View button
  - Byte 0, bit 4 (`0x10`) - A button
  - Byte 0, bit 5 (`0x20`) - B button
  - Byte 0, bit 6 (`0x40`) - X button
  - Byte 0, bit 7 (`0x80`) - Y button
  - Byte 1, bit 0 (`0x01`) - D-pad up
  - Byte 1, bit 1 (`0x02`) - D-pad down
  - Byte 1, bit 2 (`0x04`) - D-pad left
  - Byte 1, bit 3 (`0x08`) - D-pad right
  - Byte 1, bit 4 (`0x10`) - Left bumper
  - Byte 1, bit 5 (`0x20`) - Right bumper
  - Byte 1, bit 6 (`0x40`) - Left stick press
  - Byte 1, bit 7 (`0x80`) - Right stick press
- Bytes 2-3: Left trigger (little-endian, unsigned)
  - Unpressed at 0, fully pressed at `0x03FF`.
- Bytes 4-5: Right trigger (little-endian, unsigned)
- Bytes 6-7: Left stick X (little-endian, signed)
  - Centered at 0, left at -32768, right at 32767.
- Bytes 8-9: Left stick Y (little-endian, signed)
  - Centered at 0, down at -32768, up at 32767.
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

## Output Command Info

Detail info about any commands that can be sent to the device.

### Example: Command ID `0x09`: Set Vibration

This command is found in the standard Xbox One gamepad's descriptor, and is used to set the state of its four vibration motors.

Length: Typically 9 bytes (listed max: 60 bytes)

- Sometimes the in-practice length of a command differs from what the descriptor reports as its max length.

Bytes:

- Byte 0: Unknown
  - Typically `0x00`
- Byte 1: Flags
  - Bitmask of which motors to set.
  - Bit 0 (`0x01`): Right rumble
  - Bit 1 (`0x02`): Left rumble
  - Bit 2 (`0x04`): Right trigger
  - Bit 3 (`0x08`): Left trigger
- Byte 2: Left trigger motor strength
- Byte 3: Right trigger motor strength
- Byte 4: Left rumble motor strength
- Byte 5: Right rumble motor strength
- Byte 6: Rumble duration
  - Unsure of the exact unit for this value.
  - Typically `0xFF` for instantaneous rumble effects.
- Byte 7: Activation delay
  - Unsure of the exact unit for this value.
  - Typically `0x00` for instantaneous rumble effects.
- Byte 8: Number of times to repeat
  - Typically `0xEB` for instantaneous rumble effects.

```cpp
enum GipGamepadMotorFlags
{
    GipGamepadMotor_RightRumble = 0x01,
    GipGamepadMotor_LeftRumble = 0x02,
    GipGamepadMotor_RightTrigger = 0x04,
    GipGamepadMotor_LeftTrigger = 0x08
}

struct GipGamepadVibration
{
    uint8_t unk;
    uint8_t flags;

    uint8_t leftTrigger;
    uint8_t rightTrigger;
    uint8_t leftRumble;
    uint8_t rightRumble;

    uint8_t duration;
    uint8_t delay;
    uint8_t repeat;
}
```
