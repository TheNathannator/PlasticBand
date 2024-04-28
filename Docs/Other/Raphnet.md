## Device Info

- Vendor/product ID:
  - `289b:0028` - 1-player WUSBMote v2.0
  - `289b:0029` - 2-player WUSBMote v2.0
  - `289b:002A` - 1-player WUSBMote v2.0 (mouse mode)
  - `289b:002B` - 1-player WUSBMote v2.1
  - `289b:002C` - 2-player WUSBMote v2.1
  - `289b:0080` - 1-player WUSBMote v2.2
  - `289b:0081` - 2-player WUSBMote v2.2
  - `289b:0082` - WUSBMote v2.2 (mouse mode)
  - `289b:0083` - WUSBMote v2.2 (tablet mode)

- Device name: `1-player WUSBMote v2.2`
- Manufacturer name: `raphnet technologies`
- Revision: `2.20`

## Axis
Axis range from 0 to 32000. O is left/up, 32000 is right/down. 
If `dpad to axis` is enabled, then the dpad values are mapped to the left joystick Y axis

## Guitar
```c
typedef struct {
    uint8_t reportId;
    uint16_t joyX;
    uint16_t joyY;
    uint16_t unused;
    uint16_t slider;
    uint16_t unused2;
    uint16_t whammy;
    uint8_t green : 1;
    uint8_t red : 1;
    uint8_t yellow : 1;
    uint8_t blue : 1;
    uint8_t orange : 1;
    uint8_t up : 1;
    uint8_t plus : 1;
    uint8_t minus : 1;

    uint8_t down : 1;
    uint8_t : 7;

} __attribute__((packed)) RaphnetGuitar_Data_t;
```
Whammy on my guitar seems to start at 12000 when unpressed, and then it goes to 30000 when pressed