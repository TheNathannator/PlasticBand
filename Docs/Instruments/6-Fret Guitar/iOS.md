# iOS Guitar Hero Live Guitars

## Device Info

- Vendor ID: (Unknown) 
- Product ID: (Unknown) 
- Device name: `Ble Guitar`
- Bluetooth LE GATT characteristic UUID: `533E1524-3ABE-F33F-CD00-594E8B0A8EA3`

## Input Info

The information here is based on how things are handled in GHLtarUtility.

The iOS guitar is a Bluetooth LE device, and input data must be retrieved from the GATT characteristic listed above. While the report is structured identically to that of the PS3/Wii U GHL guitar, the report length appears to be shorter, based on the comments in GHLtarUtility. The code doesn't confirm this though, as the length of the buffer to read the data isn't hardcoded, unlike how it handles other devices.

---

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
struct iOSSixFretGuitarState
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

    bool : 1;
    bool pause : 1;
    bool ghtv : 1;
    bool heroPower : 1;

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
}
```

## References

- https://github.com/ghlre/GHLtarUtility
