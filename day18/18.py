import operator

import pytest
from pyparsing import (
    Forward,
    Group,
    Literal,
    ParseResults,
    Suppress,
    Word,
    ZeroOrMore,
    nums,
)

# map operator symbols to corresponding arithmetic operations
OPS = {
    "+": operator.add,
    "*": operator.mul,
}


def parse(string):
    plus = Literal("+")
    mult = Literal("*")

    oper = plus ^ mult

    lpar, rpar = map(Suppress, "()")
    number = Word(nums)
    expr = Forward()
    group = Group(lpar + expr + rpar)
    atom = oper + (expr ^ group)
    expr <<= (number ^ group) + ZeroOrMore(atom)

    return expr.parseString(string)


def evaluate(parsed: ParseResults) -> int:
    left, op, right = parsed[0:3]

    if not isinstance(left, int):
        left = int(left) if isinstance(left, str) else evaluate(left)
    if not isinstance(right, int):
        right = int(right) if isinstance(right, str) else evaluate(right)

    evaluated = OPS[op](int(left), int(right))
    if len(parsed) > 3:
        return evaluate([evaluated] + parsed[3:])
    return evaluated


# run with pytest 18.py
@pytest.mark.parametrize(
    "expression, expected",
    [
        ("1 + 2", 3),
        ("1 + 2 * 3 + 4 * 5 + 6", 71),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ],
)
def test_parse_and_evaluate(expression, expected):
    parsed = parse(expression)
    assert evaluate(parsed) == expected


if __name__ == "__main__":
    with open("18.in") as f:
        lines = f.read().strip().split("\n")

    print("part1", end="")
    total = 0
    for line in lines:
        v = evaluate(parse(line))
        print(".", end="", flush=True)
        total += v
    print(f"{total}")
