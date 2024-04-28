# Xbox 360 Controller Info Template

This is a template and guide for creating Xbox 360 controller data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

Resources to assist in the documentation process:

- TODO: XInput test app 

## Controller Info

- XInput type: Name (Number)
- XInput subtype: Name (Number)
  - If the subtype is not part of standard XInput subtypes, instead use "Number, not part of XInput standards" (replace `Number` with the subtype number).
- Stick hardware IDs:
  - Some controllers report additional hardware IDs through the stick capabilities.
  - Vendor ID (left stick X): `<Hex value>`
  - Product ID (left stick Y): `<Hex value>`
  - Revision (right stick X): `<Hex value>`

XInput type and subtype may be retrieved through the `XInputGetCapabilities` function of XInput.

## Input Info

Detail input information here.

### XUSB-Only Info

Some input data is not present in the normal XInput report and instead requires lower-level access to the raw report that the device sends. Xbox 360 controllers all send 6 bytes at the end of their report which normally go unused, but some less conventional controllers such as some of the instruments make use of this data.

Accessing this data usually requires raw USB access or interfacing directly with a driver, see the [Other Options](../Controller%20Communication%20Basics/Xbox%20360.md#other-options) section of the Xbox 360 communication doc for more info.

## Vibration Info

Detail any information about what vibration reports will do, if applicable.

### As A Struct

```cpp
// Remove the comments here when done, as these are for guidance purposes only.
// Replace <type> here with a name for the device type. This struct can be usable in place of the regular XINPUT_GAMEPAD struct.
// You may add your own comments wherever needed to explain quirks in the inputs.
struct XInput<type>Gamepad
{
    // Rename and/or redefine the data members here to more closely match the reported data from the controller.
    // The data for XInput devices is always in little-endian order (to my knowledge at least), so keep that in mind when redefining.
    // Ensure that the struct remains the same size in bytes as the original struct.
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
    bool : 1;

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

    // Any XUSB-only data should be wrapped in an `#ifdef USING_XUSB` block like so:
#ifdef USING_XUSB
    uint8_t reserved[6];
#endif
} __attribute__((__packed__));
```

## References

List any external references you used here.
