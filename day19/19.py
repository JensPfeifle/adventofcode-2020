"""
My input contains only 3 rule forms:

34: 86 82 | 99 127
88: 93 17 -> matches 2*n chars
99: "a" -> matches one char

(except for rule 0 of the example)

"""
from __future__ import annotations

from typing import *

with open("example.in") as f:
    ipt = f.read()


RULES = {}
rules, messages = ipt.split("\n\n")
for raw in rules.splitlines():
    rule_id, rule_str = raw.split(": ")
    RULES[rule_id] = rule_str


def try_match(rule: str, ipt: str) -> bool:
    # 99: "a" -> matches one char
    if '"' in rule:
        return rule[1] == ipt
    # 88: 93 17 -> matches 2*n chars
    if "|" not in rule:
        a, b = rule.split(" ")
        rule_a, rule_b = RULES[a], RULES[b]

    # 34: 86 82 | 99 127


root = RULES["0"].split(" ")
print(root)
for message in messages:

    pass
