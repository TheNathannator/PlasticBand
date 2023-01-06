# Xbox 360 Controller Info Template

This is a template and guide for creating Xbox 360 controller data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

Resources to assist in the documentation process:

- TODO: XInput test app 

## Controller Info

- XInput Type: Name (Number)
- XInput Subtype: Name (Number)
  - If the subtype is not part of standard XInput subtypes, instead use "Number, not part of XInput standards" (replace `Number` with the subtype number).

XInput type and subtype may be retrieved through the XInputGetCapabilities function of XInput.

## Input Info

Detail input information here.

## Vibration Info

Detail any information about what vibration reports will do, if applicable.

### As A Struct

```c
// Remove the comments here when done, as these are for guidance purposes only.
// Replace <type> here with a name for the device type. This struct can be usable in place of the regular XINPUT_GAMEPAD struct.
// You may add your own comments wherever needed to explain quirks in the inputs.
struct XInput<type>Gamepad
{
    // Rename and/or redefine the data members here to more closely match the reported data from the controller.
    // The data for XInput devices is always in little-endian order (to my knowledge at least), so keep that in mind when redefining.
    // Ensure that the struct remains the same size in bytes as the original struct, unless there's data beyond standard XInput
    // data that needs to be included for a complete definition of everything.
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
}
```

## References

List any external references you used here.
