# Rock Band and Guitar Hero Peripheral Documentation

This folder contains documentation on how to communicate with and interpret the data of RB/GH peripherals.

## Contributions

These docs are a work in progress, contributions are very welcome!

To get started, use one of the templates in the [_Templates](_Templates/) folder. Fill out information required to identify the device, include any device descriptors if possible, and give a writeup on how its input data works. Code examples (including data structures) are also appreciated. If you're missing info, be sure to make note of such.

You may add images to documentation, and you may also create an `_Ignored` folder anywhere within the repository to have a folder you can put anything in without fear of accidentally committing it.

## General Notes

Data structures in these documents assume that the processor and data are little-endian, and that the structure is unaligned with no packing applied, unless otherwise specified. If you use them directly, you'll want to ensure these assumptions are correct, or that you adjust the structures to make them correct on your system.

## Todo List

- 5-fret guitar
  - Guitar Hero
    - [x] Xbox 360
    - [ ] PS2 (assuming they can be differentiated from regular PS3 controllers at all, and hoping the PS2 adapter landscape isn't too fragmented)
    - [ ] PS3
    - [ ] Wii
  - Rock Band
    - [x] Xbox 360
    - [x] Xbox One
    - [ ] PS2/3
    - [ ] PS4
    - [ ] Wii
- 6-fret guitar
  - [x] Xbox 360
  - [ ] Xbox One
  - [ ] PS3/Wii U
  - [ ] PS4
  - [ ] iOS
- 4-lane (Rock Band) drums
  - [x] Xbox 360
  - [x] Xbox One
  - [ ] PS2/3
  - [ ] PS4
  - [ ] Wii
- 5-lane (Guitar Hero) drums
  - [ ] Xbox 360
    - Document is partially done, needs hardware verification
  - [ ] PS2/3
  - [ ] Wii
- Turntable (DJ Hero)
  - [x] Xbox 360
  - [ ] PS3
  - [ ] Wii
- Pro Guitar (Rock Band)
  - [ ] Xbox 360
    - Document is partially done, needs hardware verification
  - [ ] PS3
  - [ ] Wii
- Keyboard/keytar (Rock Band)
  - [x] Xbox 360
  - [ ] PS3
  - [ ] Wii
- Other
  - [ ] Rock Band 4 Legacy Wireless Adapter
  - [ ] Rock Band 4 Legacy Wired Adapter
    - Document is partially done, needs additional research and verification

## License

[![Creative Commons Attribution-ShareAlike 4.0 International License](https://i.creativecommons.org/l/by-sa/4.0/88x31.png)](https://creativecommons.org/licenses/by-sa/4.0/)

This documentation is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/). See the Creative Commons website for details.
