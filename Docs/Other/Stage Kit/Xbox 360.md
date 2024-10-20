# Xbox 360 Rock Band Stage Kit

The Rock Band Stage Kit is a controller that emulates certain effects that may be used during a live stage performance. It contains a panel of LEDs and a strobe light, and can optionally connect to a fog machine.

## Controller Info

- XInput type: Gamepad (1)
- XInput subtype: 9, not part of XInput standards

## Input Info

This device uses the standard Xbox 360 report format, [documented here](../../Base%20Reports/Xbox%20360.md). Refer to that document for the base layout.

The stage kit has no special inputs. It has face buttons, a d-pad, start, select, and guide.

## Output Info

The vibration control is where the stage kit's specialty lies. It uses vibration info as a protocol to control the various parts of the kit. The right motor value is a sort of command ID, and the left motor value is the parameter to that report. This is the nomenclature that will be used.

Be aware that although XInput exposes the motor values as a 16-bit unsigned integer, only the top byte ends up getting sent to the controller. For simplicity, motor values will be listed in byte form, and an XInput implementation will need to append an additional `0x00` byte to these values (such as by left-shifting by 8).

### Commands

The following table lists each available command ID, along with what they do and what parameter they take, if any.

| Command ID | Description          | Parameter
| :--------- | :----------          | :--------
| `0x01`     | Fog machine on       | None
| `0x02`     | Fog machine off      | None
|            |                      |
| `0x03`     | Strobe light slow    | None
| `0x04`     | Strobe light medium  | None
| `0x05`     | Strobe light fast    | None
| `0x06`     | Strobe light fastest | None
| `0x07`     | Strobe light off     | None
|            |                      |
| `0x20`     | Set blue LEDs        | Bitmask for which of the 8 LEDs to set
| `0x40`     | Set green LEDs       | Bitmask for which of the 8 LEDs to set
| `0x60`     | Set yellow LEDs      | Bitmask for which of the 8 LEDs to set
| `0x80`     | Set red LEDs         | Bitmask for which of the 8 LEDs to set
|            |                      |
| `0xFF`     | Reset everything     | None

### As A Struct

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

struct XInputStageKitCommand
{
    uint16le_t parameter;
    uint16le_t commandId;

    // Helper constructor for using byte values
    XInputStageKitCommand(uint8_t command, uint8_t param)
        : commandId(command << 8)
        : parameter(param << 8)
    {
    }
} __attribute__((packed));
```

## References

- https://github.com/RBTools/Stage-Kit-Lightworks
- Subtype sourced by [sanjay900](https://github.com/sanjay900)
