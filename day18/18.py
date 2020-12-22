import operator

import pyparsing
from pyparsing import (
    Forward,
    Group,
    Literal,
    Suppress,
    Word,
    ZeroOrMore,
    nums,
    ParseResults,
)

example1 = "1 + 2 * 3 + 4 * 5 + 6"
example2 = "1 + (2 * 3) + (4 * (5 + 6))"
example3 = "2 * 3 + (4 * 5)"
# map operator symbols to corresponding arithmetic operations
opn = {
    "+": operator.add,
    "*": operator.mul,
}


result = None

stack = []


def evaluate(parsed: ParseResults) -> int:
    if not stack:
        stack.append(parsed[0])  # value
        stack.append(parsed[1])  # operator

    operation = stack.pop()
    left = stack.pop()

    op = opn[operation]
    right = parsed[2:]
    if len(right) == 1:

    print(f"evalutating {left} {operation} {right}")


plus = Literal("+")
mult = Literal("*")

oper = plus ^ mult

lpar, rpar = map(Suppress, "()")
number = Word(nums)

expr = Forward()
group = Group(lpar + expr + rpar)
atom = oper + (expr ^ group)
expr <<= number + ZeroOrMore(atom)

ex = example2
print(ex)
print(evaluate(expr.parseString(ex)))
