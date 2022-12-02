#!/usr/bin/python3

from fileinput import input


WIN = { 'X': 0, 'Y': 3, 'Z': 6 }
SHAPE = {
    'A': { 'X': 3, 'Y': 1, 'Z': 2 },
    'B': { 'X': 1, 'Y': 2, 'Z': 3 },
    'C': { 'X': 2, 'Y': 3, 'Z': 1 },
}

score = 0
for op, _, res, _ in input():
    score += WIN[res] + SHAPE[op][res]
print(score)
