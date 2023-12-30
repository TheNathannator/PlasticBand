# Guitar Praise Guitar for PC

## Device Info

- Vendor/product ID: `0314:0830`
- Revision: `0x0110`
- Device name: `GUITAR PRAISE` / `GUITAR PRAISE十⊒  �ï㖗痜藀矍`
  - The latter appears in the [USB descriptor dump](../../Descriptor%20Dumps/Guitar%20Praise/USB%20Descriptor.txt), though not in the actual dumped data it seems.

## Input Info

This guitar uses a more bespoke input report, though it does not appear to be purpose-built.

Length: 8 bytes

- Byte 0: Report ID (always 1)
- Bytes 1-3: Unused axes
  - Always `0x80`.
- Byte 4: Whammy
  - Inverted: `0xFF` is released, `0x00` is fully pressed.
- Bytes 5-6: Button bitmask
  - Byte 5, bits 0-3 (`0x0F`): D-pad/strum bar
    - Same format as the PS3 guitars. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
      Visual representation:

      ```
          0
        7   1
      6   8   2
        5   3
          4
      ```

    - The guitar itself does not actually have a d-pad, but the HID descriptor reports this value as a full hat switch.

  - Byte 5, bit 4 (`0x10`) - Green
  - Byte 5, bit 5 (`0x20`) - Red
  - Byte 5, bit 6 (`0x40`) - Yellow
  - Byte 5, bit 7 (`0x80`) - Blue
  - Byte 6, bit 0 (`0x01`) - Orange
  - Byte 6, bit 1 (`0x02`) - Tilt
  - Byte 6, bit 2 (`0x04`) - Start
  - Byte 6, bit 3 (`0x08`) - Select
  - Byte 6, bit 4-7 - Unused
- Byte 7: Unused/strum bar duplicate
  - The HID descriptor reports this byte as constant (it's a continuation of the 4 unused button bits), but the guitar for some reason reports something correlating to strumbar state here.
  - `0x09` when strumming up, `0x04` when strumming down, `0x00` otherwise.

### As A Struct

```cpp
struct GuitarPraiseState
{
    uint8_t reportId = 0x01;

    uint8_t unused[3];
    uint8_t whammy;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad_strum : 4;

    bool green : 1;
    bool red : 1;
    bool yellow : 1;
    bool blue : 1;

    bool orange : 1;
    bool tilt : 1;
    bool start : 1;
    bool select : 1;

    uint8_t : 4;

    uint8_t strumBar2;
};
```

## Output Info

The HID descriptor describes an output report for some reason, probably left over from whatever firmware the guitar uses (the presence of the 3 unused axes in the input indicates this is likely gamepad firmware). This output report is described as vendor defined, so not much can be guessed about it.

```cpp
struct GuitarPraiseOutput
{
    uint8_t reportId = 0x01;

    uint8_t unknown[4];
};
```

## References
