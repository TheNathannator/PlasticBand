# PS3/Wii U Guitar Hero Live Guitars

## Device Info

- Vendor/product ID: `12BA:074B`
- Revision:
- Device name: 
- PS3 ID: 

## Input Info

This device sends a report modelled after the one [documented here](../../Controller%20Communication%20Basics/PS3%20Instruments.md). Refer to that document for the base layout.

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

Whammy: Right stick Y

- Ranges from `0x80` when not pressed to `0xFF` when fully pressed. (Assumed based on other GHL guitars)

Tilt: Accelerometer X

D-pad center button: PlayStation button

Pause buttons: Start button

Hero Power button: Select button

GHTV button: L3

### As A Struct

```cpp
struct PS3WiiUSixFretGuitarState
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    bool white1 : 1;
    bool black1 : 1;
    bool black2 : 1;
    bool black3 : 1;

    bool white2 : 1;
    bool white3 : 1;
    bool : 1;
    bool : 1;

    bool heroPower : 1;
    bool pause : 1;
    bool ghtv : 1;
    bool : 1;

    bool dpad_center : 1;
    uint8_t : 3;

    //      0
    //   7     1
    // 6    15   2
    //   5     3
    //      4
    uint8_t dpad;

    uint8_t unused1;
    uint8_t strumBar;
    uint8_t unused2;
    uint8_t whammy;

    uint8_t unused3[12];

    // Reminder that this value is 10-bit in range
    uint16le_t tilt;

    uint16le_t unused4[3];
} __attribute__((__packed__));
```

## Output Reports

### Output Type `0x02`: Keep-Alive Packet

This guitar requires a keep-alive packet to be sent every 10 seconds at minimum in order for full input data to be sent. Without it, the strumbar will cut out any fret inputs being held.

Length: 8 bytes

```cpp
struct PS3GHLGuitarKeepAlive
{
#ifdef WINDOWS
    uint8_t reportId = 0x00;
#endif

    uint8_t outputType = 0x02;
    uint8_t data[7] = { 0x08, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00 };
} __attribute__((__packed__));
```

## References

- https://github.com/ghlre/GHLtarUtility
- https://github.com/evilynux/hid-ghlive-dkms
- https://github.com/Octave13/GHLPokeMachine
- https://github.com/RPCS3/rpcs3/blob/master/rpcs3/Emu/Io/GHLtar.cpp
