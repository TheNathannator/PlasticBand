# PS3 Base Report Format

The base report format for the PS3 (and Wii Rock Band) instrument peripherals. These devices are all standard HID devices which describe the following format through their HID descriptor.

The HID descriptor for these devices unfortunately does not expose all of the axes with well-defined usage values, and instead throws the pressure and accelerometer axes into the vendor-defined usage page. Manual handling of these values is required if they contain desired state.

Note that the DualShock 3 does not follow this format. Presumably the PS3/4/5 properly makes use of the HID descriptor for its input handling rather than assuming a specific state format.

## Input Reports

All PS3 instruments have no report ID, and only send a single type of report.

The base report goes like this:

```cpp
struct PS3ControllerState
{
    // (The descriptor data annotating this structure was taken from the Guitar Hero World Tour guitar.)

    // 0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
    // 0x09, 0x05,        // Usage (Game Pad)
    // 0xA1, 0x01,        // Collection (Application)

#ifdef WINDOWS
    // Windows always provides a report ID in its HID APIs
    uint8_t reportId = 0x00;
#endif

    // Button bits

    // 0x15, 0x00,        //   Logical Minimum (0)
    // 0x25, 0x01,        //   Logical Maximum (1)
    // 0x35, 0x00,        //   Physical Minimum (0)
    // 0x45, 0x01,        //   Physical Maximum (1)
    // 0x75, 0x01,        //   Report Size (1)
    // 0x95, 0x0D,        //   Report Count (13)
    // 0x05, 0x09,        //   Usage Page (Button)
    // 0x19, 0x01,        //   Usage Minimum (0x01)
    // 0x29, 0x0D,        //   Usage Maximum (0x0D)
    // 0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

    bool l1 : 1;
    bool r1 : 1;
    bool l2 : 1;
    bool r2 : 1;

    bool select : 1;
    bool start : 1;
    bool l3 : 1;
    bool r3 : 1;

    bool ps : 1;
    // Padding
    // 0x95, 0x03,        //   Report Count (3)
    // 0x81, 0x01,        //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    uint8_t : 3;

    // D-pad
    // This value is not a bitmask, rather it encodes different possible states as individual numbers.
    // Visual representation:
    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4

    // 0x05, 0x01,        //   Usage Page (Generic Desktop Ctrls)
    // 0x25, 0x07,        //   Logical Maximum (7)
    // 0x46, 0x3B, 0x01,  //   Physical Maximum (315)
    // 0x75, 0x04,        //   Report Size (4)
    // 0x95, 0x01,        //   Report Count (1)
    // 0x65, 0x14,        //   Unit (System: English Rotation, Length: Centimeter)
    // 0x09, 0x39,        //   Usage (Hat switch)
    // 0x81, 0x42,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,Null State)
    uint8_t dpad : 4;
    // Padding
    // 0x65, 0x00,        //   Unit (None)
    // 0x95, 0x01,        //   Report Count (1)
    // 0x81, 0x01,        //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    uint8_t : 4;

    // Stick axes
    // Neutral state is 0x7F/0x80
    // X axis is left at 0x00, right at 0xFF
    // Y axis is top at 0x00, bottom at 0xFF

    // 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
    // 0x46, 0xFF, 0x00,  //   Physical Maximum (255)
    // 0x09, 0x30,        //   Usage (X)
    // 0x09, 0x31,        //   Usage (Y)
    // 0x09, 0x32,        //   Usage (Z)
    // 0x09, 0x35,        //   Usage (Rz)
    // 0x75, 0x08,        //   Report Size (8)
    // 0x95, 0x04,        //   Report Count (4)
    // 0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;

    // Pressure axes for buttons
    // Neutral state is 0x00, max is 0xFF.

    // 0x06, 0x00, 0xFF,  //   Usage Page (Vendor Defined 0xFF00)
    // 0x09, 0x20,        //   Usage (0x20)  // FF00:0020
    // 0x09, 0x21,        //   Usage (0x21)  // FF00:0021
    // 0x09, 0x22,        //   Usage (0x22)  // FF00:0022
    // 0x09, 0x23,        //   Usage (0x23)  // FF00:0023
    // 0x09, 0x24,        //   Usage (0x24)  // FF00:0024
    // 0x09, 0x25,        //   Usage (0x25)  // FF00:0025
    // 0x09, 0x26,        //   Usage (0x26)  // FF00:0026
    // 0x09, 0x27,        //   Usage (0x27)  // FF00:0027
    // 0x09, 0x28,        //   Usage (0x28)  // FF00:0028
    // 0x09, 0x29,        //   Usage (0x29)  // FF00:0029
    // 0x09, 0x2A,        //   Usage (0x2A)  // FF00:002A
    // 0x09, 0x2B,        //   Usage (0x2B)  // FF00:002B
    // 0x95, 0x0C,        //   Report Count (12)
    // 0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    uint8_t pressure_dpadUp;
    uint8_t pressure_dpadRight;
    uint8_t pressure_dpadLeft;
    uint8_t pressure_dpadDown;
    uint8_t pressure_l2;
    uint8_t pressure_r2;
    uint8_t pressure_l1;
    uint8_t pressure_r1;
    uint8_t pressure_triangle;
    uint8_t pressure_circle;
    uint8_t pressure_cross;
    uint8_t pressure_square;

    // Feature/output reports are included in the same collection as input data
    // 0x0A, 0x21, 0x26,  //   Usage (0x2621)  // FF00:2621
    // 0x95, 0x08,        //   Report Count (8)
    // 0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    // 0x0A, 0x21, 0x26,  //   Usage (0x2621)  // FF00:2621
    // 0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    // Accelerometer axes
    // Each of the following are 10 bits in accuracy.
    // Centered/neutral state is nominally 0x0200, actual values may vary.

    // 0x26, 0xFF, 0x03,  //   Logical Maximum (1023)
    // 0x46, 0xFF, 0x03,  //   Physical Maximum (1023)
    // 0x09, 0x2C,        //   Usage (0x2C)  // FF00:002C
    // 0x09, 0x2D,        //   Usage (0x2D)  // FF00:002D
    // 0x09, 0x2E,        //   Usage (0x2E)  // FF00:002E
    // 0x09, 0x2F,        //   Usage (0x2F)  // FF00:002F
    // 0x75, 0x10,        //   Report Size (16)
    // 0x95, 0x04,        //   Report Count (4)
    // 0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
                       // Guessed based on Guitar Hero guitars and some DS3 notes
    uint16le_t accelX; // Left/right acceleration (roll)
    uint16le_t accelZ; // Forward/back acceleration (pitch)
    uint16le_t accelY; // Up/down acceleration (gravity)
    uint16le_t gyro;   // Left/right instantaneous rotation (yaw)
    // 0xC0,              // End Collection
} __attribute__((packed)); // 27/28 bytes
```

