# General Notes for 4-Lane Drumkits

Some general notes that apply to all 4-lane drumkits regardless of platform.

## Deciphering Pads and Cymbals

(This section does not apply to Xbox One or PS4 drumkits.)

Decoding the button flags on Rock Band kits is not particularly trivial. There are some pitfalls, complications, and hardware issues that need to be accounted for in order for things to work correctly.

The code below is based on a number of observations about how exactly the drumkits sends input, and should hopefully account for all important hardware issues. If you are using the code below and encounter issues, please feel free to open an issue for it and include report data logs to help troubleshoot the issue.

This code is designed for checking the instantaneous state, it doesn't work well with methods that only give you an input on the frame it happens. There's some examples and comments on how to filter the instantaneous state further if that's desired.

```cpp
// Bitmask of individual pads/cymbals
typedef enum
{
    None = 0,

    FourLanePad_Red = 0x01,
    FourLanePad_Yellow = 0x02,
    FourLanePad_Blue = 0x04,
    FourLanePad_Green = 0x08,

    FourLaneCymbal_Yellow = 0x20,
    FourLaneCymbal_Blue = 0x40,
    FourLaneCymbal_Green = 0x80
} FourLanePad;

// The previously-calculated pads.
static FourLanePad previousPads;

FourLanePad GetCurrentPads(uint8_t[] report)
{
    // The currently-calculated pads.
    FourLanePad pads;

    // Retrieve each input flag
    // The exact details of this varies between models of drumkit,
    // see the docs for each model for exact details on the inputs.
    bool red = ...
    bool yellow = ...
    bool blue = ...
    bool green = ...
    bool pad = ...
    bool cymbal = ...

    // Yellow and blue cymbals send d-pad up and down respectively, at least on Xbox 360 kits
    // The kit only sends either d-pad up or down, not both at the same time (even when hitting both Yc+Bc)
    bool dpadUp = ...
    bool dpadDown = ...

    // Pad + cymbal hits can be ambiguous, resolve them first
    if (pad && cymbal)
    {
        // There's only ambiguity between pad + cymbal hits of different colors, same-color pad + cymbal can be used directly
        int colorCount = 0;
        colorCount += red ? 1 : 0; // Red is technically non-ambiguous but it's simpler to include it in the color count
        colorCount += (yellow || dpadUp) ? 1 : 0; // The d-pad flags are checked here as well due to the hardware bug mentioned below
        colorCount += (blue || dpadDown) ? 1 : 0;
        colorCount += (green || !(dpadUp || dpadDown)) ? 1 : 0;

        if (colorCount > 1)
        {
            // The d-pad inputs let us resolve the ambiguity of a pad+cymbal hit
            // Only d-pad is checked here since it is the only unique identifier due to hardware bugs
            // (sometimes the color flag is released before the d-pad input is, particularly at high polling rates)

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

    // If you want to see which pads have changed state, you can XOR the decoded pads with the previous pads.
    FourLanePad changedPads = pads ^ previousPads;
    // If you want only new pads, you can additionally mask out the current pads from the changed pads.
    FourLanePad newPads = changedPads & pads;
    // For pads that are no longer active, you can negate the calculated pads before masking.
    FourLanePad removedPads = changedPads & ~pads;

    // Store the current pads as the now-previous now that processing is done
    previousPads = pads;

    return pads;
}
```
