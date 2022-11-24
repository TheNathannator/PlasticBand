# General Notes for 4-Lane Drumkits

Some general notes that apply to all 4-lane drumkits regardless of platform.

## Deciphering Pads and Cymbals

Decoding the button flags on Rock Band kits is not particularly trivial. There are some pitfalls, complications, and hardware issues that need to be accounted for in order for things to work correctly. I spent a few hours cumulative getting some logic figured out so no one will ever have to do this again, hopefully there are no platform-specific issues or additional hardware bugs to throw a wrench in things lol

This code is built off of observations and assumptions on the Xbox 360 kit. As such, this might not work for the other kit platforms as-is. This also may not work 100% for the MIDI Pro Adapters, there's some known issues with them even on console (though ideally e-kit users would use direct MIDI input where possible). This code is also designed for checking the instantaneous state, it doesn't work well with methods that only give you an input on the frame it happens.

```c
// Bitmask of individual pads/cymbals
enum FourLanePad
{
    None = 0,

    FourLanePad_Red = 0x01,
    FourLanePad_Yellow = 0x02,
    FourLanePad_Blue = 0x04,
    FourLanePad_Green = 0x08,

    FourLaneCymbal_Yellow = 0x20,
    FourLaneCymbal_Blue = 0x40,
    FourLaneCymbal_Green = 0x80
} pads;

// Retrieve each input flag
// The exact details of this varies between platforms
bool red = ...
bool yellow = ...
bool blue = ...
bool green = ...
bool pad = ...
bool cymbal = ...

// Yellow and blue cymbals send d-pad up and down respectively, at least on Xbox 360 kits
// (I sure hope they do on PS3 and Wii as well, otherwise this whole thing falls apart lol)
// The kit only sends either d-pad up or down, not both at the same time (even when hitting both Yc+Bc)
// There also appears to be a hardware bug with the cymbals where the color flag is sometimes cleared before the d-pad is
// (hope this isn't too different between other platforms either...)
bool dpadUp = ...
bool dpadDown = ...

// Pad + cymbal hits can be ambiguous, resolve them first
if (pad && cymbal)
{
    // There's only ambiguity between pad + cymbal hits of different colors, same-color pad + cymbal can be used directly
    int colorCount = 0;
    colorCount += red ? 1 : 0; // Red is technically non-ambiguous but it's simpler to include it in the color count
    colorCount += (yellow || dpadUp) ? 1 : 0; // The d-pad flags are checked here as well due to the hardware bug mentioned earlier
    colorCount += (blue || dpadDown) ? 1 : 0;
    colorCount += (green || !(dpadUp || dpadDown)) ? 1 : 0;

    if (colorCount > 1)
    {
        // The d-pad inputs let us resolve the ambiguity of a pad+cymbal hit
        // Only d-pad is checked here since it is the only unique identifier

        // Yellow
        if (dpadUp)
        {
            pads |= FourLaneCymbal_Yellow;
            yellow = false;
            cymbal = false;
        }

        // Blue
        if (dpadDown)
        {
            pads |= FourLaneCymbal_Blue;
            blue = false;
            cymbal = false;
        }

        // Green
        if (!(dpadUp || dpadDown))
        {
            pads |= FourLaneCymbal_Green;
            green = false;
            cymbal = false;
        }
    }
}

// Now that disambiguation has been applied, we can process things normally

// Check for pad hits
// Rock Band 1 kits don't send the pad or cymbal flags, so we also check if cymbal is not set for compatibility with those
// This does mean that just pressing the face buttons will count as pad hits; this behavior can be observed in Rock Band as well
if (pad || !cymbal)
{
    if (red) pads |= FourLanePad_Red;
    if (yellow) pads |= FourLanePad_Yellow;
    if (blue) pads |= FourLanePad_Blue;
    if (green) pads |= FourLanePad_Green;
}

// Check for cymbal hits
if (cymbal)
{
    if (yellow) pads |= FourLaneCymbal_Yellow;
    if (blue) pads |= FourLaneCymbal_Blue;
    if (green) pads |= FourLaneCymbal_Green;
}
```
