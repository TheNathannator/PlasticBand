# Rock Band MIDI Pro Adapter for Xbox 360, PS3, and Wii

The Rock Band MIDI Pro Adapter (MPA) allows connecting MIDI drumkits and keyboards, along with the Pro Squier guitar, for use in Rock Band 3. As implied, it has 3 modes: guitar, keyboard, and drums, each of which emulate the regular Pro Guitar, Drums, and Keyboard instrument controllers.

Aside from some product ID differences on PS3 and Wii, there is no additional logic needed to support the MPA. It functions identically to each of the respective instruments for each mode, with potentially extremely minor differences that won't matter for anything.

## Hardware IDs

These hardware IDs are listed in the respective instrument docs as well, for easy reference they are copied here.

Some product IDs are assumed based on observed patterns:

- With the exception of the PS3 drumkit, the first number indicates the platform, and the second indicates the instrument type.
  - First number: 2 is PS3, 3 is Wii.
  - Second number: 1 is drums, 3 is keyboard, 4 is Mustang guitar mode, 5 is Squire guitar mode.
- The MPA product ID ends with 8, where the normal instrument's product ID ends with 0.

Xbox 360:

- XInput Type: Gamepad (1)
- XInput Subtype:
- Product ID:
  - Guitar mode: 25, not part of XInput standards
  - Drums mode: Drumkit (8)
  - Keyboard mode: 15, not part of XInput standards

PS3:

- Vendor ID: `0x12BA`
- Product ID:
  - Guitar mode with the Pro Mustang: `0x2438`
  - Guitar mode with the Pro Squire: `0x2538`
  - Drums mode: `0x0218`
  - Keyboard mode: `0x2338`

Wii:

- Vendor ID: `0x1BAD`
- Product ID:
  - Guitar mode with the Pro Mustang: `0x3438`
  - Guitar mode with the Pro Squire: `0x3538`
  - Drums mode: `0x3138`
  - Keyboard mode: `0x3338`

## References

- https://docs.google.com/spreadsheets/d/1Y3QM1tEcf0bGiUTjT7R-3mwEAKrCL0qYoySmk3RLo8c/edit?usp=sharing
