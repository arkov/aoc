# didnt want to know where the tracer T is and only calculate the steps it can skip.
# when head moves along only one axis, we trace the path along the way without the position the H lands on.
# if it continues to move along the same axis, we will "pick up" the location on the following move.
# does the head change its direction (not the opposite), we dont need to remember the location the Head stood on as we will skip it because we move diagonally.
# when the Head moves in the opposite direction, we wouldnt pick up the temporary location with the aforementioned algorithm.
# if the task were naughty (I didnt check) the Head could switch the moves and always take 1 step with a right angle and we would never pick up any more steps as T.
# that is why implemented the old T location, in order to assert that H and T are always only 1 block away.
def moveHead(d, s):
    global x, y, visited_1, visited_9
    steps = None
    match d:
        case "U":
            steps = [(x, yi) for yi in range(y, y + s + 1)]
            y += s
        case "D":
            steps = [(x, yi) for yi in range(y, y - s - 1, -1)]
            y -= s
        case "L":
            steps = [(xi, y) for xi in range(x, x - s - 1, -1)]
            x -= s
        case "R":
            steps = [(xi, y) for xi in range(x, x + s + 1)]
            x += s

    for i in range(1, len(steps)):
        locations[0] = steps[i]
        for r in range(1, 10):
            sx, sy = locations[r - 1]
            tx, ty = locations[r]
            dx = sx - tx
            dy = sy - ty

            if abs(dx) <= 1 and abs(dy) <= 1:
                continue

            locations[r] = (
                tx + (dx / abs(dx) if dx != 0 else 0),
                ty + (dy / abs(dy) if dy != 0 else 0),
            )

            if r == 1:
                visited_1.add(locations[r])

            if r == 9:
                visited_9.add(locations[r])


with open("input.txt", "r") as f:
    lines = [l.strip() for l in f.readlines()]

x, y = 0, 0
visited_1 = {(0, 0)}
visited_9 = {(0, 0)}
locations = {r: (0, 0) for r in range(1, 10)}

for l in lines:
    d, s = l.split(" ")
    moveHead(d, int(s))

print(len(visited_1), len(visited_9))
