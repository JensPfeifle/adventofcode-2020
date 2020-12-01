from collections import namedtuple
from typing import List, Tuple

Position = namedtuple("Position", "x,y")


def count_trees(map_: str, slope: Tuple[int, int]) -> int:
    pos = Position(0, 0)
    rows = map_.strip().splitlines()
    trees_encountered = 0
    while pos[1] < len(rows):
        tile = rows[pos.y][pos.x % len(rows[0])]
        if tile == "#":  # tree
            trees_encountered += 1
        pos = Position(pos.x + slope[0], pos.y + slope[1])
    return trees_encountered


example = """
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"""

print("example:", count_trees(example, (3, 1)))
print("part1:", count_trees(open("3.in").read(), (3, 1)))

acc = 1
with open("3.in") as f:
    inpt = f.read()
    for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        acc *= count_trees(inpt, slope)
print("part2:", acc)
