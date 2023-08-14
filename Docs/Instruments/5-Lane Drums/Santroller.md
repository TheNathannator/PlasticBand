# Santroller Guitar Hero Drums

## Device Info

- Vendor/product ID
  - USB: `1209:2882`
  - Bluetooth: `1209:2885`
- Revision: `0x0500`
- Device name: `Santroller`

## Input Info

Length: 10 bytes

- Byte 0: Report ID (always 1)
- Bytes 1-2: 16-bit button bitmask
  - Byte 0, bit 0 (`0x01`) - Green pad, A / × button
  - Byte 0, bit 1 (`0x02`) - Red pad, B / ○ button
  - Byte 0, bit 2 (`0x04`) - Yellow cymbal, Y / Δ button
  - Byte 0, bit 3 (`0x08`) - Blue pad, X / □ button
  - Byte 0, bit 4 (`0x10`) - Orange cymbal
  - Byte 0, bit 5 (`0x20`) - Kick
  - Byte 0, bit 6 (`0x40`) - Select button
  - Byte 0, bit 7 (`0x80`) - Start button
  - Byte 1, bit 0 (`0x01`) - Home button
  - Byte 1, bits 1-7 - Unused
- Byte 3 - D-pad
  - Same format as the PS3 drumkits. This value is not a bitmask, rather it encodes different possible states as individual numbers.\
    Visual representation:

    ```
        0
      7   1
    6   8   2
      5   3
        4
    ```

- Bytes 4-9 - Velocities
  - Byte 4 - Green
  - Byte 5 - Red
  - Byte 6 - Yellow
  - Byte 7 - Blue
  - Byte 8 - Orange
  - Byte 9 - Kick
  - Full range: 0 = no hit, 1 = min velocity, 255 = max velocity.

### As A Struct

```cpp
struct SantrollerFiveLaneDrumsState
{
    uint8_t reportId = 0x01;
    
    uint8_t green_a : 1; // cross
    uint8_t red_b : 1; // circle
    uint8_t yellow_y : 1; // triangle
    uint8_t blue_x : 1; // square

    uint8_t orange : 1;
    uint8_t kick : 1;
    uint8_t select : 1;
    uint8_t start : 1;

    uint8_t home : 1;
    uint8_t : 7;

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
    uint8_t velocity_orange;
    uint8_t velocity_kick;
} __attribute__((__packed__)); // 10 bytes
```

## References

- https://github.com/sanjay900/Ardwiino/blob/master/include/reports/pc_reports.h
