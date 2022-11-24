# Communicating with PS3 Guitars, Drums, and Similar

The PS3 Guitar Hero and Rock Band peripherals are standard HID devices that all follow a similar report format.

Unfortunately, if these devices are anything like a regular PS3 gamepad, the HID descriptor is not formed correctly. Manual parsing of the input report or use of a fixed descriptor is needed.

Vendor ID is typically `0x12BA` but may vary, and product ID is never the same between devices. The IDs for a device are noted in the docs for the device.

## Report Format

The base report goes like this:

```cpp
struct Ps3Report
{
    // Button bits
    bool triangle : 1;
    bool circle : 1;
    bool cross : 1;
    bool square : 1;

    bool l2 : 1;
    bool r2 : 1;
    bool l1 : 1;
    bool r1 : 1;

    bool start : 1;
    bool select : 1;
    bool l3 : 1;
    bool r3 : 1;

    bool ps : 1;
    uint8_t padding : 3;

    // This value is not a bitmask, rather it encodes different possible states as individual numbers.
    // Visual representation:
    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4 
    uint8_t dpad;

    // Stick axes
    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;

    // Pressure axes for buttons
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
    int16_t accelX;
    int16_t accelY;
    int16_t accelZ;
    int16_t gyro;
};
```

What everything means changes between devices, but the amount of data typically does not.

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-controllers.html
