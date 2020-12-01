from typing import List, Callable


def is_valid(policy, pw):
    appearances, letter = policy.split(" ")
    min_app, max_app = (int(n) for n in appearances.split("-"))
    if pw.count(letter) >= min_app and pw.count(letter) <= max_app:
        return True
    else:
        return False


def is_valid_official(policy, pw):
    positions, letter = policy.split(" ")
    pos1, pos2 = (int(n) for n in positions.split("-"))
    # adjust for 1 indexing
    pos1 -= 1
    pos2 -= 1
    assert pos1 >= 0 and pos2 >= 0
    valid = False
    if pw[pos1] == letter or pw[pos2] == letter:
        valid = True
    if pw[pos1] == letter and pw[pos2] == letter:
        valid = False
    return valid


def count_valid_pw(inpt: str, validator: Callable):
    num_valid = 0
    for entry in inpt.splitlines():
        if not entry:
            continue
        policy, pw = (s.strip() for s in entry.split(":"))
        if validator(policy, pw):
            num_valid += 1
    print(f"{num_valid} passwords are valid")


example = """
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"""

print(f"Example (part1)")
count_valid_pw(example, is_valid)

print(f"Example (part2)")
count_valid_pw(example, is_valid_official)

print(f"Part1")
with open("2.in") as inpt:
    count_valid_pw(inpt.read(), is_valid)

print(f"Part2")
with open("2.in") as inpt:
count_valid_pw(inpt.read(), is_valid_official)
