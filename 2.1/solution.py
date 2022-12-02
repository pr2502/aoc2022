#!/usr/bin/python3

from fileinput import input


WIN = {
    'A': { 'X': 3, 'Y': 6, 'Z': 0 },
    'B': { 'X': 0, 'Y': 3, 'Z': 6 },
    'C': { 'X': 6, 'Y': 0, 'Z': 3 },
}
SHAPE = { 'X': 1, 'Y': 2, 'Z': 3 }

score = 0
for op, _, me, _ in input():
    score += WIN[op][me] + SHAPE[me]
print(score)
