# General Notes for Guitar Hero Guitars

## Pickup Switch (Xbox 360, PS3, Wii)

The pickup switch on the Xbox 360, PS3, and Wii guitars all behave fairly similarly, and can be handled in mostly the same manner. Presumably the PS4 pickup switch might behave in the same manner, but the details for that are currently unknown. The Xbox One pickup switch behaves much more simply and doesn't need any of this logic.

The switch seems to aim for nominal values that correspond to a function of $\text{raw} = (50 * \text{notch}) + 25$: the first notch rests at ~25, the second at ~75, the third at ~125, and so on. If we apply a threshold of ± 25 to these nominal values, we can calculate the notch by doing a simple divide by 50, with an additional bounds check for the last 5 byte values above 255.

There's one small catch with PS3/Wii guitars: the notch axis will return a value of `0x7F` after a second or two of the pickup switch being at rest, so this value must be ignored. Thankfully, these guitars use fully discrete values for each of the notches, so this value is never used to represent an actual position. The threshold ranges are only required for Xbox 360 guitars, which report a direct analog value instead of discrete values.

In code, this calculation can be implemented as the following:

```cs
// The previous notch value from the guitar
int lastNotch = 0;

int CalculateNotch(int rawValue)
{
    if (rawValue == 0x7F)
        return lastNotch;

    int notch = Math.Clamp(rawValue / 50, 0, 4);
    lastNotch = notch;
    return notch;
}
```

### Xbox 360 Approximate Values

In practice, the value reported for each of the notches on Xbox 360 guitars only varies by 3-10 at most. The threshold calculation suggested above just happens to work with both Xbox 360 and PS3/Wii guitars, so it's the easiest way to handle both of them.

For reference, these are the values I recorded using my own guitar's pickup switch at each of the notches:

| Value  | Notch |
| `0x17` | 1st   |
| `0x4B` | 2nd   |
| `0x79` | 3rd   |
| `0xAB` | 4th   |
| `0xE0` | 5th   |

If the pickup is between a notch, it will report a value that is somewhere between these values.

### PS3/Wii Discrete Values

For reference, the values that the PS3/Wii pickup switch uses are listed below:

| Value  | Notch   |
| `0x19` | 1st     |
| `0x4C` | 2nd     |
| `0x96` | 3rd     |
| `0xB2` | 4th     |
| `0xE5` | 5th     |
| `0x7F` | At rest |
