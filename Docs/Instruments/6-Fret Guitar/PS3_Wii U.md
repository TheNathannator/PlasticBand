# PS3/Wii U Guitar Hero Live Guitars

## Device Info

- Vendor ID: `0x12BA` ("Licensed by Sony Computer Entertainment America")
- Product ID: `0x074B`
- PS3 ID: 

## Input Info

The d-pad works as normal, with the exception that it is neutral at 15 (`0x0F`) instead of 8.

Frets:

| Fret    | Button |
| :---    | :---:  |
| Black 1 | ×      |
| Black 2 | ○      |
| Black 3 | Δ      |
| White 1 | □      |
| White 2 | L1     |
| White 3 | L2     |

Strumbar: Left stick Y

Whammy: Right stick Y

- TODO: This might behave more strangely than other guitars, based on the RPCS3 emulation code for this. 

Tilt: Accelerometer X

D-pad center button: PlayStation button

Pause buttons: Start button

Hero Power button: Select button

GHTV button: L3

### As A Struct

```c
struct PS3WiiUSixFretGuitarState
{
    uint8_t reportId;

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
    int16_t tilt;

    int16_t unused4[3];
}
```

## Keep-Alive Packet

This guitar requires a keep-alive packet to be sent every 10 seconds at minimum in order for full input data to be sent. This is sent as an HID output report, and there are 3 variations of this packet used by different projects that all seem to work:

- GHLtarUtility: `0x02, 0x08, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00` (though the code seems to only send 8 bytes of this)
- hid-ghlive-dkms: `0x02, 0x08, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00`
- GHLPokeMachine: `0x02, 0x02, 0x08, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00`

Without this, the strumbar will cut out any fret inputs being held.

## References

- https://github.com/ghlre/GHLtarUtility
- https://github.com/evilynux/hid-ghlive-dkms
- https://github.com/Octave13/GHLPokeMachine
- https://github.com/RPCS3/rpcs3/blob/master/rpcs3/Emu/Io/GHLtar.cpp
