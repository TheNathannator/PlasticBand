# Xbox One Controller Info Template

This is a template and guide for creating Xbox One controller data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

Resources to assist in the documentation process:

- The [GIP interface writeup](https://gist.github.com/TheNathannator/bcebc77e653f71e77634144940871596) has documentation on common messages that can be received or sent, along with references to other sources of information on the protocol.
- There's currently no programs to assist. TODO! 

## Controller Info

- Vendor ID: Hex number
  - Acquired from the arrival message (`0x02`)
- Product ID: Hex number
  - Acquired from the arrival message (`0x02`)
- Interface GUID:
  - Acquired from the descriptor message (`0x04`)
  - Multiple can be included in the descriptor, only list the one(s) unique to the type of device being documented
- Class string:
  - Acquired from the descriptor message (`0x04`)
  - Multiple can be included in the descriptor, only list the one(s) unique to the type of device being documented

## Input Command Info

Detail information about commands that may be received from the device, excluding common core ones such as status messages (command ID `0x03`) unless there's something important of note about them.

Typically, devices will report input data via the `0x20` command ID.

It is recommended to create a struct out of the described data. For example, here's the standard gamepad's state as a struct:

```c
struct GipGamepadState
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

It's recommended to create a struct for this data as well.
