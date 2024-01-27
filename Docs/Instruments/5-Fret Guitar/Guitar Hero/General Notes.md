# General Notes for Guitar Hero Guitars

## Table of Contents

- [Touch/Slider Bar](#touchslider-bar)
  - [World Tour](#world-tour)
    - [Possibilities Ordered by Fret](#possibilities-ordered-by-fret)
    - [Possibilities Ordered by Value](#possibilities-ordered-by-value)
  - [Guitar Hero 5](#guitar-hero-5)
    - [Possibilities Ordered by Fret](#possibilities-ordered-by-fret-1)
    - [Possibilities Ordered by Value](#possibilities-ordered-by-value-1)
    - [Other Notes](#other-notes)
  - [WT and GH5 Side-By-Side](#wt-and-gh5-side-by-side)

## Touch/Slider Bar

The slider bar is a little complicated. Rather than being a bitmask, it uses several discrete numbers for its various states.

The values for PS3 and Xbox 360 guitars differ in representation due to different bit sizes and stick resting points between the two input formats. PS3 values are an unsigned byte resting at around `0x80`, while Xbox 360 values are a signed short resting at `0x0000`.

Only the PS3 values will be listed here since they make more sense when ordered sequentially. For Xbox 360 guitars, the values should be converted first before use for simplicity. The conversions differ slightly between models due to different semantics for the values.

### World Tour

On the World Tour guitars, only individual segments and adjacent two-segment combinations have values. The values also vary, being ranges instead of just a single one. The ranges listed here are generous and not exact, as the ranges vary somewhat between guitars for different platforms.

PS3 <-> 360 conversions:

- 360 -> PS3: `(value >> 8) ^ 0x80`
- PS3 -> 360: `(value ^ 0x80) << 8`

#### Possibilities Ordered by Fret

| Fret | Value range
| :--- | :----------
| None | `0x70` - `0x8F`
| |
| G    | `0x00` - `0x2F`
| R    | `0x40` - `0x5F`
| Y    | `0x90` - `0x9F`
| B    | `0xB0` - `0xCF`
| O    | `0xF0` - `0xFF`
| |
| GR   | `0x30` - `0x3F`
| RY   | `0x60` - `0x6F`
| YB   | `0xA0` - `0xAF`
| BO   | `0xD0` - `0xEF`

#### Possibilities Ordered by Value

| Fret | Value range
| :--- | :----------
| G    | `0x00` - `0x2F`
| GR   | `0x30` - `0x3F`
| R    | `0x40` - `0x5F`
| RY   | `0x60` - `0x6F`
| None | `0x70` - `0x8F`
| Y    | `0x90` - `0x9F`
| YB   | `0xA0` - `0xAF`
| B    | `0xB0` - `0xCF`
| BO   | `0xD0` - `0xEF`
| O    | `0xF0` - `0xFF`

### Guitar Hero 5

On the Guitar Hero 5 guitars, all possible combinations of segments have a corresponding value. These values are constant and don't vary, there are no ranges involved. The values that exist in the World Tour ranges are backwards-compatible with those ranges.

This is also the format used by all Santroller guitars, as it has full fret permutations, and all GH games support both slider formats including World Tour.

PS3 <-> 360 conversions:

- 360 -> PS3: `(value ^ 0x80) & 0xFF`
- PS3 -> 360: `-((sbyte)(value ^ 0x80) * -257)`

#### Possibilities Ordered by Fret

| Fret  | Values
| :---  | :-----
| None  | `0x80`
| G     | `0x15`
| R     | `0x4D`
| Y     | `0x9A`
| B     | `0xC9`
| O     | `0xFF`
| |
| GR    | `0x30`
| GY    | `0x99`
| GB    | `0xC7`
| GO    | `0xFB`
| RY    | `0x66`
| RB    | `0xC8`
| RO    | `0xFD`
| YB    | `0xAF`
| YO    | `0xFE`
| BO    | `0xE6`
| |
| GRY   | `0x65`
| GRB   | `0xC6`
| GRO   | `0xF9`
| GYB   | `0xAD`
| GYO   | `0xFA`
| GBO   | `0xE2`
| RYB   | `0xAE`
| RYO   | `0xFC`
| RBO   | `0xE4`
| YBO   | `0xE5`
| |
| GRYB  | `0xAC`
| GRYO  | `0xF8`
| GRBO  | `0xE0`
| GYBO  | `0xE1`
| RYBO  | `0xE3`
| |
| GRYBO | `0xDF`

#### Possibilities Ordered by Value

These are divided with accordance to [the notes written below](#other-notes).

| Fret  | Values
| :---  | :-----
| G     | `0x15`
| |
| GR    | `0x30`
| |
| R     | `0x4D`
| |
| GRY   | `0x65`
| RY    | `0x66`
| |
| None  | `0x80`
| |
| GY    | `0x99`
| Y     | `0x9A`
| |
| GRYB  | `0xAC`
| GYB   | `0xAD`
| RYB   | `0xAE`
| YB    | `0xAF`
| |
| GRB   | `0xC6`
| GB    | `0xC7`
| RB    | `0xC8`
| B     | `0xC9`
| |
| GRYBO | `0xDF`
| GRBO  | `0xE0`
| GYBO  | `0xE1`
| GBO   | `0xE2`
| RYBO  | `0xE3`
| RBO   | `0xE4`
| YBO   | `0xE5`
| BO    | `0xE6`
| |
| GRYO  | `0xF8`
| GRO   | `0xF9`
| GYO   | `0xFA`
| GO    | `0xFB`
| RYO   | `0xFC`
| RO    | `0xFD`
| YO    | `0xFE`
| O     | `0xFF`

#### Other Notes

Some observations which give insight into how the values are calculated on the guitar's side:

- Each value seems to correspond with which frets are the top ones being held, with certain fret combos being used as a base value for other combos:

  | Top frets | Base value
  | :-------- | :---------
  | G         | `0x15`
  | GR        | `0x30`
  | R         | `0x4D`
  | RY        | `0x66`
  | Y         | `0x9A`
  | YB        | `0xAF`
  | B         | `0xC9`
  | BO        | `0xE6`
  | O         | `0xFF`

  This set of values corresponds directly to the World Tour slider ranges.

- For frets pressed below the top fret combo, a certain amount is subtracted from the base value:

  | Fret | Amount subtracted |
  | :--: | :---------------- |
  | G    | 1                 |
  | R    | 2                 |
  | Y    | 4                 |

  This means that, for example, a GRYB chord will equal (YB = `0xAF`) - (G + R = `3`) = `0xAC`, and a GRY chord will equal (RY = `0x66`) - (G = `1`) = `0x65`.

### WT and GH5 Side-By-Side

| Fret  | World Tour      | GH5
| :---  | :---------      | :--
| None  | `0x70` - `0x8F` | `0x80`
| G     | `0x00` - `0x2F` | `0x15`
| R     | `0x40` - `0x5F` | `0x4D`
| Y     | `0x90` - `0x9F` | `0x9A`
| B     | `0xC0` - `0xCF` | `0xC9`
| O     | `0xF0` - `0xFF` | `0xFF`
| | |
| GR    | `0x30` - `0x3F` | `0x30`
| GY    |                 | `0x99`
| GB    |                 | `0xC7`
| GO    |                 | `0xFB`
| RY    | `0x60` - `0x6F` | `0x66`
| RB    |                 | `0xC8`
| RO    |                 | `0xFD`
| YB    | `0xA0` - `0xBF` | `0xAF`
| YO    |                 | `0xFE`
| BO    | `0xD0` - `0xEF` | `0xE6`
| | |
| GRY   |                 | `0x65`
| GRB   |                 | `0xC6`
| GRO   |                 | `0xF9`
| GYB   |                 | `0xAD`
| GYO   |                 | `0xFA`
| GBO   |                 | `0xE2`
| RYB   |                 | `0xAE`
| RYO   |                 | `0xFC`
| RBO   |                 | `0xE4`
| YBO   |                 | `0xE5`
| | |
| GRYB  |                 | `0xAC`
| GRYO  |                 | `0xF8`
| GRBO  |                 | `0xE0`
| GYBO  |                 | `0xE1`
| RYBO  |                 | `0xE3`
| | |
| GRYBO |                 | `0xDF`
| GRBO  |                 | `0xE0`
| GYBO  |                 | `0xE1`
| RYBO  |                 | `0xE3`
| | |
| GRYBO |                 | `0xDF`
