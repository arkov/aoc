with open("input.txt") as f:
    lines = [l.strip() for l in f.readlines()]


def toascii(s: str):
    if s is None:
        return 0
    return (
        ord(s) - ord("a") + 1
        if ord("a") <= ord(s) <= ord("z")
        else ord(s) - ord("A") + 26 + 1
    )


def task1():
    s = 0
    for l in lines:
        a, b = set(l[: len(l) // 2]), set(l[len(l) // 2 :])
        intersec = (a & b).pop()
        s += toascii(intersec)
    print(s)


def task2():
    s = 0
    i = 0
    while i < len(lines):
        a, b, c = [set(lines[i + k]) for k in range(0, 3)]
        intersec = (a & b & c).pop()
        s += toascii(intersec)
        i += 3
    print(s)


task1()
task2()
