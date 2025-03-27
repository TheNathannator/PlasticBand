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

## Feature reports

These feature reports are used for configuring the raphnet, and for reading information about the raphnet

To retrieve information, you must send a Request Data packet, and then constantly poll for a response until you get one.

### Commands
```cpp
enum RaphnetCommands {
    GetControllerType = 0x06, // Get the type of controller that is connected
};
```
### Raphnet Controller Types

```cpp
enum RaphnetControllerType {
   RNT_TYPE_NONE_NEW = 100,
   RNT_TYPE_CLASSIC = 101,
   RNT_TYPE_NUNCHUK = 112,
   RNT_TYPE_CLASSIC_PRO = 113,
   RNT_TYPE_WIIMOTE_TAIKO = 114,
   RNT_TYPE_PSX_DIGITAL = 119,
   RNT_TYPE_PSX_ANALOG = 120,
   RNT_TYPE_PSX_NEGCON = 121,
   RNT_TYPE_PSX_MOUSE = 122,
   RNT_TYPE_WII_GUITAR = 127,
   RNT_TYPE_UDRAW_TABLET = 128,
   RNT_TYPE_WII_DRUM = 130
};
```

### Request Data

```cpp
struct RaphnetExchangeRequestReport
{
    uint8_t reportId = 0x00;

    RaphnetCommands command; 
    uint8_t channel; // used for devices with multiple ports, 0 for devices with a single port
} __attribute__((packed)); // 2/3 bytes
```

### Command `0x06` response - Get Controller Type

The following reponse is returned if you request the controller type.

```cpp
struct RaphnetControllerTypeResponseReport
{
    uint8_t reportId = 0x00;

    RaphnetCommands command; 
    uint8_t channel;
    RaphnetControllerType type;
    uint8_t padding[29];
} __attribute__((packed)); // 32/33 bytes, padding isn't strictly necessary
```