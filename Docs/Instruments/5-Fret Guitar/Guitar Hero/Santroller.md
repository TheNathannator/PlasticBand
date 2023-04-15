# Santroller HID Guitar Hero Guitars

The only real difference between this and a PS3 Guitar Hero Guitar, is that we also map tilt to the left stick x, as by default most applications don't know how to deal with the accelerometer data.

## Device Info

- Vendor/product ID:
  - `1209:2882`
- Revision:
  - `0x070x`
  - The last nibble may vary, see [the main Santroller doc](../../Other/Santroller.md).
- Device name:
  - Santroller

## Input Info

Start/select, PS button, and d-pad work as normal, with the exception that the d-pad is neutral at 31 (`0x1F`) instead of 8.

Frets:

| Fret   | Button |
| :--:   | :----: |
| Green  | ×      |
| Red    | ○      |
| Yellow | □      |
| Blue   | Δ      |
| Orange | L1     |

Strumbar: D-pad up/down

- Note again that the PoV hat byte is neutral at 31 (`0x1F`) instead of 8 on GH guitars.

Tilt: Left Stick X

- This is centered at 128, and is above 128 if the guitar is tilted up and below 128 if it is tilted down.

Whammy: Right stick X

Touch/slider bar: Right stick Y

- See the [General Notes](General%20Notes.md) document for more info.

RJ14 (pedal) port: R1

Accelerometers:

- These guitars feature a full 3-axis accelerometer which outputs values relative to gravity. The axis names used here match the fields that they're placed on compared to the normal PS3 controller report.
- X axis (used for standard tilt): Accelerometer X
  - Above `0x200` when the guitar is pointed up, below when the guitar is pointed down.
- Y axis: Accelerometer Y
  - Above `0x200` when the guitar is face down, below when the guitar is face up.
- Z axis: Accelerometer Z
  - Above `0x200` when the guitar is pointed right, below when the guitar is pointed left.
- Values and ranges may vary. All values listed are nominal, for proper handling calibration will need to be provided.

- X and Z are limited to a range of `0x180`-`0x280`.
- When reaching its minimum, X will snap to the maximum. After a certain point beyond that, it will somewhat jitter between max and min.


### As A Struct

```cpp
struct SantrollerGuitarHeroGuitarState
{
    uint8_t reportId;

    bool yellow : 1;
    bool green : 1;
    bool red : 1;
    bool blue : 1;

    bool orange : 1;
    bool spPedal : 1;
    bool : 1;
    bool : 1;

    bool select : 1;
    bool start : 1;
    bool : 1;
    bool : 1;

    bool ps : 1;
    uint8_t : 3;

    //      0
    //   7     1
    // 6    31   2
    //   5     3
    //      4
    uint8_t dpad_strum;

    uint8_t tilt;
    uint8_t unused1;
    uint8_t whammy;
    uint8_t slider;

    uint8_t unused2[12];

    // Reminder that these values are 10-bit in range
    int16_t accelX;
    int16_t accelZ;
    int16_t accelY;
    int16_t unused3;
}
```

## References

- https://sanjay900.github.io/guitar-configurator/controller-reverse-engineering/ps3-gh-guitar.html
- [Phase Shift mappings file](../../Other/device_list.json)
