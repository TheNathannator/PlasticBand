# Xbox 360 Rock Band Stage Kit

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: 9, not part of XInput standards

## Input Info

The stage kit has no special inputs. It has face buttons, a d-pad, start, select, and guide.

## Output Info

The vibration control is where the stage kit's specialty lies. It uses vibration info as a protocol to control the various parts of the kit. The right motor value is a sort of report ID, and the left motor value is the data of that report. This is the nomenclature that will be used.

Be aware that although XInput exposes the motor values as a 16-bit unsigned integer, it internally turns this value into a byte, so only the top 8 bits are really sent to the device. For simplicity, motor values will be listed in byte form, and an XInput implementation will need to pad on an additional `0x00` byte to these values.

### Fog Machine

The fog machine is enabled and disabled with the following reports. They don't have any additional data, and should be sent with the left motor set to 0.

- ID `0x01`: Fog machine on
- ID `0x02`: Fog machine off

### Strobe Light

The strobe light is controlled using the following reports. They don't have any additional data, and should be sent with the left motor set to 0.

- ID `0x03`: Strobe light slow
- ID `0x04`: Strobe light medium
- ID `0x05`: Strobe light fast
- ID `0x06`: Strobe light fastest
- ID `0x07`: Strobe light off

### LED Display

The stage kit has a set of LEDs that can be toggled. There are 4 sets of 8 LEDs, each colored red, yellow, blue, and green. The reports for these use the left motor as a bitmask of which LEDs should be enabled or disabled. The least-order bit (bit 0) is the first LED, the highest-order bit (bit 7) is the 8th LED.

- ID `0x20`: Blue LEDs
- ID `0x40`: Green LEDs
- ID `0x60`: Yellow LEDs
- ID `0x80`: Red LEDs

### Miscellaneous

- ID `0x00`: Neutral state
- ID `0xFF`: Disable everything (no left motor data)

### Report ID Enum

```cpp
enum XInputStageKitReportId
{
    FogOn = 0x01,
    FogOff = 0x02,

    StrobeSlow = 0x03,
    StrobeMedium = 0x04,
    StrobeFast = 0x05,
    StrobeFastest = 0x06,
    StrobeOff = 0x07,

    BlueLeds = 0x20,
    GreenLeds = 0x40,
    YellowLeds = 0x60,
    RedLeds = 0x80,

    DisableAll = 0xFF
};
```

## References

- https://github.com/RBTools/Stage-Kit-Lightworks
- Subtype sourced by [sanjay900](https://github.com/sanjay900)
