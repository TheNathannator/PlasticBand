# PS3/Wii U Guitar Hero Live Guitars

## Device Info

- Vendor/product ID: `12BA:074B`
- Revision:
- Device name: 
- PS3 flags: `0x06`

## Input Info

This device sends a report modelled after the one [documented here](../../Base%20Reports/PS3.md). Refer to that document for the base layout.

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

    bool white1 : 1; // square
    bool black1 : 1; // cross
    bool black2 : 1; // circle
    bool black3 : 1; // triangle

    bool white2 : 1; // l1
    bool white3 : 1; // r1
    bool : 1;
    bool : 1;

    bool heroPower : 1; // select
    bool pause : 1; // start
    bool ghtv : 1; // l3
    bool : 1;

    bool dpad_center : 1;
    uint8_t : 3;

    //      0
    //   7     1
    // 6    15   2
    //   5     3
    //      4
    uint8_t dpad;

    uint8_t unused_leftStickX;
    uint8_t strumBar; // leftStickY
    uint8_t unused_rightStickX;
    uint8_t whammy; // rightStickY

    uint8_t unused_pressure[12];

    // Reminder that this value is 10-bit in range
    uint16le_t tilt; // accelX
    uint16le_t unused_accelerometer[3];
} __attribute__((packed));
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
} __attribute__((packed));
```

## References

- https://github.com/ghlre/GHLtarUtility
- https://github.com/evilynux/hid-ghlive-dkms
- https://github.com/Octave13/GHLPokeMachine
- https://github.com/RPCS3/rpcs3/blob/master/rpcs3/Emu/Io/GHLtar.cpp
