example_inpt = [
    1721,
    979,
    366,
    299,
    675,
    1456,
]


inpt = example_inpt
for i in range(len(inpt)):
    for j in range(i):
        if inpt[i] + inpt[j] == 2020:
            print(f"part1 test: {inpt[i] * inpt[j]}")


inpt = [int(l) for l in open("1.in").read().splitlines()]

for i in range(len(inpt)):
    for j in range(i):
        if inpt[i] + inpt[j] == 2020:
            print(f"part1: {inpt[i] * inpt[j]}")


inpt = example_inpt
for i in range(len(inpt)):
    for j in range(i):
        for k in range(j):
            if inpt[i] + inpt[j] + inpt[k] == 2020:
                print(f"part2 test: {inpt[i] * inpt[j] * inpt[k]}")

inpt = [int(l) for l in open("1.in").read().splitlines()]
for i in range(len(inpt)):
    for j in range(i):
        for k in range(j):
            if inpt[i] + inpt[j] + inpt[k] == 2020:
                print(f"part2: {inpt[i] * inpt[j] * inpt[k]}")
