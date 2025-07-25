# PS3 and Wii Rock Band 3 Pro Guitar

## Controller Info

PS3:

- Mustang:
  - Vendor/product ID: `12BA:2430`
  - Revision: `0x0000`
  - Device name: `Harmonix RB3 Mustang Guitar for PlayStation®3`
  - PS3 flags: `0x06`
- Squire:
  - Vendor/product ID: `12BA:2530` (assumed based on patterns with other RB device PIDs)
  - Revision:
  - Device name:
  - PS3 flags:

Wii:

- Vendor/product ID:
  - Mustang: `1BAD:3430`
  - Squire: `1BAD:3530` (assumed based on patterns with other RB device PIDs)
- Revision: `0x0005`
- Device name:

MIDI Pro Adapter information may be found in [its own document](../../Other/Rock%20Band%20Adapters/MIDI%20Pro%20Adapters.md).

## Input Info

This device sends a report modelled after the one [documented here](../../Base%20Reports/PS3.md). Refer to that document for the base layout.

Face buttons and d-pad function as normal.

Fret numbers:

- For simplicity and clarity, some single-byte values are combined here little-endian-wise into a single 16-bit integer.

| String | Input                     | Byte Offset | Bits                     |
| :----- | :----                     | :---------- | :---                     |
| Low E  | Right Thumb X + Y         | 5 + 6       | `0b_xxxx_xxxx_xxx1_1111` |
| A      | Right Thumb X + Y         | 5 + 6       | `0b_xxxx_xx11_111x_xxxx` |
| D      | Right Thumb X + Y         | 5 + 6       | `0b_x111_11xx_xxxx_xxxx` |
| G      | Pressure D-pad Up + Right | 7 + 8       | `0b_xxxx_xxxx_xxx1_1111` |
| B      | Pressure D-pad Up + Right | 7 + 8       | `0b_xxxx_xx11_111x_xxxx` |
| High E | Pressure D-pad Up + Right | 7 + 8       | `0b_x111_11xx_xxxx_xxxx` |

String velocity:

| String | Input               | Byte Offset | Bits           |
| :----- | :----               | :---------- | :---           |
| Low E  | Pressure D-pad Left | 9           | `0b_x111_1111` |
| A      | Pressure D-pad Down | 10          | `0b_x111_1111` |
| D      | Pressure L2         | 11          | `0b_x111_1111` |
| G      | Pressure R2         | 12          | `0b_x111_1111` |
| B      | Pressure L1         | 13          | `0b_x111_1111` |
| High E | Pressure R1         | 14          | `0b_x111_1111` |

Standard 5-fret color flags:

| Fret Color | Input                | Byte Offset | Bits           |
| :--------- | :----                | :---------- | :---           |
| Solo flag  | Pressure D-pad Right | 8           | `0b_1xxx_xxxx` |
| Green      | Pressure D-pad Left  | 9           | `0b_1xxx_xxxx` |
| Red        | Pressure D-pad Down  | 10          | `0b_1xxx_xxxx` |
| Yellow     | Pressure L2          | 11          | `0b_1xxx_xxxx` |
| Blue       | Pressure R2          | 12          | `0b_1xxx_xxxx` |
| Orange     | Pressure L1          | 13          | `0b_1xxx_xxxx` |

Tilt and Auto-Calibration:

- Tilt is duplicated across 3 bytes, 2 of which are used for auto-calibration sensors when enabled. When no sensors are enabled, all 3 of the inputs listed below are tilt.

- Microphone: Pressure Δ (byte offset 15)
  - When enabled, neutral at `0x7F`, and decreases when sound is picked up.
- Light sensor: Pressure ○ (byte offset 16)
  - When enabled, neutral at `0x00`, and increases when light is detected.
- Tilt: Pressure × (byte offset 17)
  - `0x7F` when tilted, `0x40` when not tilted.

Pedal port:

- Pedal press: Pressure □ (byte offset 18),  `0b_1000_0000`
- Pedal connection: Accel X high byte (byte offset 20), `0b_0000_0001`

### As A Struct

