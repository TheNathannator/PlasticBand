# General Notes for 5-Lane Drumkits

Some general notes that apply to all 5-lane drumkits regardless of platform.

## Using an E-Kit via a 5-Lane Kit

Guitar Hero kits come equipped with a MIDI In port for the purpose of using an e-kit in Guitar Hero games. Assuming the kit uses the same MIDI notes as the drum tuner software does, this port responds to MIDI channel 10, with the following notes mapping to the kit's pads, cymbals, and kick:

| MIDI Note   | Pad    | General MIDI Name |
| :--------   | :--    | :---------------- |
| `0x26` (38) | Red    | Acoustic Snare    |
| `0x2E` (46) | Yellow | Open Hi-Hat       |
| `0x30` (48) | Blue   | Hi Mid Tom        |
| `0x31` (49) | Orange | Crash Cymbal 1    |
| `0x2D` (45) | Green  | Low Tom           |
| `0x24` (36) | Kick   | Bass Drum 1       |

## Configuring Sensitivity via the MIDI In Port

Guitar Hero kits allow the sensitivity of the pads, cymbals, and kick pedal to be configured through the MIDI In port. Activision and RedOctane released a "Drum Tuning Kit" program for this purpose.

### Making Adjustments

#### Control Channels

All adjustments are done through Control Channel messages, using channel 16 (the `0xBF` message byte):

`BF <controller number> <value>`

Each pad and cymbal, along with the kick, has its own controller number:

| Pad    | Number |
| :--    | :----- |
| Red    | `0x68` |
| Yellow | `0x69` |
| Blue   | `0x66` |
| Orange | `0x6A` |
| Green  | `0x67` |
| Kick   | `0x64` |

Additionally, controllers `0x65` and `0x77` are used when saving configurations.

#### Sensitivity Values

The drum tuner software offers a range of 21 sensitivity values, from 0 to 20. When sending the values to the kit, it translates this sensitivity value into a new value based on whether it's a pad, cymbal, or the kick:

```
Sensitivity - 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20
----------------------------------------------------------------------------
Pads        - 28 1E 19 16 14 13 12 11 10 0F 0E 0D 0C 0B 0A 09 08 07 06 05 04
Cymbals     - 3C 35 2E 29 26 22 1E 1B 18 16 14 12 0F 0D 0B 09 08 07 06 05 04
Pedal       - 33 2D 28 24 21 1E 1C 1A 18 16 14 12 10 0E 0C 0A 09 08 07 06 05
```

### Testing Sensitivities

The drum tuner software has a Test button to test the sensitivity of each pad. It does this by sending MIDI notes that correspond to the pads, cymbals, and kick pedal on channel 10 at a velocity of `0x60` (`99 <note> 60`).

The table below is ordered by the sequence in which the program sends the notes, which is in MIDI note order rather than pad order.

| MIDI Note   | Pad    | General MIDI Name |
| :--------   | :--    | :---------------- |
| `0x24` (36) | Kick   | Bass Drum 1       |
| `0x26` (38) | Red    | Acoustic Snare    |
| `0x2D` (45) | Green  | Low Tom           |
| `0x2E` (46) | Yellow | Open Hi-Hat       |
| `0x30` (48) | Blue   | Hi Mid Tom        |
| `0x31` (49) | Orange | Crash Cymbal 1    |

### Saving Configurations

Once all desired adjustments have been made, a sequence must be sent to save the values, otherwise the kit will reset to its previous state when powering off.

The sequence is as follows:

1. `BF 65 03`: Control Channel message on channel 16, to controller `0x65` with the value `0x03`.
2. Wait 750 ms.
3. `BF 77 77`: Control Channel message on channel 16, to controller `0x77` with the value `0x77`.
4. Wait 500 ms.
5. Send test sequence:
   1. `99 24 60`: Note on (channel 10, note 36, velocity 60)
   2. `99 26 60`: Note on (channel 10, note 38, velocity 60)
   3. `99 2D 60`: Note on (channel 10, note 45, velocity 60)
   4. `99 2E 60`: Note on (channel 10, note 46, velocity 60)
   5. `99 30 60`: Note on (channel 10, note 48, velocity 60)
   6. `99 31 60`: Note on (channel 10, note 49, velocity 60)

## References

- The [WiiBrew page](https://wiibrew.org/wiki/Wiimote/Extension_Controllers/Guitar_Hero_World_Tour_(Wii)_Drums) for the Guitar Hero World Tour drums
- My own research done when I initially learned about and hunted down the original software (couldn't test it with actual GH drums but it sent MIDI messages regardless)
- [General MIDI specifications](https://www.midi.org/specifications/midi1-specifications/general-midi-specifications/general-midi-1) for percussion note names
