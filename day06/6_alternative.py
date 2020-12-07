# source: https://www.reddit.com/r/adventofcode/comments/k7ndux/2020_day_06_solutions/geu5067/
# a bit slower than my solution, but the .split("\n\n") is nice

# # Part 1
groups = open("6.in").read().split("\n\n")


def count_answers(group):
    return len(set(group.replace("\n", "")))


print(sum(count_answers(g) for g in groups))

# Part 2

groups = open("6.in").read().split("\n\n")


def count_answers_pt2(group):
    questions = set(group.replace("\n", ""))
    answers = group.split()
    return sum(all(q in a for a in answers) for q in questions)


print(sum(count_answers_pt2(g) for g in groups))
