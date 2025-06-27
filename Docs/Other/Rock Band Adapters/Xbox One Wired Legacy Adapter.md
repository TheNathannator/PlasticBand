# Xbox One Rock Band 4 PDP Wired Legacy Adapter

The PDP wired legacy adapter allows connecting wired Xbox 360 controllers to an Xbox One. Although it was made specifically for Rock Band 4, it doesn't seem like there's anything stopping it from allowing other Xbox 360 controllers to be used, as it just passes in raw input data from the connected controller.

Documenting this is a bit pointless with regards to PC use since Xbox 360 devices are usable without it, but the packet capture was there and it's not that difficult to understand, so why not? lol

## Device Info

- Vendor/product ID: `0E6F:0175`
- Interface GUIDs:
  - `FD019937-9816-45E7-8034-CFD6AB3AE6BF` (Preferred)
  - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
  - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Input device)
- Class strings:
  - `PDP.Xbox.RBAdapter.LegacyUSB` (Preferred)
  - `Windows.Xbox.Input.NavigationController`

## Inbound Message Info

### Message ID `0x20`: Input State

Length: 23 bytes

- Bytes 0-1: 16-bit navigation button bitmask
  - These buttons are provided for compatibility with the Navigation Controller interface to allow navigation in the Xbox One menus.
  - Byte 0, bit 0 (`0x01`) - Unused
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
- Byte 2: Unknown
  - In the packet captures, this is `0x08` (subtype?)
- Byte 3: XUSB report ID
  - Typically `0x00`
- Byte 4: XUSB report length
  - Typically `0x14`
  - This seems to include the report ID and the length byte itself, the actual state data after this is only 18 bytes long (`0x12`)
- Bytes 5-22: XUSB report
  - Formatted identically to the `XINPUT_GAMEPAD` struct from XInput.
  - Bytes 5-6: Buttons
    - Byte 5, bit 0 (`0x01`) - D-pad up
    - Byte 5, bit 1 (`0x02`) - D-pad down
    - Byte 5, bit 2 (`0x04`) - D-pad left
    - Byte 5, bit 3 (`0x08`) - D-pad right
    - Byte 5, bit 4 (`0x10`) - Start button
    - Byte 5, bit 5 (`0x20`) - Back button
    - Byte 5, bit 6 (`0x40`) - Left stick press
    - Byte 5, bit 7 (`0x80`) - Right stick press
    - Byte 6, bit 0 (`0x01`) - Left bumper
    - Byte 6, bit 1 (`0x02`) - Right bumper
    - Byte 6, bit 2 (`0x04`) - Guide button
    - Byte 6, bit 3 (`0x08`) - Sync button
    - Byte 6, bit 4 (`0x10`) - A button
    - Byte 6, bit 5 (`0x20`) - B button
    - Byte 6, bit 6 (`0x40`) - X button
    - Byte 6, bit 7 (`0x80`) - Y button
  - Byte 7: Left trigger
  - Byte 8: Right trigger
  - Bytes 9-10: Left stick X (little-endian, signed)
  - Bytes 11-12: Left stick Y (little-endian, signed)
  - Bytes 13-14: Right stick X (little-endian, signed)
  - Bytes 15-16: Right stick Y (little-endian, signed)
  - Bytes 17-22: Reserved

```cpp
struct GipLegacyWiredState
{
    bool : 1;
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

    uint8_t subType;
    uint8_t reportId;
    uint8_t reportLength;

    struct {
        bool dpadUp : 1;
        bool dpadDown : 1;
        bool dpadLeft : 1;
        bool dpadRight : 1;

        bool start : 1;
        bool back : 1;
        bool leftThumbClick : 1;
        bool rightThumbClick : 1;

        bool leftShoulder : 1;
        bool rightShoulder : 1;
        bool guide : 1;
        bool reserved : 1;

        bool a : 1;
        bool b : 1;
        bool x : 1;
        bool y : 1;

        uint8_t leftTrigger;
        uint8_t rightTrigger;
        int16le_t leftStickX;
        int16le_t leftStickY;
        int16le_t rightStickX;
        int16le_t rightStickY;

        uint8_t reserved[6];
    } xusbState;
} __attribute__((packed));
```

### Message ID `0x22`: Receive XUSB Device Info

Length: 5 bytes

- Byte 0: XInput subtype
- Byte 1-2: Vendor ID (big-endian)
- Byte 3-4: Product ID (big-endian)

```cpp
struct GipLegacyWiredDeviceInfo
{
    uint8_t subType;
    uint16be_t vendorId;
    uint16be_t productId;
} __attribute__((packed));
```

### Message ID `0x23`: Disconnection?

This only occurs once in the packet log at the very end.

Length: 1 byte

- Byte 0: Subtype?
  - The only instance of this in the packet log is set to `0x08`, closest match is the subtype.

```cpp
struct GipLegacyWiredDisconnection
{
    uint8_t subType;
} __attribute__((packed));
```

## Outbound Message

### Message ID `0x21`: Set state? 

Reported length: 23 bytes

The data for this one is unknown, it's reported in the metadata block but unfortunately it's not present in the referenced packet captures.

### Message ID `0x24`: Request Device Info

Length: 0 bytes

This request is sent to retrieve information about the connected device. The info is returned under message ID `0x22`.

## References

- A [packet capture](https://www.dropbox.com/s/465dln4zr3wn1pa/USB%20captures.zip) found on http://forum.gimx.fr/viewtopic.php?t=2897&start=10
