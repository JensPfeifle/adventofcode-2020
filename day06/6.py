from collections import Counter
from itertools import chain
from typing import Iterable, Iterator, List


def iter_line_groups(inpt: str) -> Iterator[List[str]]:
    line_group: List[str] = []
    for line in inpt.strip().splitlines():
        stripped_line = line.strip()
        if stripped_line:
            line_group.append(stripped_line)
        else:
            yield line_group
            line_group = []
    # yield final group
    yield line_group


def part1(groups: Iterable[List[str]]) -> int:
    # fmt:off
    return sum(
        len(set(chain.from_iterable(group_answers)))
        for group_answers in groups
    )
    # fmt:on


def part2(groups: Iterable[List[str]]) -> int:
    total = 0
    for group_answers in groups:
        for count in Counter(chain.from_iterable(group_answers)).values():
            if count == len(group_answers):
                total += 1
    return total


example = """
abc

a
b
c

ab
ac

a
a
a
a

b
"""

with open("6.in") as f:
    inpt = f.read()

print("example:", part1(iter_line_groups(example)))
print("part1:", part1(iter_line_groups(inpt)))
print("example:", part2(iter_line_groups(example)))
print("part2:", part2(iter_line_groups(inpt)))
