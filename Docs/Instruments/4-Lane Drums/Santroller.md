# Santroller Rock Band Drums

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0600`
- Device name: `Santroller`

## Input Info

Length: 11 bytes

- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - A / × button
  - Byte 0, bit 1 (`0x02`) - B / ○ button
  - Byte 0, bit 2 (`0x04`) - X / □ button
  - Byte 0, bit 3 (`0x08`) - Y / Δ button
  - Byte 0, bit 4 (`0x10`) - Green pad
  - Byte 0, bit 5 (`0x20`) - Red pad
  - Byte 0, bit 6 (`0x40`) - Yellow pad
  - Byte 0, bit 7 (`0x80`) - Blue pad
  - Byte 1, bit 0 (`0x01`) - Green cymbal
  - Byte 1, bit 1 (`0x02`) - Yellow cymbal
  - Byte 1, bit 2 (`0x04`) - Blue cymbal
  - Byte 1, bit 3 (`0x08`) - 1st kick pedal
  - Byte 1, bit 4 (`0x10`) - 2nd kick pedal
  - Byte 1, bit 5 (`0x20`) - Select button
  - Byte 1, bit 6 (`0x40`) - Start button
  - Byte 1, bit 7 (`0x80`) - Home button
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
    uint8_t reportId;
    
    uint8_t a : 1; // cross
    uint8_t b : 1; // circle
    uint8_t x : 1; // square
    uint8_t y : 1; // triangle

    uint8_t green : 1;
    uint8_t red : 1;
    uint8_t yellow : 1;
    uint8_t blue : 1;

    uint8_t greenCymbal : 1;
    uint8_t redCymbal : 1;
    uint8_t blueCymbal : 1;
    uint8_t kick1 : 1;

    uint8_t kick2 : 1;
    uint8_t select : 1;
    uint8_t start : 1;
    uint8_t home : 1;

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
} __attribute__((__packed__));
```

## References

- https://github.com/sanjay900/Ardwiino/blob/master/include/reports/pc_reports.h
