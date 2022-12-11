from collections import deque


def calc(old, op, n):
    if op == "*" and n == "old":
        n = old
    match (op):
        case "+":
            return old + n
        case "*":
            return old * n
        case other:
            print("not good")


with open("input.txt", "r") as f:
    lines = "".join(f.readlines()).split("\n\n")

monkeylines = [[l.strip() for l in monkey.split("\n")] for monkey in lines]
monkeys = {}


for m in monkeylines:
    i = int(m[0].split(" ")[-1].replace(":", "")[0])
    items = deque([int(i) for i in m[1].split(":")[-1].split(",")])
    op, n = m[2].split(" ")[-2:]
    n = int(n) if n != "old" else "old"
    divisor = int(m[3].split(" ")[-1])
    m1, m2 = [int(l.split(" ")[-1]) for l in m[4:]]
    monkeys[i] = {
        "items": items,
        "op": op,
        "n": n,
        "div": divisor,
        "next": {True: m1, False: m2},
        "inspections": 0,
    }

for i in range(20):
    for k, v in monkeys.items():
        for item in v["items"].copy():
            worry = calc(item, v["op"], v["n"]) // 3
            v["items"].popleft()
            monkeys[v["next"][worry % v["div"] == 0]]["items"].append(worry)
            monkeys[k]["inspections"] += 1


tmp = [v["inspections"] for _, v in monkeys.items()]
tmp.sort(reverse=True)
print(tmp[0] * tmp[1])
