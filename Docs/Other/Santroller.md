# Santroller Devices

Santroller Devices are devices programmed using the newest version of the [Santroller Configuration Tool](https://github.com/sanjay900/guitar-configurator). This is a tool written by [sanjay900](https://github.com/sanjay900) for emulating most instruments. On a PC, these will use a format similar to a PS3 Instrument, or an Xbox 360 instrument on Windows. However, these devices also have some additional features that the standard instruments dont have, along with some semantics to their revision number.

## Table of Contents

- [Device Info](#device-info)
  - [Device Types](#device-types)
  - [Rhythm Types](#rhythm-types)
  - [Console Types](#console-types)
- [Input Information](#input-information)
  - [HID Mode Differences](#hid-mode-differences)
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
  - Standard: `1209:2882`
  - XInput mode: `1209:2884`
- Device name: `Santroller`
- Manufacturer name: `sanjay900`
- Revision:
  - This value is used to encode certain properties of the device, such as device and console types. It makes use of the standard binary-coded-decimal format already used for revision numbers, `0xMMmr`, where `MM` is the major number, `m` is the minor, and `r` is the revision.
  - Major is used for the device type, minor for the rhythm (i.e. alternate) type, and revision for the console type.

When in XInput mode, the information above is encoded into the standard XInput capabilities:

- Left stick X: Vendor ID (`0x1209`)
- Left stick Y: Product ID (always uses `0x2882`)
- Right stick X: Revision

### Device Types

The device type indicates what kind of device is being emulated.

| Value  | Device Type             |
| :----  | :----------             |
| `0x01` | Gamepad                 |
| `0x02` | Wheel                   |
| `0x03` | Arcade stick            |
| `0x04` | Flight stick            |
| `0x05` | Dance pad               |
| `0x06` | Arcade pad              |
| `0x07` | Guitar                  |
| `0x08` | Guitar Hero Live guitar |
| `0x09` | Drumkit                 |
| `0x0A` | DJ Hero turntable       |
| `0x0B` | Stage kit               |

### Rhythm Types

The rhythm type is used for variants of the same kind of device.

For most devices, this will be 0. For guitars and drumkits, this value is used to distinguish between Rock Band and Guitar Hero devices.
  
| Value | Rhythm Type |
| :---- | :---------- |
| `0x0` | Guitar Hero |
| `0x1` | Rock Band   |

### Console Types

The console type indicates the console/input mode that the device is using.

For USB devices this will always be 0 (universal). For Bluetooth devices, this can change if the user has plugged the receiver into a console or held down a console mode binding.

| Value | Console Type               |
| :---- | :-----------               |
| `0x0` | Universal                  |
| `0x1` | Keyboard or mouse          |
| `0x2` | MIDI                       |
| `0x4` | Xbox 360                   |
| `0x5` | PS3 (when used on PC)      |
| `0x6` | Wii (Rock Band)            |
| `0x7` | Switch                     |
| `0x8` | PS4                        |
| `0x9` | Xbox One                   |
| `0xA` | PS3 (when used on console) |
| `0xB` | Windows/Xbox One detection |
| `0xC` | Windows in XInput mode     |

Some specific notes:

- The gamepad device type in PS3 console mode will use a report format similar to the rhythm devices rather than replicating the one used by an actual DualShock 3, due to quirks with emulating PS3 controllers.
- The Windows/Xbox One detection type is used if the controller is trying to detect whether it is plugged into a Windows PC or an Xbox One.

## Input Information

Input reports sent by a device match those of the standard devices being emulated.

### HID Mode Differences

These devices use a report ID unlike the original PS3 instruments, as this was necessary for PS4 console detection. Tilt information on guitars is also copied to left stick X, as this makes it easier to use in games with generic controller support.

These differences do not apply to XInput mode.

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
| `0x90`     | Green note (both tables) | 1 to enable, 0 to disable
| `0x91`     | Red note (both tables)   | 1 to enable, 0 to disable
| `0x92`     | Blue note (both tables)  | 1 to enable, 0 to disable
|            |                          |
| `0x93`     | Green note (left table)  | 1 to enable, 0 to disable
| `0x94`     | Red note (left table)    | 1 to enable, 0 to disable
| `0x95`     | Blue note (left table)   | 1 to enable, 0 to disable
|            |                          |
| `0x96`     | Green note (right table) | 1 to enable, 0 to disable
| `0x97`     | Red note (right table)   | 1 to enable, 0 to disable
| `0x98`     | Blue note (right table)  | 1 to enable, 0 to disable
|            |                          |
| `0xA0`     | Euphoria LED brightness  | Brightness: 0 = off, 255 = max
