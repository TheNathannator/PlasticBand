# Xbox 360 Rock Band 3 Keyboard

## Controller Info

- XInput Type: Gamepad (1)
- XInput Subtype: 15, not part of XInput standards

## Input Info

Face buttons work like a standard Xbox 360 controller.

### Keys

The keys are sent as a bitmask, which gets split up across several of the XInput axes.

C1 is the leftmost key, C3 is the rightmost key.

| Key | Input         | Bits                     |
| :-- | :----         | :--:                     |
| C1  | Left trigger  | `0b_1xxx_xxxx`           |
| Db1 | Left trigger  | `0b_x1xx_xxxx`           |
| D1  | Left trigger  | `0b_xx1x_xxxx`           |
| Eb1 | Left trigger  | `0b_xxx1_xxxx`           |
| E1  | Left trigger  | `0b_xxxx_1xxx`           |
| F1  | Left trigger  | `0b_xxxx_x1xx`           |
| Gb1 | Left trigger  | `0b_xxxx_xx1x`           |
| G1  | Left trigger  | `0b_xxxx_xxx1`           |
|     |               |                          |
| Ab1 | Right trigger | `0b_1xxx_xxxx`           |
| A1  | Right trigger | `0b_x1xx_xxxx`           |
| Bb1 | Right trigger | `0b_xx1x_xxxx`           |
| B1  | Right trigger | `0b_xxx1_xxxx`           |
| C2  | Right trigger | `0b_xxxx_1xxx`           |
| Db2 | Right trigger | `0b_xxxx_x1xx`           |
| D2  | Right trigger | `0b_xxxx_xx1x`           |
| Eb2 | Right trigger | `0b_xxxx_xxx1`           |
|     |               |                          |
| E2  | Left stick X  | `0b_xxxx_xxxx_1xxx_xxxx` |
| F2  | Left stick X  | `0b_xxxx_xxxx_x1xx_xxxx` |
| Gb2 | Left stick X  | `0b_xxxx_xxxx_xx1x_xxxx` |
| G2  | Left stick X  | `0b_xxxx_xxxx_xxx1_xxxx` |
| Ab2 | Left stick X  | `0b_xxxx_xxxx_xxxx_1xxx` |
| A2  | Left stick X  | `0b_xxxx_xxxx_xxxx_x1xx` |
| Bb2 | Left stick X  | `0b_xxxx_xxxx_xxxx_xx1x` |
| B2  | Left stick X  | `0b_xxxx_xxxx_xxxx_xxx1` |
|     |               |                          |
| C3  | Left stick X  | `0b_1xxx_xxxx_xxxx_xxxx` |

### Velocity

Velocities also get registered across several of the XInput axes.

| Key     | Input         | Bits                     |
| :--     | :----         | :--:                     |
| 1st key | Left stick X  | `0b_x111_1111_xxxx_xxxx` |
| 2nd key | Left stick Y  | `0b_xxxx_xxxx_x111_1111` |
| 3rd key | Left stick Y  | `0b_x111_1111_xxxx_xxxx` |
| 4th key | Right stick X | `0b_xxxx_xxxx_x111_1111` |
| 5th key | Right stick X | `0b_x111_1111_xxxx_xxxx` |

When a key is pressed or released, if there are 5 or less keys pressed, the velocities of the currently pressed keys will be assigned from left to right into the velocity slots. If there are more than 5 keys pressed, the velocity values do not change from what they were previously, and the velocity of the 6th+ keys pressed will not register until one of the other keys is released, at which point it will register as a velocity of 64.

There is an issue where if a key is pressed, then released only slightly such that it stops registering, then pressed again, it will not register a velocity. This can wreak havoc on velocity recognition if not accounted for. This is most likely due to the nature of how the keys use two contacts for velocity detection, and the keyboard is unable to correctly use a default velocity when only one gets released.

### Other

Overdrive button: Right Stick Y, `0b_xxxx_xxxx_1xxx_xxxx`

Pedal port digital input: Right Stick Y, `0b_1xxx_xxxx_xxxx_xxxx`

Pedal port analog input: Right Stick Y, `0b_x111_1111_xxxx_xxxx`

