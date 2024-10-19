# Xbox 360 Base Layout

The base report format for Xbox 360 devices.

## XInput

```cpp
struct XInputGamepad
{
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
} __attribute__((__packed__));
```

## XUSB

If you're interacting directly with the XUSB driver or otherwise have raw access to the report sent from the device, you can read 6 additional bytes from the input report data. This will be noted in struct definitions using the `USING_XUSB` define.

```cpp
#ifdef USING_XUSB
struct XusbGamepad : XInputGamepad
{
    uint8_t reserved[6];
} __attribute__((__packed__));
#endif
```
