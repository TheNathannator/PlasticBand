# Xbox One Rock Band 4 PDP Wired Legacy Adapter

The PDP wired legacy adapter is a bit of an interesting device. It's an adapter for connecting wired Xbox 360 controllers to an Xbox One, and although it was made specifically for Rock Band 4, it doesn't seem like there's anything stopping it from allowing non-Rock Band 360 controllers to be used.

Documenting this device is a bit pointless with regards to PC use since Xbox 360 devices work perfectly regardless, but the packet capture was there and it's not that difficult to understand, so why not lol

## Device Info

- Vendor ID: `0x0EF6`
- Product ID: `0x0175`
- Interface GUIDs:
  - Primary: `FD019937-9816-45E7-8034-CFD6AB3AE6BF`
  - Secondary:
    - `B8F31FE7-7386-40E9-A9F8-2F21263ACFB7` (Navigation)
    - `9776FF56-9BFD-4581-AD45-B645BBA526D6` (Unknown)
- Class strings:
  - Primary: `PDP.Xbox.RBAdapter.LegacyUSB`
  - Secondary: `Windows.Xbox.Input.NavigationController`

## Input Command Info

### Command ID `0x20`: Input State

Length: 23 bytes

Bytes:

- Bytes 0-1: 16-bit navigation button bitmask
  - These buttons are provided for compatibility with the Navigation Controller interface to allow navigation in the Xbox One menus.
  - Byte 0, bit 0 (`0x01`) - Sync button
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
- Byte 2: Unknown
  - In the packet captures, this is `0x08` (subtype?)
- Byte 3: XUSB report ID
  - Typically `0x00`
- Byte 4: XUSB report length
  - Typically `0x14`
  - This seems to include the report ID and the length byte itself, the actual state data after this is only 18 bytes long
- Bytes 5-22: XUSB report
  - Bytes 5-6: Buttons
    - Formatted identically to the `XINPUT_GAMEPAD` struct from XInput.
    - Byte 5, bit 0 (`0x01`) - D-pad Up
    - Byte 5, bit 1 (`0x02`) - D-pad Down
    - Byte 5, bit 2 (`0x04`) - D-pad Left
    - Byte 5, bit 3 (`0x08`) - D-pad Right
    - Byte 5, bit 4 (`0x10`) - Start Button
    - Byte 5, bit 5 (`0x20`) - Back Button
    - Byte 5, bit 6 (`0x40`) - Left Stick Press
    - Byte 5, bit 7 (`0x80`) - Right Stick Press
    - Byte 6, bit 0 (`0x01`) - Left Bumper
    - Byte 6, bit 1 (`0x02`) - Right Bumper
    - Byte 6, bit 2 (`0x04`) - Guide Button
    - Byte 6, bit 3 (`0x08`) - Sync Button
    - Byte 6, bit 4 (`0x10`) - A Button
    - Byte 6, bit 5 (`0x20`) - B Button
    - Byte 6, bit 6 (`0x40`) - X Button
    - Byte 6, bit 7 (`0x80`) - Y Button
  - Byte 7: Left trigger
  - Byte 8: Right trigger
  - Bytes 9-10: Left stick X (little-endian, signed)
  - Bytes 11-12: Left stick Y (little-endian, signed)
  - Bytes 13-14: Right stick X (little-endian, signed)
  - Bytes 15-16: Right stick Y (little-endian, signed)
  - Bytes 17-22: Reserved

```c
#include <xinput.h>

struct GipLegacyWiredState
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

    bool leftShoulder : 1;
    bool rightShoulder : 1;
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    uint8_t unk1;
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
        int16_t leftStickX;
        int16_t leftStickY;
        int16_t rightStickX;
        int16_t rightStickY;

        uint8_t reserved[6];
    } xusbState;
}
```

### Command ID `0x22`: Receive XUSB Device Info

Length: 5 bytes

- Byte 0: XInput subtype
- Byte 1-2: Vendor ID (big-endian)
- Byte 3-4: Product ID (big-endian)

```c
struct GipLegacyWiredDeviceInfo
{
    uint8_t subType;
    uint16be_t vendorId;
    uint16be_t productId;
}
```

### Command ID `0x23`: Unknown

Length: 1 byte

- Byte 0: Unknown
  - The only instance of this in the packet log is set to `0x08`.

```c
typedef GipLegacyWired_0x23 uint8_t;
```

## Output Command Info

### Command ID `0x21`: Set state? 

Reported length: 23 bytes

The data for this one is unknown, it's reported in the descriptor but unfortunately it's not present in the referenced packet captures.

### Command ID `0x24`: Request XUSB Device Info

Length: 0 bytes

This request is sent to retrieve information about the connected XUSB device. The info is returned under command ID `0x22`.

## References

- A [packet capture](https://www.dropbox.com/s/465dln4zr3wn1pa/USB%20captures.zip) found on http://forum.gimx.fr/viewtopic.php?t=2897&start=10
