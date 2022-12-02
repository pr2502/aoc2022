#!/usr/bin/python3

from fileinput import input

high, current = [0,0,0,0], 0
for line in input():
    line = line.strip()
    if line == "":
        high[3], current = current, 0
        high.sort(reverse=True)
    else:
        current += int(line)
print(sum(high[0:3]))
