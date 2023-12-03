def task1():
    with open("input.txt", "r") as f:
        text = f.readlines()
    # move (points, winning, losing)
    gamelogic = {
        "rock": (1, "scissors", "paper"),
        "paper": (2, "rock", "scissors"),
        "scissors": (3, "paper", "rock"),
    }
    translate = {
        "A": "rock",
        "B": "paper",
        "C": "scissors",
        "X": "rock",
        "Y": "paper",
        "Z": "scissors",
    }

    s = 0
    for line in text:
        line = line.upper().strip().split(" ")
        me, elve = translate[line[1]], translate[line[0]]
        round = 3
        if gamelogic[me][1] == elve:
            round = 6
        elif gamelogic[me][2] == elve:
            round = 0
        s += round + gamelogic[me][0]
    print(s)


def task2():
    with open("input.txt", "r") as f:
        text = f.readlines()
    gamelogic = {
        "rock": (1, "scissors", "paper"),
        "paper": (2, "rock", "scissors"),
        "scissors": (3, "paper", "rock"),
    }
    translate = {"A": "rock", "B": "paper", "C": "scissors"}
    # x = win, y = draw, z = lose

    s = 0
    for line in text:
        line = line.upper().strip().split(" ")
        cond, elve = line[1], translate[line[0]]
        match cond:
            case "Z":
                point = gamelogic[gamelogic[elve][2]][0]
                round = 6
            case "Y":
                point = gamelogic[elve][0]
                round = 3
            case "X":
                point = gamelogic[gamelogic[elve][1]][0]
                round = 0
        s += round + point
    print(s)


task1()
task2()
