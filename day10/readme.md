```
(0),  1,  4,  5,  6,  7, 10, 11, 12, 15, 16, 19, (22)
(0),  1,  4,  5,  6,  7, 10,     12, 15, 16, 19, (22)
(0),  1,  4,  5,      7, 10, 11, 12, 15, 16, 19, (22)
(0),  1,  4,  5,      7, 10,     12, 15, 16, 19, (22)
(0),  1,  4,      6,  7, 10, 11, 12, 15, 16, 19, (22)
(0),  1,  4,      6,  7, 10,     12, 15, 16, 19, (22)
(0),  1,  4,          7, 10, 11, 12, 15, 16, 19, (22)
(0),  1,  4,          7, 10,     12, 15, 16, 19, (22)

Convert from absolute joltage to joltage differences for above chain of adapters:
    1   3   1   1   1   3   1   1   3   1   3   3

Whereever two 1 jolt adaters are chained, we can leave out an adapter:
    1   3   1   1   1   3   1       3   1   3   3

-> 2x combinations

Where three 1 jolt adaters are chained, we can leave out an adapter or two:
    1   3   1       1   3   1   1   3   1   3   3
    1   3   1   1       3   1   1   3   1   3   3
    1   3   1           3   1   1   3   1   3   3

-> 4x combinations

Extending this to four 1 jolt adapters in series:
  For a modified example:
    1   3   1   1   1   1   3   1   3   1   3   3

  Combinations:
    1   3   1       1   1   3   1   3   1   3   3
    1   3   1   1       1   3   1   3   1   3   3
    1   3   1   1   1       3   1   3   1   3   3
    1   3   1           1   3   1   3   1   3   3
    1   3   1   1           3   1   3   1   3   3
    1   3   1       1       3   1   3   1   3   3

 -> 7x combinations

```