```cpp
struct Ps3Report
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool square : 1;
    bool cross : 1;
    bool circle : 1;
    bool triangle : 1;

    bool : 1;
    bool : 1;
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 3;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t unused_leftStick[2];

    // rightStick
    uint8_t lowEFret : 5;
    uint8_t aFret : 5;
    uint8_t dFret : 5;
    bool : 1;
    // pressures[0-1]
    uint8_t gFret : 5;
    uint8_t bFret : 5;
    uint8_t highEFret : 5;
    bool soloFlag : 1;

    // pressures[2]
    uint8_t lowEVelocity : 7;
    bool greenFret: 1;
    // pressures[3]
    uint8_t aVelocity : 7;
    bool redFret: 1;
    // pressures[4]
    uint8_t dVelocity : 7;
    bool yellowFret: 1;
    // pressures[5]
    uint8_t gVelocity : 7;
    bool blueFret: 1;
    // pressures[6]
    uint8_t bVelocity : 7;
    bool orangeFret: 1;
    // pressures[7]
    uint8_t highEVelocity : 7;
    bool : 1;

    // Note: when the auto-cal sensors aren't activated, they just duplicate the tilt axis
    uint8_t autoCal_Microphone; // pressures[8]
    uint8_t autoCal_Light; // pressures[9]
    uint8_t tilt; // pressures[10]

    // pressures[11]
    uint8_t : 7;
    bool pedal : 1;

    // accelX
    uint8_t unused2;
    bool pedalConnection : 1;
    uint8_t : 7;

    uint16le_t unused_accelZ;
    uint16le_t unused_accelY;

    // gyro
    // Unsure what this is, but this is what it's defined as in the spreadsheet linked below.
    // No description is provided for it until more investigation can be done.
    uint8_t counter;
    uint8_t unused4;
} __attribute__((packed));
```

## Feature Reports

### Enable Guitar Data (PS3)

The PS3 Pro Guitars require a feature report to be sent before they will output information for the frets and such:

```cpp
struct PS3ProGuitarEnable
{
    uint8_t reportId = 0x00;

    uint8_t data[40] = {
        0xE9, 0x00, 0x89, 0x1B, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    };
} __attribute__((packed));
```

The Wii Pro Guitars do not require this report, they send the data regardless.

**TODO:** Is this supposed to be sent as 5 separate reports like the auto-calibration reports are, or does it work either way?

### Auto-Calibration Sensors

Pro Guitars have a couple sensors that are used for an auto-calibration process in Rock Band. These sensors can be enabled by sending 5 reports in sequence:

- Enable microphone:

  ```
  Report 1: {0x00, 0xE9, 0x00, 0x83, 0x1B, 0x00, 0x00, 0x00, 0x02}
  Report 2: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 3: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00}
  Report 4: {0x00, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 5: {0x00, 0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  ```

- Enable light sensor:

  ```
  Report 1: {0x00, 0xE9, 0x00, 0x83, 0x1B, 0x00, 0x00, 0x00, 0x01}
  Report 2: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 3: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00}
  Report 4: {0x00, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 5: {0x00, 0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  ```

- Disable sensors:

  ```
  Report 1: {0x00, 0xE9, 0x00, 0x83, 0x1B, 0x00, 0x00, 0x00, 0x00}
  Report 2: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 3: {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00}
  Report 4: {0x00, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00}
  Report 5: {0x00, 0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
  ```

Optionally, a feature report can be read after the reports are sent. The returned report will be all 0s if a sensor was enabled successfully, and will return something else otherwise (such as if it's a guitar which doesn't support the sensors).

**TODO:**

- Does this work on regular guitars as well, or does that have a different report?
- Is this actually the same for both consoles? sanjay's page for this has different reports for PS3 vs Wii, but the spreadsheet linked below has the same report as Wii on its page for PS3 Pro Guitars

As C/C++ code:

```cpp
#include <hidapi.h>

enum PS3WiiProGuitarAutoCalibrationState
{
    Disabled = 0x00,
    LightSensor = 0x01,
    Microphone = 0x02
};

struct PS3WiiProGuitarAutoCalibration
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    uint8_t unk1[7] = {0xE9, 0x00, 0x83, 0x1B, 0x00, 0x00, 0x00};
    uint8_t sensorState;
} __attribute__((packed));

uint8_t otherReports[4][9] = {
    {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
    {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00},
    {0x00, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00},
    {0x00, 0xE9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
};

void send_report(hid_device *dev, PS3WiiProGuitarAutoCalibration *data)
{
    // Send reports
    hid_send_feature_report(dev, (uint8_t*)data, sizeof(*data));
    for (int i = 0; i < 4; i++)
    {
        hid_send_feature_report(dev, otherReports[i], sizeof(otherReports[i]));
    }

    // Optional: get feature report to get the device's response
    // The report will be all 0s if the sensor was successfully enabled,
    // and will be different once deactivated
    uint8_t in_buf[9];
    hid_get_feature_report(dev, in_buf, sizeof(in_buf));
}
```

## References

- https://jasonharley2o.com/wiki/doku.php?id=rb3mustang
- https://docs.google.com/spreadsheets/d/1Y3QM1tEcf0bGiUTjT7R-3mwEAKrCL0qYoySmk3RLo8c/edit?usp=sharing
- https://github.com/dynamix1337/AutoCalibrationRB
