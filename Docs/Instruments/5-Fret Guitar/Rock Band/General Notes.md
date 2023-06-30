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

### Xbox 360 Approximate Values

In practice, the value reported for each of the notches on Xbox 360 guitars only varies by 3-10 at most. The threshold calculation suggested above just happens to work with both Xbox 360 and PS3/Wii guitars, so it's the easiest way to handle both of them.

For reference, these are the values I recorded using my own guitar's pickup switch at each of the notches:

| Value  | Notch |
| :----  | :---- |
| `0x17` | 1st   |
| `0x4B` | 2nd   |
| `0x79` | 3rd   |
| `0xAB` | 4th   |
| `0xE0` | 5th   |

If the pickup is between a notch, it will report a value that is somewhere between these values.

### PS3/Wii Discrete Values

For reference, the values that the PS3/Wii pickup switch uses are listed below:

| Value  | Notch   |
| :----  | :----   |
| `0x19` | 1st     |
| `0x4C` | 2nd     |
| `0x96` | 3rd     |
| `0xB2` | 4th     |
| `0xE5` | 5th     |
| `0x7F` | At rest |
