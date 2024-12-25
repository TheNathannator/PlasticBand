# Instruemnt Peripheral Documentation

This folder contains documentation on how to communicate with and interpret the data of instrument peripherals, primarily those from Rock Band and Guitar Hero.

## Contents

- [_Templates](_Templates/) - Templates for new device docs.
- [Base Reports](Base%20Reports/) - Base report definitions for each general format of report.
- [Controller Communication Basics](Controller%20Communication%20Basics/) - The basics on how to read each category of peripheral, from an application point of view. Primarily covers which system APIs or libraries are available for the task.
- [Descriptor Dumps](Descriptor%20Dumps/) - Descriptor dumps for various peripherals.
- [Instruments](Instruments/) - Documentation for instrument peripherals.
- [Other](Other/) - Documentation for miscellaneous peripherals, plus other miscellaneous resources.

Take a look at the [Controller Communication Basics](Controller%20Communication%20Basics/) folder to familiarize yourself with how to interact with the peripherals on a fundamental level before taking a look at the specific docs, as well as the [Base Reports](Base%20Reports/) folder to learn the base layouts for specific platform's peripherals. The docs assume you are familiar with these base layouts, and refer to names on them to document inputs.

Data structures in these documents are primarily for documentation only, and are not guaranteed to function correctly as-is (as thoroughly testing them is non-trivial). Eventually a collection of properly-defined structures for various languages is planned, but for now be careful to ensure that endianness and packing are correctly handled if you use them.

## Contributions

These docs are a work in progress, contributions are very welcome!

To get started, use one of the templates in the [_Templates](_Templates/) folder. Fill out information required to identify the device, include any device descriptors if possible, and give a writeup on how its input data works, along with a data structure to represent the data. If you're missing info, be sure to make note of such. Descriptor dumps can be added to the [Descriptor Dumps](Descriptor%20Dumps/) folder.

You may add images to documentation, and you may also create an `_Ignored` folder anywhere within the repository to have a folder you can put anything in without fear of accidentally committing it.

## Todo List

- All HID devices:
  - [ ] Revision numbers
  - [ ] Reported device names
- 5-fret guitar
  - Guitar Hero
    - [x] Xbox 360
    - [ ] ~~PS2~~
      - There is no standardization across different PS2 adapters, so there is no uniform way to natively support these guitars within a game. They will require manual mapping.
    - [x] PS3
    - [ ] Wii
  - Rock Band
    - [x] Xbox 360
    - [x] Xbox One
    - [x] PS2/3
    - [x] PS4
    - [x] Wii
- 6-fret guitar
  - [x] Xbox 360
  - [x] Xbox One
  - [x] PS3/Wii U
  - [x] PS4
  - [x] iOS
    - TODO: vendor/product ID
- 4-lane (Rock Band) drums
  - [x] Xbox 360
  - [x] Xbox One
  - [x] PS2/3
  - [x] PS4
  - [x] Wii
- 5-lane (Guitar Hero) drums
  - [x] Xbox 360
  - [x] PS2/3
  - [ ] Wii
- Turntable (DJ Hero)
  - [x] Xbox 360
  - [x] PS3
  - [ ] Wii
- Pro Guitar (Rock Band)
  - [x] Xbox 360
  - [x] PS3
  - [x] Wii
- Keyboard/keytar (Rock Band)
  - [x] Xbox 360
  - [x] PS3
  - [x] Wii
- Other
  - [x] Rock Band 4 Legacy Wireless Adapter
  - [x] Rock Band 4 Legacy Wired Adapter
  - [x] Xbox 360 Rock Band 2 Stage Kit
  - [ ] Raphnet Adapter
    - [ ] With a guitar connected
    - [ ] With a drumkit connected
    - [ ] With a turntable connected
    - [ ] etc...
  - [x] Santroller Devices

There may be other things not listed here, search for `TODO` in all files to see what else there is.

## License

[![Creative Commons Attribution-ShareAlike 4.0 International License](https://i.creativecommons.org/l/by-sa/4.0/88x31.png)](https://creativecommons.org/licenses/by-sa/4.0/)

This documentation is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/). See [LICENSE](LICENSE) or the Creative Commons website for details.
