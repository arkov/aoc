with open("input.txt", "r") as f:
    lines = [l.strip() for l in f.readlines()]

s1, s2 = 0, 0

for l in lines:
    a, b = l.split(",")
    a, b = [int(x) for x in a.split("-")], [int(x) for x in b.split("-")]
    a, b = {x for x in range(a[0], a[1] + 1)}, {x for x in range(b[0], b[1] + 1)}
    if a <= b or b <= a:
        s1 += 1
    s2 += 1 if a & b else 0
print(f"{s1=} {s2=}")
