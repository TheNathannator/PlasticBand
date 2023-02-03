# Communicating with PS3 Guitars, Drums, and Similar

The PS3 Guitar Hero and Rock Band peripherals are standard HID devices that all follow a similar report format.

Unfortunately, if these devices are anything like a regular PS3 gamepad, the HID descriptor is not formed correctly. Manual parsing of the input report or use of a fixed descriptor is needed.

Vendor ID is typically `0x12BA` but may vary. The IDs for a device are noted in the docs for the device.

## Input Reports

The base report goes like this:

```cpp
struct Ps3Report
{
    uint8_t reportId;

    // Button bits
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
    uint8_t : 3;

    // This value is not a bitmask, rather it encodes different possible states as individual numbers.
    // Visual representation:
    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4 
    uint8_t dpad;

    // Stick axes
    // Neutral state is 0x80
    // X axis is left at 0x00, right at 0xFF
    // Y axis is top at 0x00, bottom at 0xFF
    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;

    // Pressure axes for buttons
    // Neutral state is 0x00, max is 0xFF
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

    // Each of the following are 10 bits in accuracy
    // Centered/neutral state is nominally 0x0200, actual values may vary
    int16_t accelX; // Left/right acceleration (roll)
    int16_t accelZ; // Forward/back acceleration (pitch)
    int16_t accelY; // Up/down acceleration (gravity)
    int16_t gyro;   // Left/right instantaneous rotation (yaw)
};
```

What everything means changes between devices, but the amount of data does not.

## Output Reports

These output reports are ones that all of the PS3 devices documented in this repo support. These do not apply to PS3 gamepad controllers, those use a different output report format.

The output reports follow this general format:

```cpp
struct PS3GenericOutputReport
{
    // The main report ID
    // Might not matter what this is set to, but games on PS3 typically send these with a
    // USB transfer wValue of 0x0201, which would imply an ID of 0x01
    // For safety, a report ID of 0x00 is used
    uint8_t reportId = 0x00;

    // A secondary ID used to determine the type of request
    uint8_t outputType;
    uint8_t data[7];
}
```

### Output Type `0x01`: Player LEDs

```cpp
struct PS3PlayerLeds
{
    uint8_t reportId = 0x00;

    uint8_t outputType = 0x01;
    uint8_t unk1 = 0x08;
    bool player1 : 1;
    bool player2 : 1;
    bool player3 : 1;
    bool player4 : 1;
    uint8_t : 4;
    uint8_t padding[5];
}
```

## Feature Reports

PS3 controllers have a feature report that can be requested, which contains some data about the controller:

```cpp
struct PS3Descriptor
{
    uint8_t reportId = 0x00;

    uint8_t unk1 = 0x21;
    uint8_t unk2 = 0x26;
    uint8_t unk3 = 0x01;
    uint8_t ps3_id;
    uint8_t unknown[4];
}
```

The `ps3_id` field varies between devices, on standard PS3 gamepads it's `0x07`. It could potentially be used as another way of identifying devices.

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-controllers.html
- Additional information provided by [sanjay900](https://github.com/sanjay900)
