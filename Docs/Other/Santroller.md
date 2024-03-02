# Santroller Devices

Santroller devices are devices programmed using the the [Santroller Configuration Tool](https://github.com/Santroller/Santroller), which is a tool written by [sanjay900](https://github.com/sanjay900) for emulating instrument controllers using an Arduino/Raspberry Pi microcontroller. While emulation is the primary goal of the project, there are also additional features it supports which games can use if desired.

**NOTE:** The Santroller firmware is still in active development, and some things have yet to be finalized. Details such as report formats and LED output reports are subject to change. Supporting many different consoles is a rather finnicky process, and unexpected side-effects can occur that necessitate a change in format.

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

HID mode uses standard HID/USB device information for identification:

- Vendor/product ID:
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Device name: `Santroller`
- Manufacturer name: `sanjay900`
- Revision:
  - This value encodes the device type into the highest byte (`0xXX00`). See the [Device Types](#device-types) table below for more info.

XInput mode encodes some of the information above into the standard XInput capabilities:

- Left stick X: Vendor ID (`0x1209`)
- Left stick Y: Product ID (`0x2882`)
- Right stick X: Revision (used to better identify controller type)

### Device Types

The device type indicates what kind of device is being emulated.

| Value  | Device Type             |
| :----  | :----------             |
| `0x01` | Gamepad                 |
| `0x02` | Dance pad               |
| `0x03` | Guitar Hero guitar      |
| `0x04` | Rock Band guitar        |
| `0x05` | Guitar Hero drums       |
| `0x06` | Rock Band drums         |
| `0x07` | Guitar Hero Live guitar |
| `0x08` | DJ Hero turntable       |
| `0x09` | Rock Band stage kit     |

## Input Information

On PC, Santroller devices will either emulate XInput instruments or utilize a custom HID mode:

- XInput mode is identical to the instrument being emulated (aside from the capability information mentioned above).
- HID mode has its own custom report format per-device. Each device type has its own document alongside the other instrument documents.

On consoles, Santroller devices will automatically emulate the corresponding device for that platform.

## Output Report Info

Santroller devices support a number of output commands for setting the state of LEDs and the like, using a format based on the Xbox 360 stage kit's output reports. All of the stage kit commands are supported, along with a number of additional commands for more specific events.

### XInput Mode

In XInput mode, output reports are handled the same way as the stage kit, using XInput rumble data to send commands.

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
} __attribute__((__packed__));
```

Just like with the stage kit, command IDs and parameters are listed in byte form. When using XInput to send commands, these values must have an additional `0x00` byte appended.

### HID Mode

In HID mode, the following report is used to send commands:

```cpp
struct HidSantrollerCommand
{
    uint8_t reportId = 0x01;

    uint8_t outputType = 0x5A;
    uint8_t parameter;
    uint8_t commandId;
} __attribute__((__packed__));
```

The padding normally present on other PS3 device commands is not required, and will not cause problems if sent.

### Commands

#### General

In addition to the commands supported by the stage kit, the following general commands are supported:

| Command ID    | Description              | Parameter
| :---------    | :----------              | :--------
| `0x08`        | Star Power gauge fill    | Fill amount: 0 = empty, 255 = full
| `0x09`        | Star Power active        | 1 to enable, 0 to disable
| `0x0A`        | Multiplier number        | The current multiplier number, plus `0x0A` (11-255)
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
