# Santroller Devices

Santroller Devices are devices programmed using the newest version of the [Santroller Configuration Tool](https://github.com/sanjay900/guitar-configurator). This is a tool written by [sanjay900](https://github.com/sanjay900) for emulating most instruments.
On a PC, these will use a format similar to a PS3 instrument, or an Xbox 360 instrument on Windows. However, these devices also have some additional features that the standard instruments don't have, along with semantics to some device IDs.

**NOTE:** Since the new version of the configurator is still in development, the details here are subject to change. Things should be finalized when it releases fully.

## Table of Contents

- [Device Info](#device-info)
  - [Device Types](#device-types)
- [Input Information](#input-information)
- [Output Report Info](#output-report-info)
  - [XInput Mode](#xinput-mode)
  - [HID Mode](#hid-mode)
  - [Commands](#commands)
    - [General](#general)
    - [5-Fret Guitars](#5-fret-guitars)
    - [6-Fret Guitars](#6-fret-guitars)
    - [Rock Band Drums](#rock-band-drums)
    - [Guitar Hero Drums](#guitar-hero-drums)
    - [Turntable](#turntable)

## Device Info

- Vendor/product ID:
  - USB:  `1209:2882`
  - Bluetooth:  `1209:2885`
- Device name: `Santroller`
- Manufacturer name: `sanjay900`
- Revision:
  - This value encodes the device type into the highest byte `0xXX00`

When in XInput mode, the information above is encoded into the standard XInput capabilities:

- Left stick X: Vendor ID (`0x1209`)
- Left stick Y: Product ID (`0x2882`)

### Device Types

The device type indicates what kind of device is being emulated.

| Value  | Device Type             |
| :----  | :----------             |
| `0x01` | Gamepad                 |
| `0x02` | Dance Pad               |
| `0x03` | Guitar Hero Guitar      |
| `0x04` | Rock Band Guitar        |
| `0x05` | Guitar Hero Drums       |
| `0x06` | Rock Band Drums         |
| `0x07` | Guitar Hero Live guitar |
| `0x08` | DJ Hero turntable       |
| `0x09` | Stage kit               |

## Input Information

Santroller devices will either emulate XInput instruments, or utilise custom HID reports.

## Output Report Info

Santroller devices support a number of output commands for setting the state of some LEDs, using a format based on the Xbox 360 Stage Kit's output reports. Most/all of the Stage Kit commands are supported, along with a number of additional commands for more specific events.

### XInput Mode

In XInput mode, output reports are handled the same way as the Stage Kit, using XInput rumble data to send commands.

```cpp
struct XInputSantrollerCommand
{
    uint16_t parameter;
    uint16_t commandId;

    // Helper constructor for using byte values
    XInputSantrollerCommand(uint8_t command, uint8_t param)
        : commandId(command << 8)
        : parameter(param << 8)
    {
    }
}
```

Just like with the Stage Kit, command IDs and parameters are listed in byte form. When using XInput to send commands, these values must have an additional `0x00` byte appended.

### HID Mode

In HID mode, the following report is used to send commands:

```cpp
struct HidSantrollerCommand
{
    uint8_t reportId = 0x01;

    uint8_t outputType = 0x5A;
    uint8_t parameter;
    uint8_t commandId;
}
```

The padding normally present on other PS3 device commands is not required, and will not cause problems if sent.

### Commands

#### General

In addition to the commands supported by the stage kit, the following general commands are supported:

| Command ID    | Description              | Parameter
| :---------    | :----------              | :--------
| `0x08`        | Star Power gauge fill    | Fill amount: 0 = empty, 255 = full
| `0x09`        | Star Power active        | 1 to enable, 0 to disable
| `0x0A`        | Multiplier number        | The current multiplier number plus 10 (11-255)
| `0x0B`        | Solo section             | 1 to enable, 0 to disable
|               |                          |
| `0x90`-`0xBF` | Device-specific commands |

Some notes:

- Commands will be ignored if both the command ID and parameter value are the same. This is a workaround for DJ Hero on Xbox 360, which sweeps through the full vibration range on the left and right motors, and is the reason why the multiplier number command specifies "plus 10" for the value.
- Some device-specific commands allow manual triggering of lights that are normally triggered by pressing inputs. These commands are meant for things like bot playback, and should not be used during normal gameplay.

#### 5-Fret Guitars

| Command ID | Description | Parameter
| :--------- | :---------- | :--------
| `0x90`     | Open note   | 1 to enable, 0 to disable
| `0x91`     | Green note  | 1 to enable, 0 to disable
| `0x92`     | Red note    | 1 to enable, 0 to disable
| `0x93`     | Yellow note | 1 to enable, 0 to disable
| `0x94`     | Blue note   | 1 to enable, 0 to disable
| `0x95`     | Orange note | 1 to enable, 0 to disable

#### 6-Fret Guitars

| Command ID | Description  | Parameter
| :--------- | :----------  | :--------
| `0x90`     | Open note    | 1 to enable, 0 to disable
| `0x91`     | Black 1 note | 1 to enable, 0 to disable
| `0x92`     | Black 2 note | 1 to enable, 0 to disable
| `0x93`     | Black 3 note | 1 to enable, 0 to disable
| `0x94`     | White 1 note | 1 to enable, 0 to disable
| `0x95`     | White 2 note | 1 to enable, 0 to disable
| `0x96`     | White 3 note | 1 to enable, 0 to disable

#### Rock Band Drums

| Command ID | Description   | Parameter
| :--------- | :----------   | :--------
| `0x90`     | Kick pedal    | 1 to enable, 0 to disable
| `0x91`     | Red pad       | 1 to enable, 0 to disable
| `0x92`     | Yellow pad    | 1 to enable, 0 to disable
| `0x93`     | Blue pad      | 1 to enable, 0 to disable
| `0x94`     | Green pad     | 1 to enable, 0 to disable
| `0x95`     | Yellow cymbal | 1 to enable, 0 to disable
| `0x96`     | Blue cymbal   | 1 to enable, 0 to disable
| `0x97`     | Green cymbal  | 1 to enable, 0 to disable

#### Guitar Hero Drums

| Command ID | Description   | Parameter
| :--------- | :----------   | :--------
| `0x90`     | Kick pedal    | 1 to enable, 0 to disable
| `0x91`     | Red pad       | 1 to enable, 0 to disable
| `0x92`     | Yellow cymbal | 1 to enable, 0 to disable
| `0x93`     | Blue pad      | 1 to enable, 0 to disable
| `0x94`     | Orange cymbal | 1 to enable, 0 to disable
| `0x95`     | Green pad     | 1 to enable, 0 to disable

#### Turntable

| Command ID | Description              | Parameter
| :--------- | :----------              | :--------
| `0x90`     | Scratch (left table)     | 1 to enable, 0 to disable
| `0x91`     | Green note (left table)  | 1 to enable, 0 to disable
| `0x92`     | Red note (left table)    | 1 to enable, 0 to disable
| `0x93`     | Blue note (left table)   | 1 to enable, 0 to disable
|            |                          |
| `0x98`     | Scratch (right table)    | 1 to enable, 0 to disable
| `0x99`     | Green note (right table) | 1 to enable, 0 to disable
| `0x9A`     | Red note (right table)   | 1 to enable, 0 to disable
| `0x9B`     | Blue note (right table)  | 1 to enable, 0 to disable
|            |                          |
| `0xA0`     | Euphoria LED brightness  | Brightness: 0 = off, 255 = max
