# Raphnet Wii Adapter

## Device Info

- Vendor/product ID:
  - `289B:0028` - 1-player WUSBMote v2.0
  - `289B:0029` - 2-player WUSBMote v2.0
  - `289B:002A` - 1-player WUSBMote v2.0 (mouse mode)
  - `289B:002B` - 1-player WUSBMote v2.1
  - `289B:002C` - 2-player WUSBMote v2.1
  - `289B:0080` - 1-player WUSBMote v2.2
  - `289B:0081` - 2-player WUSBMote v2.2
  - `289B:0082` - WUSBMote v2.2 (mouse mode)
  - `289B:0083` - WUSBMote v2.2 (tablet mode)
- Revision: `0x0220`
- Device name: `1-player WUSBMote v2.2`
- Manufacturer name: `raphnet technologies`

## Input Info

The Raphnet adapter supports multiple devices under a shared report format.

Some notes:

- Axis values range from 0 to 32000. 0 is left/up, 32000 is right/down. 
- If D-pad to Axis is enabled in the Raphnet adapter manager, then the D-pad values are mapped to the left joystick.

TODO: how to determine what type of controller is connected?

### Classic Controller

```cpp
struct RaphnetGamepadState
{
    uint8_t reportId;

    uint16le_t leftStickX;
    uint16le_t leftStickY;
    uint16le_t rightStickX;
    uint16le_t rightStickY;
    uint16le_t leftTrigger;
    uint16le_t rightTrigger;

    bool y : 1;
    bool b : 1;
    bool select : 1;
    bool start : 1;

    bool a : 1;
    bool x : 1;
    bool leftTriggerPress : 1;
    bool rightTriggerPress : 1;

    bool leftShoulder : 1;
    bool rightShoulder : 1;
    bool home : 1;
    bool : 1;

    bool up : 1;
    bool down : 1;
    bool left : 1;
    bool right : 1;
} __attribute__((packed));
```

### Guitar

```cpp
struct RaphnetFiveFretGuitarState
{
    uint8_t reportId;

    uint16le_t joyX;
    uint16le_t joyY;
    uint16le_t unused;
    uint16le_t slider;
    uint16le_t unused2;
    uint16le_t whammy;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;

    bool orange : 1;
    bool up : 1;
    bool plus : 1;
    bool minus : 1;

    bool down : 1;
    uint8_t : 7;
} __attribute__((packed));
```

The whammy range seems to be miscalibrated, as it starts at around 12000 when unpressed, and goes to 30000 when pressed. Calibration will be required for it to be read correctly.

### Drum Kit

```cpp
struct RaphnetFiveLaneDrumsState
{
    uint8_t reportId;

    uint16le_t joyX;
    uint16le_t joyY;
    uint16le_t unused[4];

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;

    bool orange : 1;
    bool plus : 1;
    bool minus : 1;
    bool : 1;

    uint8_t unused2;
} __attribute__((packed));
```
