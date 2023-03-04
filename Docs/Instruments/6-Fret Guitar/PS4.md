# PS4 Guitar Hero Live Guitars

## Device Info

- Vendor ID: `0x12BA` ("Licensed by Sony Computer Entertainment America")
- Product ID: `0x07BB`

## Input Info

TODO: Some details may need to be double-checked, notably the d-pad center, pause, Hero Power, and GHTV buttons. 

The d-pad works as normal, with the exception that it is neutral at 15 (`0x0F`) instead of 8.

Frets:

| Fret    | Button |
| :---    | :---:  |
| Black 1 | ×      |
| Black 2 | ○      |
| Black 3 | Δ      |
| White 1 | □      |
| White 2 | L1     |
| White 3 | R1     |

Strumbar: Left stick Y

- `0x80` when not pressed, `0x00` when strumming up, `0xFF` when strumming down.

Whammy: Right stick X

- Ranges from `0x80` when not pressed to `0xFF` when fully pressed. (Assumed based on other GHL guitars)

Tilt: Right stick Y

D-pad center button: Share button

Pause buttons: L3

Hero Power button: R3

GHTV button: L2

### As A Struct

```cpp
struct PS4SixFretGuitarState
{
    uint8_t reportId;

    uint8_t unused1;
    uint8_t strum;
    uint8_t whammy;
    uint8_t tilt;

    //      0
    //   7     1
    // 6    15   2
    //   5     3
    //      4
    uint8_t dpad : 4;
    bool white1 : 1;
    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool ghtv : 1;
    bool : 1;
    bool dpadCenter : 1;
    bool : 1;
    bool pause : 1;
    bool heroPower : 1;

    uint8_t unused2[57];
};
```

## Output Reports

### Report ID `0x30`: Keep-Alive Packet

This guitar requires a keep-alive packet to be sent every 8 seconds at minimum in order for full input data to be sent. Without it, the strumbar will cut out any fret inputs being held.

Length: 8 bytes

```cpp
struct PS3GHLGuitarKeepAlive
{
    uint8_t reportId = 0x30;

    uint8_t data[8] = { 0x02, 0x08, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00 };
};
```

TODO:

- Is the report ID correct/needed?
- Does this report ID also handle setting player LEDs?

## References

- https://github.com/Sera486/GHLtarUtility
- https://github.com/evilynux/hid-ghlive-dkms
- https://github.com/Octave13/GHLPokeMachine
