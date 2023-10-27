# Santroller Rock Band Drums

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0600`
- Device name: `Santroller`

## Input Info

Length: 11 bytes

To ensure better user experience regarding menu navigation, the method used on Xbox 360/PS3/Wii drumkits for pad/cymbal hits is used here. Without it, there is no easy way to ensure games which do more generic control binding will recognize the face buttons and pads as the same thing, which is important for menus.

In particular, this is done for compatibility with Clone Hero; it does not allow multiple bindings to an action and does not have separate menu navigation actions, so separate face button bits will cause issues with binding and also prevent using the face buttons on menus.

- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - A / × button, green pad/cymbal
  - Byte 0, bit 1 (`0x02`) - B / ○ button, red pad
  - Byte 0, bit 2 (`0x04`) - X / □ button, blue pad/cymbal
  - Byte 0, bit 3 (`0x08`) - Y / Δ button, yellow pad/cymbal
  - Byte 0, bit 4 (`0x10`) - Pad hit flag
  - Byte 0, bit 5 (`0x20`) - Cymbal hit flag
  - Byte 0, bit 6 (`0x40`) - 1st kick pedal
  - Byte 0, bit 7 (`0x80`) - 2nd kick pedal
  - Byte 1, bit 0 (`0x01`) - Select button
  - Byte 1, bit 1 (`0x02`) - Start button
  - Byte 1, bit 2 (`0x04`) - Home button
  - Byte 1, bit 3-7 - Unused
- Byte 3 - D-pad
  - Same format as the PS3 drums. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
    Visual representation:

    ```
        0
      7   1
    6   8   2
      5   3
        4
    ```

    The d-pad is also used on cymbal hits for pad/cymbal distinguishing purposes, yellow cymbal does up and blue cymbal does down. Refer to the [General Notes](General%20Notes.md#deciphering-pads-and-cymbals) doc for full details.

- Bytes 4-10 - Velocities
  - Byte 4 - Green pad
  - Byte 5 - Red pad
  - Byte 6 - Yellow pad
  - Byte 7 - Blue pad
  - Byte 8 - Green cymbal
  - Byte 9 - Yellow cymbal
  - Byte 10 - Blue cymbal
  - Full range: 0 = no hit, 1 = min velocity, 255 = max velocity.

### As A Struct

```cpp
struct SantrollerFourLaneDrumsState
{
    uint8_t reportId = 0x01;

    uint8_t a_green : 1;  // cross
    uint8_t b_red : 1;  // circle
    uint8_t x_blue : 1;  // square
    uint8_t y_yellow : 1;  // triangle

    uint8_t padFlag : 1; 
    uint8_t cymbalFlag : 1;
    uint8_t kick1 : 1;
    uint8_t kick2 : 1;

    uint8_t select : 1;
    uint8_t start : 1;
    uint8_t guide : 1;
    uint8_t : 5;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad;

    uint8_t velocity_green;
    uint8_t velocity_red;
    uint8_t velocity_yellow;
    uint8_t velocity_blue;
    uint8_t velocity_greenCymbal;
    uint8_t velocity_yellowCymbal;
    uint8_t velocity_blueCymbal;
} __attribute__((__packed__)); // 11 bytes
```

## References

- https://github.com/sanjay900/Ardwiino/blob/master/include/reports/pc_reports.h
