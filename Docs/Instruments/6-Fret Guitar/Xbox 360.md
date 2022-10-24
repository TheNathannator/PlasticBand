# Xbox 360 Guitar Hero Live Guitars

TODO: This document was written without actual hardware to test with. Everything here needs to be verified, and missing information needs to be filled out. 

## Controller Info

- XInput Type: 
- XInput Subtype: 

## Input Info

TODO: Face buttons 

Frets:

| Action  | Input        |
| :-----  | :---:        |
| Black 1 | A            |
| Black 2 | B            |
| Black 3 | Y            |
| White 1 | X            |
| White 2 | Left bumper  |
| White 3 | Right bumper |

Tilt and whammy are swapped compared to the other Xbox 360 guitars.

Whammy: Right stick Y

- Resting state is negative, pressed state is positive.

Tilt: Right stick X

### As A Struct

```c
struct XInput6FretGuitarGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool guide : 1;
    bool reserved : 1;

    bool black1 : 1;
    bool black2 : 1;
    bool white1 : 1;
    bool black3 : 1;

    uint16_t unused1;
    uint32_t unused2;
    int16_t tilt;
    int16_t whammy;
}
```
