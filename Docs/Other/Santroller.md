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
    - [Alternate Reports](#alternate-reports)
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
    uint16le_t parameter;
    uint16le_t commandId;

    // Helper constructor for using byte values
    XInputSantrollerCommand(uint8_t command, uint8_t param)
        : commandId(command << 8)
        : parameter(param << 8)
    {
    }
} __attribute__((__packed__)); // 4 bytes
```

Just like with the stage kit, command IDs and parameters are listed in byte form. When using XInput to send commands, these values must have an additional `0x00` byte appended.

### HID Mode

In HID mode, a PS3-style output report is used to send commands, using the `0x5A` output type:

```cpp
struct HidSantrollerCommand
{
    uint8_t reportId = 0x01;

    uint8_t outputType = 0x5A;
    uint8_t parameter;
    uint8_t commandId;
} __attribute__((__packed__)); // 4 bytes
```

The padding normally present on other PS3 device commands is not required, and will not cause problems if sent.

#### Alternate Reports

In HID mode, each device also has an alternate output report style using the `0x5B` output type to send several commands at once. The base report is as follows:

```cpp
enum class StageKitStrobe : uint8_t
{
    StrobeSlow = 1,
    StrobeMedium = 2,
    StrobeFast = 3,
    StrobeFastest = 4,
    StrobeOff = 5,
};

struct SantrollerHidCommandEx
{
    uint8_t reportId = 0x01;

    uint8_t outputType = 0x5B;
  
    StageKitStrobe stageKitStrobe : 7;
    bool stageKitFog : 1;

    uint8_t stageKitBlue;
    uint8_t stageKitGreen;
    uint8_t stageKitYellow;
    uint8_t stageKitRed;
  
    uint8_t multiplier;
    uint8_t starPowerState;
  
    bool starPowerActive : 1;
    bool soloActive : 1;
    bool noteMiss : 1;
    uint8_t : 5;
} __attribute__((__packed__)); // 10 bytes
```

This command is necessary for Bluetooth connections, where sending an output report tends to have a lot of overhead and batching data is extremely valuable.

### Commands

#### General

In addition to the commands supported by the stage kit, the following general commands are supported:

| Command ID    | Description              | Parameter
| :---------    | :----------              | :--------
| `0x08`        | Star Power gauge fill    | Fill amount: 0 = empty, 255 = full
| `0x09`        | Star Power active        | 1 to enable, 0 to disable
| `0x0A`        | Multiplier number        | The current multiplier number (1-255, 0 to disable)
| `0x0B`        | Solo section             | 1 to enable, 0 to disable
| `0x0C`        | Note miss                | 1 to enable, 0 to disable
|               |                          |
| `0x90`-`0xBF` | Device-specific commands |

Some notes:

- Most device-specific commands are for triggering lights in response to notes being hit in-game. These are separate from lights triggered directly by inputs, and are not meant for displaying inputs from e.g. replays.

#### 5-Fret Guitars

| Command ID | Description | Parameter
| :--------- | :---------- | :--------
| `0x90`     | Note hits   | `0x01`: Open<br/>`0x02`: Green<br/>`0x04`: Red<br/>`0x08`: Yellow<br/>`0x10`: Blue<br/>`0x20`: Orange

Alternate:

```cpp
struct SantrollerHidFiveFretGuitarOutput : SantrollerHidCommandEx
{
    bool openHit : 1;
    bool greenHit : 1;
    bool redHit : 1;
    bool yellowHit : 1;

    bool blueHit : 1;
    bool orangeHit : 1;
    uint8_t : 2;
} __attribute__((__packed__)); // 11 bytes
```

#### 6-Fret Guitars

| Command ID | Description | Parameter
| :--------- | :---------- | :--------
| `0x90`     | Note hits   | `0x01`: Open<br/>`0x02`: Black 1<br/>`0x04`: Black 2<br/>`0x08`: Black 3<br/>`0x10`: White 1<br/>`0x20`: White 2<br/>`0x40`: White 3

Alternate:

```cpp
struct SantrollerHidSixFretGuitarOutput : SantrollerHidCommandEx
{
    bool openHit : 1;
    bool black1Hit : 1;
    bool black2Hit : 1;
    bool black3Hit : 1;

    bool white1Hit : 1;
    bool white2Hit : 1;
    bool white3Hit : 1;
    uint8_t : 1;
} __attribute__((__packed__)); // 11 bytes
```

#### Rock Band Drums

| Command ID | Description | Parameter
| :--------- | :---------- | :--------
| `0x90`     | Note hits   | `0x01`: Kick<br/>`0x02`: Red pad<br/>`0x04`: Yellow pad<br/>`0x08`: Blue pad<br/>`0x10`: Green pad<br/>`0x20`: Yellow cymbal<br/>`0x40`: Blue cymbal<br/>`0x80`: Green cymbal

Alternate:

```cpp
struct SantrollerHidRockBandDrumsOutput : SantrollerHidCommandEx
{
    bool kickHit : 1;
    bool redPadHit : 1;
    bool yellowPadHit : 1;
    bool bluePadHit : 1;

    bool greenPadHit : 1;
    bool yellowCymbalHit : 1;
    bool blueCymbalHit : 1;
    bool greenCymbalHit : 1;
} __attribute__((__packed__)); // 11 bytes
```

#### Guitar Hero Drums

| Command ID | Description | Parameter
| :--------- | :---------- | :--------
| `0x90`     | Note hits   | `0x01`: Kick<br/>`0x02`: Red pad<br/>`0x04`: Yellow cymbal<br/>`0x08`: Blue pad<br/>`0x10`: Orange cymbal<br/>`0x20`: Green pad

Alternate:

```cpp
struct SantrollerHidGuitarHeroDrumsOutput : SantrollerHidCommandEx
{
    bool kickHit : 1;
    bool redPadHit : 1;
    bool yellowCymbalHit : 1;
    bool bluePadHit : 1;

    bool orangeCymbalHit : 1;
    bool greenPadHit : 1;
    uint8_t : 2;
} __attribute__((__packed__)); // 11 bytes
```

#### Turntable

| Command ID | Description  | Parameter
| :--------- | :----------  | :--------
| `0x90`     | Note hits    | `0x01`: Left "open" scratch<br/>`0x02`: Left green<br/>`0x04`: Left red<br/>`0x08`: Left blue<br/>`0x10`: Right "open" scratch<br/>`0x20`: Right green<br/>`0x40`: Right red<br/>`0x80`: Right blue
| `0xA0`     | Euphoria LED | Brightness: 0 = off, 255 = max

Alternate:

```cpp
struct SantrollerHidTurntableOutput : SantrollerHidCommandEx
{
    bool leftScratchHit : 1;
    bool leftGreenHit : 1;
    bool leftRedHit : 1;
    bool leftBlueHit : 1;

    bool rightScratchHit : 1;
    bool rightGreenHit : 1;
    bool rightRedHit : 1;
    bool rightBlueHit : 1;

    uint8_t euphoriaBrightness;
} __attribute__((__packed__)); // 12 bytes
```

On turntables, commands will be ignored if both the command ID and parameter value are the same until a valid non-equal command is received. This is a workaround for DJ Hero on Xbox 360, which sweeps through the full vibration range on the left and right motors when pulsing the Euphoria LED.
