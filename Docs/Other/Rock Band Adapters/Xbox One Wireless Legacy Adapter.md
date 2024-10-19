# Xbox One Rock Band 4 MadCatz Wireless Legacy Adapter

The MadCatz wireless legacy adapter allows using wireless Xbox 360 instruments on Xbox One for use in Rock Band 4. This device seems like it might have less broad controller support, as it actually translates the input data instead of passing it in raw, however no observations have been made about what it does with other controllers yet.

With regards to PC use, this may also seem pointless to document, as there's an official receiver for connecting wireless controllers, but it's still handy in the case someone has the wireless legacy adapter and not an Xbox 360 receiver. It could also be used for some more exotic scenarios, such as using more than 4 wireless controllers at once alongside a regular receiver.

## Device Info

- Vendor/product ID: `0738:4164`
- Interface GUIDs:
  - `AF259D0F-76B0-4CDB-BFD1-CEA8C0A8F5EE` (Primary)
  - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
  - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
- Class strings:
  - `MadCatz.Xbox.Module.Brangus` (Primary)
  - `Windows.Xbox.Input.NavigationController`

## Input Command Info

### Command ID `0x20`: Input State

Length: Typically 14 bytes (listed max: 50 bytes)

- Bytes 0-1: 16-bit navigation button bitmask
  - These buttons are provided for compatibility with the Navigation Controller interface to allow navigation in the Xbox One menus.
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
- Byte 2: User index
- Byte 3: Device type
  - Same as in the device information message.
- Bytes 4-13: Instrument state
  - This seems to match the state layout of the respective Xbox One instruments, not the layout of Xbox 360 controllers.

```cpp
struct GipLegacyWirelessState
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

    uint8_t userIndex;
    GipLegacyWirelessDeviceType deviceType;

    union {
        GipGuitarState guitar;
        GipDrumkitState drums;
        uint8_t raw[10];
    };
} __attribute__((packed));
```

### Command ID `0x22`: Device Information

Max length: 254

- Byte 0: User index
- Byte 1: Device type
  - Guitar: `0x01`
  - Drums: `0x02`
- Bytes 2-3: Vendor ID (big-endian)
- Byte 4: Unknown (`0x00`)
- Byte 5: XInput subtype (mask with `0x7F`)
  - Guitar: `0x87`
  - Drums: `0x88`
- Byte 6 and onward: Little-endian wide-character name string
  - Guitar: `0x0067, 0x0075, 0x0069, 0x0074, 0x0061, 0x0072` (`L"guitar"`)
  - Drums: `0x0064, 0x0072, 0x0075, 0x006D, 0x0073` (`L"drums"`)

```cpp
enum class GipLegacyWirelessDeviceType : uint8_t
{
    Guitar = 0x01,
    Drums = 0x02,
};

struct GipLegacyWirelessDeviceInfo
{
    uint8_t userIndex;
    GipLegacyWirelessDeviceType deviceType;
    uint16be_t vendorID;
    uint8_t unk;
    uint8_t xinputSubtype;
    wchar_t[] name; // max length is 124
} __attribute__((packed));
```

### Command ID `0x23`: Disconnection

Length: 1 byte

- Byte 0: Disconnected user index

```cpp
struct GipLegacyWirelessDisconnection
{
    uint8_t userIndex;
} __attribute__((packed));
```

## Output Command Info

### Command ID `0x21`: Set State?

This one is unknown, needs more research done.

Length: 2 bytes

- Byte 0: User index
- Byte 1: Unknown

```cpp
struct GipLegacyWirelessSetState
{
    uint8_t userIndex;
    uint8_t unknown;
} __attribute__((packed));
```

### Command ID `0x24`: Request Device Info

Length: 0 bytes

This request is sent to retrieve information about the connected device. The info is returned under command ID `0x22`.

## References

- https://rb4.app/js/app.js
