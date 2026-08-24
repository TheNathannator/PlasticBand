# General Notes for Guitar Hero Guitars

## Table of Contents

- [Touch/Slider Bar](#touchslider-bar)
  - [World Tour](#world-tour)
  - [Guitar Hero 5](#guitar-hero-5)
    - [Other Notes](#other-notes)
  - [WT and GH5 Side-By-Side](#wt-and-gh5-side-by-side)

## Touch/Slider Bar

The slider bar is a little complicated. Rather than being a bitmask, it uses several discrete numbers for its various states.

The values for guitars of each platform differ in representation due to different bit sizes and resting points:

- Xbox 360 values are a signed 16-bit integer resting at `0x0000`
- PS3 values are an unsigned 8-bit integer resting at `0x80`.
- Wii values are a 5-bit integer resting at `0x0F`.

### World Tour

On the World Tour guitars, only individual segments and adjacent two-segment combinations have values. Additionally, they each occupy a range of values instead of being fully discrete.

The ranges listed here are generous and not exact to measurements, to ensure each permutation can be read reliably.

<details>
<summary>Values organized by frets:</summary>

| Fret    | Xbox 360            | PS3             | Wii
| :---    | :-------            | :--             | :--
| None    | TODO                | `0x70` - `0x8F` | TODO
| | | | |
| `G----` |                     | `0x00` - `0x2F` | |
| `-R---` |                     | `0x40` - `0x5F` | |
| `--Y--` |                     | `0x90` - `0x9F` | |
| `---B-` |                     | `0xB0` - `0xCF` | |
| `----O` |                     | `0xF0` - `0xFF` | |
| | | | |
| `GR---` |                     | `0x30` - `0x3F` | |
| `-RY--` |                     | `0x60` - `0x6F` | |
| `--YB-` |                     | `0xA0` - `0xAF` | |
| `---BO` |                     | `0xD0` - `0xEF` | |

</details>

<details>
<summary>Xbox 360 values: (TODO)</summary>

| Value               | Fret
| :----               | :---

</details>

<details>
<summary>PS3 values:</summary>

| Value           | Fret
| :----           | :---
| `0x00` - `0x2F` | `G----`
| `0x30` - `0x3F` | `GR---`
| `0x40` - `0x5F` | `-R---`
| `0x60` - `0x6F` | `-RY--`
| `0x70` - `0x8F` | None
| `0x90` - `0x9F` | `--Y--`
| `0xA0` - `0xAF` | `--YB-`
| `0xB0` - `0xCF` | `---B-`
| `0xD0` - `0xEF` | `---BO`
| `0xF0` - `0xFF` | `----O`

</details>

<details>
<summary>Wii values: (TODO)</summary>

| Value           | Fret
| :----           | :---

</details>

### Guitar Hero 5

On the Guitar Hero 5 guitars (except on Wii), all possible combinations of segments have a corresponding value. These values are constant and don't vary, there are no ranges involved. The values that exist in the World Tour ranges are backwards-compatible with those ranges.

Wii GH5 guitars do *not* use this format; their sliders function identically to the one on Wii World Tour guitars.

This is also the format used by all Santroller guitars, as it has full fret permutations, and all GH games (including World Tour) support both slider formats.

<details>
<summary>PS3 <-> 360 conversions:</summary>

- 360 -> PS3: `(value ^ 0x80) & 0xFF`
- PS3 -> 360: `((sbyte)(value ^ 0x80) * 257)`

</details>

<details>
<summary>Values organized by frets:</summary>

| Frets   | 360      | PS3
| :----   | :--      | :--
| None    | `0x0000` | `0x80`
| | | |
| `G----` | `0x9495` | `0x15`
| `-R---` | `0xCCCD` | `0x4D`
| `--Y--` | `0x1A1A` | `0x9A`
| `---B-` | `0x4949` | `0xC9`
| `----O` | `0x7F7F` | `0xFF`
| | | |
| `GR---` | `0xAFB0` | `0x30`
| `G-Y--` | `0x1919` | `0x99`
| `G--B-` | `0x4747` | `0xC7`
| `G---O` | `0x7B7B` | `0xFB`
| `-RY--` | `0xE5E6` | `0x66`
| `-R-B-` | `0x4848` | `0xC8`
| `-R--O` | `0x7D7D` | `0xFD`
| `--YB-` | `0x2F2F` | `0xAF`
| `--Y-O` | `0x7E7E` | `0xFE`
| `---BO` | `0x6666` | `0xE6`
| | | |
| `GRY--` | `0xE4E5` | `0x65`
| `GR-B-` | `0x4646` | `0xC6`
| `GR--O` | `0x7979` | `0xF9`
| `G-YB-` | `0x2D2D` | `0xAD`
| `G-Y-O` | `0x7A7A` | `0xFA`
| `G--BO` | `0x6262` | `0xE2`
| `-RYB-` | `0x2E2E` | `0xAE`
| `-RY-O` | `0x7C7C` | `0xFC`
| `-R-BO` | `0x6464` | `0xE4`
| `--YBO` | `0x6565` | `0xE5`
| | | |
| `GRYB-` | `0x2C2C` | `0xAC`
| `GRY-O` | `0x7878` | `0xF8`
| `GR-BO` | `0x6060` | `0xE0`
| `G-YBO` | `0x6161` | `0xE1`
| `-RYBO` | `0x6363` | `0xE3`
| | | |
| `GRYBO` | `0x5F5F` | `0xDF`

</details>

<details>
<summary>Xbox 360 values:</summary>

| Value  | Fret
| :----  | :---
| `0x15` | `G----`
| | |
| `0x30` | `GR---`
| | |
| `0x4D` | `-R---`
| | |
| `0x65` | `GRY--`
| `0x66` | `-RY--`
| | |
| `0x80` | None
| | |
| `0x99` | `G-Y--`
| `0x9A` | `--Y--`
| | |
| `0xAC` | `GRYB-`
| `0xAD` | `G-YB-`
| `0xAE` | `-RYB-`
| `0xAF` | `--YB-`
| `0xC6` | `GR-B-`
| `0xC7` | `G--B-`
| `0xC8` | `-R-B-`
| `0xC9` | `---B-`
| | |
| `0xDF` | `GRYBO`
| `0xE0` | `GR-BO`
| `0xE1` | `G-YBO`
| `0xE2` | `G--BO`
| `0xE3` | `-RYBO`
| `0xE4` | `-R-BO`
| `0xE5` | `--YBO`
| `0xE6` | `---BO`
| | |
| `0xF8` | `GRY-O`
| `0xF9` | `GR--O`
| `0xFA` | `G-Y-O`
| `0xFB` | `G---O`
| `0xFC` | `-RY-O`
| `0xFD` | `-R--O`
| `0xFE` | `--Y-O`
| `0xFF` | `----O`

</details>

<details>
<summary>PS3 values:</summary>

| Value  | Fret
| :----  | :---
| `0x15` | `G----`
| | |
| `0x30` | `GR---`
| | |
| `0x4D` | `-R---`
| | |
| `0x65` | `GRY--`
| `0x66` | `-RY--`
| | |
| `0x80` | None
| | |
| `0x99` | `G-Y--`
| `0x9A` | `--Y--`
| | |
| `0xAC` | `GRYB-`
| `0xAD` | `G-YB-`
| `0xAE` | `-RYB-`
| `0xAF` | `--YB-`
| `0xC6` | `GR-B-`
| `0xC7` | `G--B-`
| `0xC8` | `-R-B-`
| `0xC9` | `---B-`
| | |
| `0xDF` | `GRYBO`
| `0xE0` | `GR-BO`
| `0xE1` | `G-YBO`
| `0xE2` | `G--BO`
| `0xE3` | `-RYBO`
| `0xE4` | `-R-BO`
| `0xE5` | `--YBO`
| `0xE6` | `---BO`
| | |
| `0xF8` | `GRY-O`
| `0xF9` | `GR--O`
| `0xFA` | `G-Y-O`
| `0xFB` | `G---O`
| `0xFC` | `-RY-O`
| `0xFD` | `-R--O`
| `0xFE` | `--Y-O`
| `0xFF` | `----O`

</details>

#### Other Notes

<details>
<summary>Some observations which give insight into how the values are calculated on the guitar's side:</summary>

- Each value seems to correspond with which frets are the top ones being held, with certain fret combos being used as a base value for other combos:

  | Top frets | Base value
  | :-------- | :---------
  | `G----`   | `0x15`
  | `GR---`   | `0x30`
  | `-R---`   | `0x4D`
  | `-RY--`   | `0x66`
  | `--Y--`   | `0x9A`
  | `--YB-`   | `0xAF`
  | `---B-`   | `0xC9`
  | `---BO`   | `0xE6`
  | `----O`   | `0xFF`

  This set of values corresponds directly to the World Tour slider ranges.

- For frets pressed below the top fret combo, a certain amount is subtracted from the base value:

  | Fret    | Amount subtracted |
  | :---    | :---------------- |
  | `G----` | 1                 |
  | `-R---` | 2                 |
  | `--Y--` | 4                 |

  This means that, for example, a GRYB chord will equal `(YB = 0xAF) - (G + R = 3) = 0xAC`, and a GRY chord will equal `(RY = 0x66) - (G = 1) = 0x65`.

</details>

### WT and GH5 Side-By-Side

<details>
<summary>PS3</summary>

| Fret    | World Tour      | GH5
| :---    | :---------      | :--
| None    | `0x70` - `0x8F` | `0x80`
| `G----` | `0x00` - `0x2F` | `0x15`
| `-R---` | `0x40` - `0x5F` | `0x4D`
| `--Y--` | `0x90` - `0x9F` | `0x9A`
| `---B-` | `0xC0` - `0xCF` | `0xC9`
| `----O` | `0xF0` - `0xFF` | `0xFF`
| | | |
| `GR---` | `0x30` - `0x3F` | `0x30`
| `G-Y--` |                 | `0x99`
| `G--B-` |                 | `0xC7`
| `G---O` |                 | `0xFB`
| `-RY--` | `0x60` - `0x6F` | `0x66`
| `-R-B-` |                 | `0xC8`
| `-R--O` |                 | `0xFD`
| `--YB-` | `0xA0` - `0xBF` | `0xAF`
| `--Y-O` |                 | `0xFE`
| `---BO` | `0xD0` - `0xEF` | `0xE6`
| | | |
| `GRY--` |                 | `0x65`
| `GR-B-` |                 | `0xC6`
| `GR--O` |                 | `0xF9`
| `G-YB-` |                 | `0xAD`
| `G-Y-O` |                 | `0xFA`
| `G--BO` |                 | `0xE2`
| `-RYB-` |                 | `0xAE`
| `-RY-O` |                 | `0xFC`
| `-R-BO` |                 | `0xE4`
| `--YBO` |                 | `0xE5`
| | | |
| `GRYB-` |                 | `0xAC`
| `GRY-O` |                 | `0xF8`
| `GR-BO` |                 | `0xE0`
| `G-YBO` |                 | `0xE1`
| `-RYBO` |                 | `0xE3`
| | | |
| `GRYBO` |                 | `0xDF`

</details>
