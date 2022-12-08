def scorer(trees, element):
    if trees is None:
        return 0
    s = 0
    for t in trees:
        s += 1
        if t >= element:
            break
    return s

import numpy as np # used to conventiently slice in both axes

with open("input.txt", "r") as f:
    lines = f.readlines()
    lines = np.array([[int(c) for c in l.strip()] for l in lines])

visible = set() # thought task 2 would be more complicated, could have been a simple counter
max_x = lines.shape[1] - 1
max_y = lines.shape[0] - 1

besttree = 0
for y in range(lines.shape[0]):
    for x in range(lines.shape[1]):
        t = lines[y, x]
        if (
            (max(lines[0:y, x]) if y != 0 else -1) < t
            or (max(lines[y + 1 :, x]) if y != max_y else -1) < t
            or (max(lines[y, :x]) if x != 0 else -1) < t
            or (max(lines[y, x + 1 :]) if x != max_x else -1) < t
        ):
            visible.add((x, y))

        currenttree = (
            scorer(lines[y - 1 :: -1, x], t)
            * scorer(lines[y + 1 :, x], t)
            * scorer(lines[y, x - 1 :: -1], t)
            * scorer(lines[y, x + 1 :], t)
        )
        besttree = currenttree if currenttree > besttree else besttree

print(f"task1: {len(visible)} -- task2: {besttree}")
