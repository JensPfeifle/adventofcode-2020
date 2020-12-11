from typing import Tuple, Dict

Coords = Tuple[int, int]
seats: Dict[Coords, str] = {}

with open("./11.in") as f:
    for x, line in enumerate(f.read().splitlines()):
        for y, state in enumerate(line):
            seats[(x, y)] = state


def occupied_neighbors(pos: Coords) -> int:
    neighbors = [
        (pos[0] + 1, pos[1]),
        (pos[0] - 1, pos[1]),
        (pos[0], pos[1] + 1),
        (pos[0], pos[1] - 1),
        (pos[0] + 1, pos[1] + 1),
        (pos[0] - 1, pos[1] + 1),
        (pos[0] + 1, pos[1] - 1),
        (pos[0] - 1, pos[1] - 1),
    ]
    neighbors = list(filter(lambda n: n in seats, neighbors))
    return sum(seats[n] == "#" for n in neighbors)


def pprint(d: Dict[Coords, str]):
    for i in range(10):
        for j in range(10):
            print(d[(i, j)], end="")
        print()


def step() -> bool:
    changed = False
    neighbors = {p: occupied_neighbors(p) for p in seats}
    for p in seats:
        if seats[p] == "L" and neighbors[p] == 0:
            seats[p] = "#"
            changed = True
        if seats[p] == "#" and neighbors[p] >= 4:
            seats[p] = "L"
            changed = True
    return changed


while True:
    pprint(seats)
    print("----------")
    if not step():
        break
num_occupied = list(seats.values()).count("#")
print(num_occupied)
