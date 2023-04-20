# Santroller Devices

Santroller Devices are devices programmed using the newest version of the [Santroller Configuration Tool](https://github.com/sanjay900/guitar-configurator). This is a tool written by [sanjay900](https://github.com/sanjay900) for emulating most instruments. On a PC, these will use a format similar to a PS3 Instrument, or an Xbox 360 instrument on Windows. However, these devices also have some additional features that the standard instruments dont have, along with some semantics to their revision number.

## Device Info

- Vendor/product ID: `1209:2882`
- Device name: `Santroller`
- Manufacturer name: `sanjay900`
- Revision:
  - This value is used to encode certain properties of the device, such as device and console types. It makes use of the standard binary-coded-decimal format already used for revision numbers, `0xMMmr`, where `MM` is the major number, `m` is the minor, and `r` is the revision.
  - Major is used for the device type, minor for the rhythm (i.e. alternate) type, and revision for the console type.

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
| `0x3` | Stage kit                  |
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

- The PS3 console type in gamepad mode will use a report format similar to the rhythm devices rather than replicating the one used by an actual DualShock 3, due to quirks with emulating PS3 controllers.
- The Windows/Xbox One detection type is used if the controller is trying to detect whether it is plugged into a Windows PC or an Xbox One.

## Notable PS3 Mode Input Report Differences

These devices use a report ID unlike the original PS3 instruments, as this was necessary for PS4 console detection. Tilt information on guitars is also copied to left stick X, as this makes it easier to use in games with generic controller support.

These differences do not apply to XInput mode.

## Output Report Info

Santroller devices support a number of output commands for setting the state of some LEDs, using a format based on the Xbox 360 Stage Kit's output reports. Most/all of the Stage Kit commands are supported, along with a number of additional commands for more specific events.

### XInput Mode

In XInput mode, output reports are handled the same way as the Stage Kit, using XInput rumble state to send commands.

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

### HID Mode

In HID mode, the following report is used to send commands:

```cpp
struct HidSantrollerCommand
{
    uint8_t reportId = 0x01;

    uint8_t outputType = 0x5A;
    uint8_t unk1 = 0x08;
    uint8_t parameter;
    uint8_t commandId;
    uint8_t padding[4];
}
```

### Commands

In addition to the command IDs supported by the stage kit, the following command IDs are supported:

| Command ID | Description                 | Parameter
| :--------- | :----------                 | :--------
| `0x0A`     | Multiplier number           | The current multiplier number
| `0x0B`     | Solo section                | 1 to enable, 0 to disable
|            |                             |
| `0x0C`     | Open note                   | 1 to enable, 0 to disable
| `0x0D`     | Green note                  | 1 to enable, 0 to disable
| `0x0E`     | Red note                    | 1 to enable, 0 to disable
| `0x0F`     | Yellow note                 | 1 to enable, 0 to disable
| `0x10`     | Blue note                   | 1 to enable, 0 to disable
| `0x11`     | Orange note                 | 1 to enable, 0 to disable
|            |                             |
| `0x12`     | Yellow cymbal               | 1 to enable, 0 to disable
| `0x13`     | Blue cymbal                 | 1 to enable, 0 to disable
| `0x14`     | Green cymbal                | 1 to enable, 0 to disable
| `0x15`     | Kick pedal                  | 1 to enable, 0 to disable
|            |                             |
| `0x16`     | Star Power gauge (inactive) | Fill amount: 0 = empty, 255 = full
| `0x17`     | Star Power gauge (active)   | Fill amount: 0 = empty, 255 = full

Note that the commands for specific notes (`0x0C`-`0x15`) are meant for things like bot playback; the devices themselves will already handle lighting up LEDs for pressed inputs and this does not need to be handled during actual gameplay.

#### Turntable-Specific

Santroller turntables have a dedicated command for setting the brightness of the euphoria button's LED:

| Command ID | Description             | Parameter
| :--------- | :----------             | :--------
| `0x18`     | Euphoria LED brightness | Brightness: 0 = off, 255 = max

This command is not required for HID mode, the report used with the PS3 turntable will work as well. It *is* required for XInput mode, as the way normal XInput turntables set the euphoria LED is incompatible with the command system used here.
