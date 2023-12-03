with open("input.txt", "r") as f:
    lines = [l.strip() for l in f.readlines()]

indexes = [20, 60, 100, 140, 180, 220]
sum = 0
reg = 1
i = 0
output = [["." for _ in range(40)] for _ in range(6)]

for l in lines:
    if i % 40 in range(reg - 1, reg + 2):  # pixels are 0-based
        output[i // 40][i % 40] = "#"

    i += 1
    if i in indexes:
        sum += i * reg

    if l == "noop":
        continue

    else:
        if reg in range(i % 40 - 1, i % 40 + 2):
            output[i // 40][i % 40] = "#"
        i += 1
        if i in indexes:
            sum += i * reg

        cmd, val = l.split(" ")
        old = reg
        reg += int(val)

print(f"t1: {sum}\n----------")
lines = ""
for i in range(6):
    lines += "".join(output[i]) + "\n"

print(f"t2:\n{lines}")
with open("output.txt", "w") as f:
    f.write(lines)
