# Santroller Devices

Santroller Devices are devices programmed using the newest version of the [Santroller Configuration Tool](https://github.com/sanjay900/guitar-configurator). This is a tool written by [sanjay900](https://github.com/sanjay900) for emulating most instruments. On a Computer, these will use a format similar to a PS3 Instrument, or a xbox 360 instrument on windows. However, these devices also have some additional features that the PS3 instruments dont have, and that information is detailed below.

## Device Info

- Vendor/product ID:
  - `1209:2882`
- Device name:
  - Santroller
- Manufacturer name:
  - sanjay900
- Revision:
  - `0xttrc` where t is the device type, r is the rhythm type and c is the console type
    - Device Type
      - 1: Gamepad
      - 2: Wheel
      - 3: Arcade Stick
      - 4: Flight Stick
      - 5: Dance Pad
      - 6: Arcade Pad
      - 7: Guitar
      - 8: Guitar Hero Live Guitar
      - 9: Drum kit
      - A: DJ Hero Turntable
    - Rhythm Type
      - For most devices, this should be 0, but for Guitars and Drum Kits, it will be as follows:
        - 0: Guitar Hero
        - 1: Rock Band
    - Console Type
      - For usb devices this will always be 0. For bluetooth devices however, this can change depending on if the user has plugged their guitar into a console or held down a console mode binding
        - 0: Universal
        - 1: Keyboard or Mouse
        - 2: MIDI
        - 3: Stage Kit
        - 4: Xbox 360
        - 5: PS3
        - 6: Wii Rock Band
        - 7: Switch
        - 8: PS4
        - 9: Xbox One
        - A: PS3 - in gamepad mode, this actually will expose a report format more similar to a rhythm controller, and this is just because of some quirks with emulating PS3 controllers
        - B: Windows / Xbox One Detection - this mode is used if the controller is trying to detect if it is plugged into a windows PC or if it is plugged into an Xbox One.
        - C: Windows XInput

## Differences
One of the main differences from the PS3 instruments is that these devices actually have a report ID, as this was necessary for allowing it to detect a PS4 console.
For Guitars, the Tilt information is also copied to Left Stick X, as this makes it easier to use in games.

## LED Commands
Santroller devices support the Stage Kit format, but they also support an extended set of LED commands as well. 
For PS3 instruments, these support the following output report to emulate the rumble command format of the stage kit:
```c
struct SantrollerLeds {
    uint8_t reportId = 0x01;
    uint8_t outputType = 0x5A;
    uint8_t unk1 = 0x08;
    uint8_t rumble_left;
    uint8_t rumble_right;
    uint8_t padding[4];
}
```

## XInput Turntable
The turntable normally sends its euphoria state on both the left and right rumble values, however this would be incompatible with the extra commands. 
Due to this, the following command is used for the turntables.
- Command 0x18: Euphoria LED state - Expected to be sent when the euphoria led needs to be lit up, the left byte is used for the brightness of the LED.

## Extra commands
The santroller LED format uses the right motor as a command, and the left motor as a parameter.

For the following commands, the left motor is set to 1 to turn the LED on, and 0 to turn it off.
- Command 0x0B: Solo - Expected to be sent when the player is inside a solo section.
- Command 0x0C: Open Note - Expected to be sent when the player plays a open note.
- Command 0x0D: Green Note / Green Pad - Expected to be sent when the player plays a green note.
- Command 0x0E: Red Note / Red Pad - Expected to be sent when the player plays a red note or hits a green drum pad.
- Command 0x0F: Yellow Note / Yellow Pad - Expected to be sent when the player plays a yellow note or hits a rock band yellow drum pad or guitar hero yellow cymbal.
- Command 0x10: Blue Note / Blue Pad - Expected to be sent when the player plays a blue note or hits a blue drum pad.
- Command 0x11: Orange Note / Orange Cymbal - Expected to be sent when the player plays a orange note or hits a guitar hero orange cymbal.
- Command 0x12: Rock Band Blue Cymbal - Expected to be sent when the player plays a rock band pro drum blue cymbal.
- Command 0x13: Rock Band Yellow Cymbal - Expected to be sent when the player plays a rock band pro drum yellow cymbal.
- Command 0x14: Rock Band Green Cymbal - Expected to be sent when the player plays a rock band pro drum green cymbal.
- Command 0x15: Kick Pedal - Expected to be sent when the player hits either kick pedal.

These commands instead use the left motor as a parameter
- Command 0x0A - combo - The Left motor is set to the current combo value.
- Command 0x16 - Star Power Gauge (inactive) - Expected to be sentt if the star power gauge changes, but star power is not active, and the left motor is set to the current percentage that the bar is filled, scaled from the min value to the max value.
- Command 0x17 - Star Power Gauge (active) - TExpected to be sent if the star power gauge changes, but star power is active, and the left motor is set to the current percentage that the bar is filled, scaled from the min value to the max value.