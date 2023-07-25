# Communicating with PS3 Guitars, Drums, and Similar

The PS4 Guitar Hero and Rock Band peripherals are standard HID devices that all follow a similar report format. This format follows that of regular PS4 gamepads, in particular the [USB version of report ID `0x01`](https://controllers.fandom.com/wiki/Sony_DualShock_4#HID_Report_0x01_Input_USB/Dongle) (even for wireless devices, though those seem to use a longer report length).

## Input Reports

For wired devices, the report goes like this:

```cpp
struct PS4Report
{
    uint8_t reportId = 0x01;

    uint8_t leftStickX;
    uint8_t leftStickY;
    uint8_t rightStickX;
    uint8_t rightStickY;

    // This value is not a bitmask, rather it encodes different possible states as individual numbers.
    // Visual representation:
    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad : 4;
    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

    bool l1 : 1;
    bool r1 : 1;
    bool l2 : 1;
    bool r2 : 1;
    bool share : 1;
    bool options : 1;
    bool l3 : 1;
    bool r3 : 1;

    bool ps : 1;
    bool touchpadPress : 1;
    uint8_t : 6;

    uint8_t l2Axis;
    uint8_t r2Axis;

    uint16_t timestamp;
    uint8_t temperture;

    int16_t angularVelocityX;
    int16_t angularVelocityZ;
    int16_t angularVelocityY;

    int16_t accelerometerX;
    int16_t accelerometerY;
    int16_t accelerometerZ;

    uint8_t[5] extData;

    uint8_t powerLevel : 4; // 0x00-0x0A, or 0x01-0x0B if plugged in
    bool pluggedPowerCable : 1;
    bool pluggedHeadphones : 1;
    bool pluggedMic : 1;
    bool pluggedExt : 1;

    bool unkExt1 : 1;
    bool unkExt2 : 1;
    bool notConnected : 1; // used by DS4 dongle to indicate no controller
    uint8_t unk1 : 5;

    uint8_t unk2;

    uint8_t touchCount;
    struct {
        uint8_t timestamp;
        struct {
            uint8_t index : 7;
            uint8_t notTouching : 1;
            uint16_t fingerX : 12;
            uint16_t fingerY : 12;
        } finger[2];
    } touches[3];

    uint8_t padding[3];
} __attribute__((__packed__));
```

For wireless devices, there are an additional 14 bytes at the end of the report:

```cpp
struct PS4WirelessReport : PS4Report
{
    uint8_t padding2[10];
    uint32_t crc32;
} __attribute__((__packed__));
```

The CRC is calculated using the standard CRC32 algorithm, however you *must* prepend the Bluetooth HID command byte used for input reports (`0xA1`) to the beginning of the buffer for it to calculate correctly (assuming you are just using HID to read the device). For example, if the report starts with `01 80 80 80 80 ...`, the CRC buffer should start with `A1 01 80 80 80 80 ...`.

## References

- https://controllers.fandom.com/wiki/Sony_DualShock_4
- https://www.psdevwiki.com/ps4/DS4-BT
