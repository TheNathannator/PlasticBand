# PS5 PDP Riffmaster Guitar

The PS4/5 Riffmaster in PS5 mode.

## Device Info

- Vendor/product ID: `0E6F:0249`
- Revision: `0x0101`
- Device name: `PDP RiffMaster Guitar for PS5`

## Input Info

This input report deviates from other PlayStation devices and uses the PS5 DualSense output report. For simplicity, the report will be documented similarly to how Xbox One reports are documented.

Length: 64 bytes

- Byte 0: Report ID (`0x01`)
- Byte 1: Joystick X
  - Left is `0x00`, right is `0xFF`.
- Byte 2: Joystick Y
  - Up is `0x00`, down is `0xFF`.
- Bytes 1-7: Unused
  - 1-4 are the DualSense stick axes, and are `0x80`
  - 5-7 are `0x00`
- Byte 8-10: Buttons/d-pad
  - Byte 8, bits 0-3 (`0x0F`): D-pad
    - Same format as usual for PS3/4 stuff. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
      Visual representation:

      ```
           0
        7     1
      6    15   2
        5     3
           4
      ```

      Note that the d-pad is neutral at 15 (`0x0F`) instead of 8.
  - Byte 8, bit 4 (`0x10`): Blue fret flag (□ button)
  - Byte 8, bit 5 (`0x20`): Green fret flag (× button)
  - Byte 8, bit 6 (`0x40`): Red fret flag (○ button)
  - Byte 8, bit 7 (`0x80`): Yellow fret flag (Δ button)
  - Byte 9, bit 0 (`0x01`): Orange fret flag (L1 button)
  - Byte 9, bit 1 (`0x02`): Unused
  - Byte 9, bit 2 (`0x04`): Unused
  - Byte 9, bit 3 (`0x08`): Unused
  - Byte 9, bit 4 (`0x10`): Select (Share / `\|/`) button
  - Byte 9, bit 5 (`0x20`): Start (Options / `≡`) button
  - Byte 9, bit 6 (`0x40`): Solo fret flag / Joystick click (L3 button)
  - Byte 9, bit 7 (`0x80`): P1 button default (R3 button)
    - Note that the P1 button is a programmable button, and can be remapped to any other input on the guitar.
  - Byte 10, bit 0 (`0x01`): PlayStation button
- Byte 11: Unused (`0x00`)
- Bytes 12-15: Packet counter (little-endian)
- Bytes 16-40: Unused
  - Not all `0x00`, but the data doesn't seem to be relevant to inputs
- Bytes 41: Whammy
  - Ranges from `0x00` when not pressed to `0xFF` when fully pressed.
- Bytes 42: Tilt
  - Nominally, `0x00` when parallel, `0xFF` when straight up.
- Bytes 43: Upper fret bitmask
  - Bit 0 (`0x01`) - Green
  - Bit 1 (`0x02`) - Red
  - Bit 2 (`0x04`) - Yellow
  - Bit 3 (`0x08`) - Blue
  - Bit 4 (`0x10`) - Orange
- Bytes 44: Lower (solo) fret bitmask
  - Bit 0 (`0x01`) - Green
  - Bit 1 (`0x02`) - Red
  - Bit 2 (`0x04`) - Yellow
  - Bit 3 (`0x08`) - Blue
  - Bit 4 (`0x10`) - Orange
- Bytes 45-55: Unused (`0x00`)
- Bytes 56-63: Checksum
  - Unsure how this checksum is implemented; for general purposes it likely doesn't matter.

### As A Struct

```cpp
struct PS5RiffmasterGuitarState
{
    uint8_t reportId = 0x01;

    uint8_t joystickX;
    uint8_t joystickY;

    uint8_t unused1[5];

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum : 4;
    bool blueFlag : 1;
    bool greenFlag : 1;
    bool redFlag : 1;
    bool yellowFlag : 1;

    bool orangeFlag : 1;
    uint8_t : 3;
    bool share : 1;
    bool options : 1;
    bool soloFlag : 1;
    bool p1 : 1;

    bool ps : 1;
    uint8_t : 7;

    uint8_t unused2;

    uint32le_t packetCounter;

    uint8_t unused3[25];

    uint8_t whammy;
    uint8_t tilt;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;
    bool orange : 1;
    bool : 1;
    bool : 1;
    bool : 1;

    bool soloGreen : 1;
    bool soloRed : 1;
    bool soloYellow : 1;
    bool soloBlue : 1;
    bool soloOrange : 1;
    bool : 1;
    bool : 1;
    bool : 1;

    uint8_t unused4[11];
    uint64_t checksum;

    bool joystickClick() { return soloFlag && !(soloGreen | soloRed | soloYellow | soloBlue | soloOrange); }
} __attribute__((__packed__));
```