- This value appears to have an inverted range (0 = max, 127 = min). Not 100% sure on that though as this wasn't tested with an actual analog pedal, instead I used a headphone/microphone splitter on a pair of headphones that have a built-in mic and connected the mic part into the port.
- These bits are all 1 when nothing is plugged in.

Effects touchpad: Present in the first byte beyond the standard XUSB data. Not accessible through standard XInput, requires interfacing with the XUSB driver or something like hacking into the [OpenXinput](https://github.com/Nemirtingas/OpenXinput) project to make an XInput function that provides extended input data.

### As A Struct

```cpp
struct XInputKeytarGamepad
{
    bool dpadUp : 1;
    bool dpadDown : 1;
    bool dpadLeft : 1;
    bool dpadRight : 1;

    bool start : 1;
    bool back : 1;
    bool leftThumbClick : 1;
    bool rightThumbClick : 1;

    bool leftShoulder : 1;
    bool rightShoulder : 1;
    bool guide : 1;
    bool reserved : 1;

    bool a : 1;
    bool b : 1;
    bool x : 1;
    bool y : 1;

    uint8_t keys[3];
    uint8_t velocity[5]; // Also contains last key in top-most bit of velocity[0]
                         // Bit clunky but it's the only practical way to represent this stuff and
                         // be able to work with it programmatically
    uint8_t padding : 7;
    bool overdrive : 1;
    uint8_t pedalAnalog : 7;
    bool pedalDigital : 1;

    // These two fields aren't available via standard XInput.
    // Ignore or remove these fields if you're using it.
    uint8_t touchPad : 7;
    bool padding2 : 1;
};
```

### Pairing Keys and Velocities Programmatically

This code does not attempt to account for the hardware issue described in [Velocity](#velocity). In practice, it's not going to be much of an issue as it's a bit of a deliberate action to make this happen, and working around it requires storing state and overall making things much more complicated for an example.

```cpp
#define KEYS_COUNT 25

void UpdateState(DWORD userIndex)
{
    // Array to copy key velocities into
    uint8_t keyArr[KEYS_COUNT];

    // Modified XINPUT_STATE that uses XInputKeytarGamepad instead of XINPUT_GAMEPAD for better data definitions
    XInputKeytarState state;

    // Get XInput state
    if (XInputGetState(userIndex, (XINPUT_STATE*)&state) != ERROR_SUCCESS)
    {
        return;
    }

    XInputKeytarGamepad gamepad = state.Gamepad;

    // Create a bitmask for the currently pressed keys.
    // When displayed in binary, the 24th bit (starting from 0) is the left-most key,
    // and the 0th bit is the right-most key.
    uint32_t keyBits = (keys[0] << 17) | // 11111111
            (keys[1] << 9) |             //         11111111
            (keys[2] << 1) |             //                 11111111
            ((velocity[0] & 0x80) >> 7); //                         10000000
                                         // xxxxxxxxxxxxxxxxxxxxxxxxx
                                         // 24                      0 

    // Put keys and velocity together
    int pressed = 0; // Number of keys pressed
    int keyMask = 0x01000000; // Bitmask used to test for a specific key
    for (int i = 0; i < KEYS_COUNT; i++)
    {
        // Check if the key is pressed
        if (keyBits & keyMask)
        {
            // There are only 5 key velocities stored
            if (pressed < 5)
            {
                // Retrieve velocity (ranges from 0-127)
                keyArr[i] = (gamepad.velocity[pressed] & 0x7F);
            }
            else
            {
                // We need to assign a default velocity if more than 5 keys are pressed.
                // The keyboard defaults to 64 for keys that were pressed after all velocities filled up
                // and then keys that had velocities registered were released, so we're using the same here.
                // This can be whatever you want it to be though, so long as it's in the range of 0-127.
                keyArr[i] = 64;
            }
        }
        else
        {
            // Key is not pressed
            keyArr[i] = 0;
        }

        // Shift the key mask to the next key
        keyMask >>= 1;
    }
}
```

## References

- https://github.com/bearzly/RockBandPiano
- https://github.com/TheNathannator/RB3_X360_Keyboard
