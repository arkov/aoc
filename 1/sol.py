with open("input.txt", "r") as f:
    lines = [l.strip() for l in f.readlines()]

s = 0
elves = []
for l in lines:
    l = l.strip()
    if len(l) == 0:
        elves.append(s)
        s = 0
        continue
    s += int(l)

if s > 0:
    elves.append(0)
elves.sort()

import functools

print(elves[-1], functools.reduce(lambda x, y: x + y, elves[-3:]))
