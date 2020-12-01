from itertools import combinations

inpt = [int(l) for l in open("1.in").read().splitlines()]

for a, b, c in combinations(inpt, 3):
    if a + b + c == 2020:
        print(a * b * c)
        break