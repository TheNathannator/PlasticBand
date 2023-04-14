# PS4 Guitar Hero Live Guitars

## Device Info

- Vendor ID: `1430:07BB`

## Input Info

The d-pad works as normal.

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

Whammy: Right stick Y

- Ranges from `0x80` when not pressed to `0xFF` when fully pressed.

Tilt: Right stick X

- Centered at `0x80`, `0xFF` when fully tilted up, `0x00` when fully tilted down.
- Also mirrored onto accelerometer X (bottom byte is the value, top byte is 0).
- Accelerometer Y (bytes 22-23) is `0x100` normally, `0x200` when fully tilted up, and `0x000` when fully tilted down.

D-pad center button: PlayStation button

Pause buttons: Options button

Hero Power button: R3

GHTV button: L3

There are some other details as well; these are listed in the state struct below.

### As A Struct

```cpp
struct PS4SixFretGuitarState
{
    uint8_t reportId;

    uint8_t unused1;
    uint8_t strum;
    uint8_t tilt;
    uint8_t whammy;

    //     0
    //   7   1
    // 6   8   2
    //   5   3
    //     4
    uint8_t dpad : 4;
    bool white1 : 1;
    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;

    bool white2 : 1;
    bool white3 : 1;
    uint8_t : 3;
    bool pause : 1;
    bool ghtv : 1;
    bool heroPower : 1;

    bool dpadCenter : 1;
    uint8_t : 7;

    uint8_t unused2[11];

    int16_t tilt2;
    int16_t tiltActive;
    int16_t unused3;

    bool isConnected; // 0 when guitar not connected, 1 when connected

    uint8_t unused4[38];
};
```

## Output Reports

### Report ID `0x30`: Keep-Alive Packet

This guitar requires a keep-alive packet to be sent every 8 seconds at minimum in order for full input data to be sent. Without it, the strumbar will cut out any fret inputs being held.

Length: 8 bytes

```cpp
struct PS4GHLGuitarKeepAlive
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
