# General Notes for Guitar Hero Guitars

## Pickup Switch (Xbox 360, PS3, Wii)

The pickup switch on the Xbox 360, PS3, and Wii guitars all behave fairly similarly, and can be handled in mostly the same manner. Presumably the PS4 pickup switch might behave in the same manner, but the details for that are currently unknown. The Xbox One pickup switch behaves much more simply and doesn't need any of this logic.

Defining ranges for each of the pickup switch's notches is as simple as dividing the range of the byte into 5 equal ranges: in other words, dividing the raw value by `255 / 5`. These ranges work perfectly for the Xbox 360 guitar, and while they cut it pretty close for the PS3/Wii guitars, they work there as well since those guitars use discrete values instead of a fully analog value. There's just one small catch: with PS3/Wii guitars, the value returned will be `0x7F` after a second or two of the pickup switch being at rest, so that value must be ignored.

In code, this calculation can be implemented as the following:

```cs
// The previous notch value from the guitar
int lastNotch = 0;

int CalculateNotch(byte rawValue)
{
    if (rawValue == 0x7F)
        return lastNotch;

    int notch = Math.Clamp(rawValue / (byte.MaxValue / 5), 0, 4);
    lastNotch = notch;
    return notch;
}
```

### Xbox 360 Notes

For reference, these are the values I recorded using my own Xbox 360 guitar's pickup switch at each of the notches:

| Value        | Notch | Range |
| :----        | :---- | :---- |
| `0x17` (23)  | 1st   | 0.45  |
| `0x4B` (75)  | 2nd   | 1.47  |
| `0x79` (121) | 3rd   | 2.37  |
| `0xAB` (171) | 4th   | 3.35  |
| `0xE0` (224) | 5th   | 4.39  |

If the pickup is between a notch, it will report a value that is somewhere between these values. In practice these values will be very brief, and the value reported at the notches only varies by 3-10.

The values at the notches are very close to those in the table in the PS3/Wii section below, which doesn't seem to be a coincidence. The only exception is with the third notch, and that's due to a technical reason on PS3 that necessitated a different value there (though you could argue the same technical reason exists on Xbox 360).

### PS3/Wii Notes

For reference, the values that the PS3/Wii pickup switch uses are listed below:

| Value        | Notch   | Range |
| :----        | :----   | :---- |
| `0x19` (25)  | 1st     | 0.49  |
| `0x4C` (76)  | 2nd     | 1.49  |
| `0x96` (150) | 3rd     | 2.94  |
| `0xB2` (178) | 4th     | 3.49  |
| `0xE5` (229) | 5th     | 4.49  |
| `0x7F` (127) | At rest | 2.49  |

These values line up almost exactly with the midpoints of each range that the method described above calculates. The only exception is the third notch, which would be `0x7F` if it was following the pattern, but has to be a different value since that's already used for the neutral/inactive state of the pickup switch. The neutral state is done so that the pickup switch isn't constantly registering as the right stick being pressed up or down. The value that replaced it seems to have been chosen with the requirement that it still works with the calculation given above.
