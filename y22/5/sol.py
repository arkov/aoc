TASK = 2 # or 2
from collections import deque
import re

with open('input.txt', 'r') as f:
    top, bottom = f.read().split('\n\n')
    top, bottom = top.split('\n'), bottom.split('\n')

stacks = dict.fromkeys([i for i in range(1, len(top[-1].split('  ')) + 1)])
for k in stacks:
    stacks[k] = deque()

for l in top[:-1]:
    i = 0
    stack = 1
    while i < len(l):
        if l[i] == ' ':
            stacks[stack]
        else:
            stacks[stack].appendleft(l[i+1])
        stack += 1
        i += 4

for l in bottom:
    params = re.match(r'move (?P<n>\d+?) from (?P<from>\d+) to (?P<to>\d+)', l)
    n, a, b = int(params['n']), int(params['from']), int(params['to'])
    if TASK == 1:
        for _ in range(n):
            if len(stacks[a]) <= 0:
                continue
            s = stacks[a].pop()
            stacks[b].append(s)
    else:
        tmp = deque()
        for _ in range(n):
            if len(stacks[a]) <= 0:
                continue
            tmp.append(stacks[a].pop())
        for _ in range(n):
            if len(tmp) >= 0:
                stacks[b].append(tmp.pop())


out = ""
for _, v in stacks.items():
    out += "" if len(v) <= 0 else v.pop()
print(out)
