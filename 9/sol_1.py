# didnt want to know where the tracer T is and only calculate the steps it can skip.
# when head moves along only one axis, we trace the path along the way without the position the H lands on.
# if it continues to move along the same axis, we will "pick up" the location on the following move.
# does the head change its direction (not the opposite), we dont need to remember the location the Head stood on as we will skip it because we move diagonally.
# when the Head moves in the opposite direction, we wouldnt pick up the temporary location with the aforementioned algorithm.
# if the task were naughty (I didnt check) the Head could switch the moves and always take 1 step with a right angle and we would never pick up any more steps as T.
# that is why implemented the old T location, in order to assert that H and T are always only 1 block away.

def moveHead(d, s):
    global x, y, tx, ty, visited
    steps = None
    match d:
        case 'U':
            steps = [(x, yi) for yi in range(y, y + s + 1)]
            y+=s
        case 'D':
            steps = [(x, yi) for yi in range(y, y - s - 1, -1)]
            y-=s
        case 'L':
            steps = [(xi, y) for xi in range(x, x - s  - 1, -1)]
            x-=s
        case 'R':
            steps = [(xi, y) for xi in range(x, x + s + 1)]
            x+=s
           
    for i in range(len(steps)):
        sx, sy = steps[i]
        if abs(sx - tx) <= 1 and abs(sy - ty) <= 1: 
            continue
        else:
            tx, ty = steps[i-1]
            visited.add((tx, ty))

with open('input.txt', 'r') as f:
    lines = [l.strip() for l in f.readlines()]

visited = {(0,0)}
x, y = 0,0
tx, ty = 0,0

for l in lines:
    d, s = l.split(' ')
    moveHead(d, int(s))
    lastmove = d

print(len(visited))