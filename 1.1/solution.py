#!/usr/bin/python3

from fileinput import input

high, current = 0, 0
for line in input():
    line = line.strip()
    if line == "":
        high, current = max(high, current), 0
    else:
        current += int(line)
print(high)
