# Xbox One Controller Info Template

This is a template and guide for creating Xbox One controller data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

See the [MS-GIPUSB open specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-gipusb/e7c90904-5e21-426e-b9ad-d82adeee0dbc) for documentation on the baseline protocol. Additionally, see the [Tools](../../Tools) folder for tools and resources to assist with research.

## Controller Info

- Vendor/product ID: `<Hex vendor ID>:<Hex product ID>`
- Interface GUIDs:
  - (List each, marking the primary GUID and listing it first)
- Class strings:
  - (List each, marking the primary string and listing it first)

Vendor ID and product ID can be retrieved from the "Hello" message (`0x02`) or from USB descriptor info. Interface GUIDs and class strings are found in the metadata message (`0x04`).

Common non-unique GUIDs include:

- `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7`: Navigation
- `9776FF56-9BFD-4581-AD45-B645BBA526D6`: Unknown

Common non-unique class strings include:

- `Windows.Xbox.Input.NavigationController`

## Inbound Message Info

Detail information about messagess that may be received from the device, excluding system messages (unless there's something important of note about them).

### Example: Message ID `0x20`: Input State

Typically, devices will report input data via the `0x20` message ID. As an example, this section details the standard gamepad's state info.

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

    uint16le_t leftTrigger;
    uint16le_t rightTrigger;

    int16le_t leftStickX;
    int16le_t leftStickY;
    int16le_t rightStickX;
    int16le_t rightStickY;
} __attribute__((packed));
```

## Outbound Message

Detail info about any messages that can be sent to the device.

### Example: Message ID `0x09`: Set Vibration

This message is found in the standard Xbox One gamepad's metadata block, and is used to set the state of its four vibration motors.

Length: Typically 9 bytes (listed max: 60 bytes)

- Note that the length reported in the metadata block is a *maximum* length. It is possible for messages to be shorter, and there will typically be corresponding data within the report that indicates what length should be expected.

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
};

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
} __attribute__((packed));
```
