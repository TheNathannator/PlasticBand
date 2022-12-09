# General Notes for 5-Fret Guitars

## Touch/Slider Bar

The slider bar is a little complicated. Rather than being a bitmask, it uses several unique numbers for various states.

The following is a table of all of its possible values, at least for Guitar Hero 5 guitars:

```
None = 0

G = 0x95 (-0x6B)
R = 0xCD (-0x33)
Y = 0x1A
B = 0x49
O = 0x7F

GR = 0xB0 (-0x50)
GY = 0x19
GB = 0x47
GO = 0x7B
RY = 0xE6 (-0x1A)
RB = 0x48
RO = 0x7D
YB = 0x2F
YO = 0x7E
BO = 0x66

GRY = 0xE5 (-0x1B)
GRB = 0x46
GRO = 0x79
GYB = 0x2D
GYO = 0x7A
GBO = 0x62
RYB = 0x2E
RYO = 0x7C
RBO = 0x64
YBO = 0x65

GRYB = 0x2C
GRYO = 0x78
GRBO = 0x60
GYBO = 0x61
RYBO = 0x63

GRYBO = 0x5F
```

Currently, there's no known way to calculate the values programmatically. However, there are some observations by others which could possibly lead to this:

- It helps to order them by signed value rather than by frets:

  ```
  G     = -0x6B (0x95)
  GR    = -0x50 (0xB0)
  R     = -0x33 (0xCD)
  GRY   = -0x1B (0xE5)
  RY    = -0x1A (0xE6)
  GY    = 0x19
  Y     = 0x1A
  GRYB  = 0x2C
  GYB   = 0x2D
  RYB   = 0x2E
  YB    = 0x2F
  GRB   = 0x46
  GB    = 0x47
  RB    = 0x48
  B     = 0x49
  GRYBO = 0x5F
  GRBO  = 0x60
  GYBO  = 0x61
  GBO   = 0x62
  RYBO  = 0x63
  RBO   = 0x64
  YBO   = 0x65
  BO    = 0x66
  GRYO  = 0x78
  GRO   = 0x79
  GYO   = 0x7A
  GO    = 0x7B
  RYO   = 0x7C
  RO    = 0x7D
  YO    = 0x7E
  O     = 0x7F
  ```

- The top nibble of each value seems to correspond with which fret(s) are the top one(s) being held, and the bottom nibble seems to start from a particular number for those top frets:

  | Top frets | First nibble | Starting number for second nibble |
  | :-------- | :----------- | :-------------------------------- |
  | G         | `0x9`        | `0x5`                             |
  | GR        | `0xB`        | `0x0`                             |
  | R         | `0xC`        | `0xD`                             |
  | RY        | `0xE`        | `0x6`                             |
  | Y         | `0x1`        | `0xA`                             |
  | YB        | `0x2`        | `0xF`                             |
  | B         | `0x4`        | `0x9`                             |
  | BO        | `0x6`        | `0x6`                             |
  | O         | `0x7`        | `0xF`                             |

- For frets below the top fret combo, a certain amount is subtracted from the overall value:

  | Fret | Amount subtracted |
  | :--: | :---------------- |
  | G    | 1                 |
  | R    | 2                 |
  | Y    | 4                 |

  This means that, for example, a GRYB chord will equal (YB = `0x2F`) - (G + R = `3`) = `0x2C`. A GRY chord will equal (RY = `0xE6`) - (G = `1`) = `0xE5`.
