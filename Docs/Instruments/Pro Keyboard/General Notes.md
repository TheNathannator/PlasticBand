# General Notes for Rock Band 3 Keyboards

Some general notes that apply to all RB3 keyboards regardless of platform.

## Key Velocities

When a key is pressed or released, if there are 5 or less keys pressed, the velocities of the currently pressed keys will be assigned from left to right into the velocity slots. If there are more than 5 keys pressed, the velocity values do not change from what they were previously, and the velocity of the 6th+ keys pressed will not register until one of the other keys is released, at which point it will register as a velocity of 64.

There is an issue (at least on my Xbox 360 keyboard) where if a key is pressed, then released only slightly such that it stops registering, then pressed again, it will not register a velocity. This can wreak havoc on velocity recognition if not accounted for. This is most likely due to the nature of how the keys use two contacts for velocity detection, and the keyboard is unable to correctly use a default velocity when only one gets released.

## Pairing Keys and Velocities

This code does not attempt to account for the hardware issue described in [Key Velocities](#key-velocities). In practice, it's not going to be much of an issue as it's a bit of a deliberate action to make this happen, and working around it is likely too complex for just an example.

```cpp
// Number of keys on the keyboard
#define KEY_COUNT 25

// We need to assign a default velocity if more than 5 keys are pressed.
// The keyboard defaults to 64 for keys that were pressed after all velocities filled up
// and then keys that had velocities registered were released, so we're using the same here.
// This is also the velocity recommended by the MIDI standard for velocity-insensitive devices.
#define DEFAULT_VELOCITY 64

// Array to copy key velocities into
uint8_t keyArr[KEY_COUNT];

// Retrieve key and velocity bytes from state
uint8_t[] keys = ...
uint8_t[] velocities = ...

// Create a bitmask for the currently pressed keys.
// When displayed in binary, the 24th bit (starting from 0) is the left-most key,
// and the 0th bit is the right-most key.
uint32_t keyMask =
    (keys[0] << 17) |              // 11111111
    (keys[1] << 9) |               //         11111111
    (keys[2] << 1) |               //                 11111111
    ((velocities[0] & 0x80) >> 7); //                         10000000
                                   // xxxxxxxxxxxxxxxxxxxxxxxxx
                                   // 24                      0

// Put keys and velocity together
int pressed = 0; // Number of keys pressed
int keyBit = 1 << (KEY_COUNT - 1); // Bitmask used to test for a specific key
for (int i = 0; i < KEY_COUNT; i++)
{
    // Check if the key is pressed
    if (keyMask & keyBit)
    {
        // There are only 5 key velocities stored
        if (pressed < 5)
        {
            // Retrieve velocity (ranges from 0-127)
            keyArr[i] = (velocities[pressed] & 0x7F);
        }
        else
        {
            // Use default velocity
            keyArr[i] = DEFAULT_VELOCITY;
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
```