What everything means changes between devices, but the amount of data does not.

## Output Reports

These output reports are ones that all of the PS3 devices documented in this repo support. These do not apply to actual DualShock 3 controllers however, those use a different output report format.

The output reports follow this general format:

```cpp
struct PS3GenericOutputReport
{
    uint8_t reportId = 0x00;

    // (Implicit)         //   Usage Page (Vendor Defined 0xFF00)
    // 0x0A, 0x21, 0x26,  //   Usage (0x2621)  // FF00:2621
    // (Implicit)         //   Report Count (8)
    // 0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    uint8_t outputType; // A secondary ID used to determine the type of request
    uint8_t data[7];    // The data for the request
} __attribute__((packed)); // 8/9 bytes
```

### Output Type `0x01`: Player LEDs

This report is used to set the player LEDs on the device.

```cpp
struct PS3SetPlayerLeds
{
    uint8_t reportId = 0x00;

    uint8_t outputType = 0x01;
    uint8_t unk1 = 0x08; // data length?
    bool player1 : 1; // 0x01
    bool player2 : 1; // 0x02
    bool player3 : 1; // 0x04
    bool player4 : 1; // 0x08
    uint8_t : 4;
    uint8_t padding[5];
} __attribute__((packed));
```

## Feature Report

PS3 controllers have a feature report that can be requested, which contains some data about the controller:

```cpp
struct PS3ControllerCapabilities
{
    uint8_t reportId = 0x00;

    // (Implicit)         //   Usage Page (Vendor Defined 0xFF00)
    // 0x0A, 0x21, 0x26,  //   Usage (0x2621)  // FF00:2621
    // 0x95, 0x08,        //   Report Count (8)
    // 0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    // Corresponds to the HID usage of the feature report.
    // This seems to be used as a form of versioning, PS4 controllers have a report under 0x2721
    // and PS5 controllers have one under 0x2821.
    uint16le_t supportedUsage = 0x2621;

    uint8_t unk3 = 0x01;
    uint8_t flags;
    uint8_t unk4[4];
} __attribute__((packed));
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-controllers.html
- Additional information provided by [sanjay900](https://github.com/sanjay900)
