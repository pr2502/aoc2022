#!/usr/bin/python3

from fileinput import input

counter = 0

for line in input():
    a, b = line.strip().split(',')
    a1, a2 = map(int, a.split('-'))
    b1, b2 = map(int, b.split('-'))

    if a1 >= b1 and a2 <= b2 or b1 >= a1 and b2 <= a2:
        counter += 1

print(counter)
