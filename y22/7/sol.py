from collections import deque


def addFolder(path: deque, folder: str):
    if len(path) == 0:
        root[folder] = {}
        return
    dir = traverse(path)
    dir[folder] = {}
    return dir


def addFile(path: deque, file: str, size: int):
    if len(path) == 0:
        root[file] = size
        return
    dir = traverse(path)
    dir[file] = size


def sumFiles_1(path: dict):
    s = 0
    dirslessthan100k = 0
    for k, v in path.items():
        if type(v) is dict:
            FOLDERSIZE, v_dirslessthan100k = sumFiles_1(v)
            s += FOLDERSIZE
            dirslessthan100k += v_dirslessthan100k
            if FOLDERSIZE <= 100000:
                dirslessthan100k += FOLDERSIZE
        else:
            s += v
    return s, dirslessthan100k


def sumFiles_2(path: dict, todel: int, maxsize: int):
    s = 0
    minfound = maxsize

    for k, v in path.items():
        if type(v) is dict:
            FOLDERSIZE, minimumfolder = sumFiles_2(
                v, todel, maxsize
            )  # FOLDERSIZE obviously >= minimumfolder, unless minimumfolder is leaf
            s += FOLDERSIZE
            if minimumfolder >= todel:
                minfound = min(minimumfolder, minfound)
        else:
            s += v
    return s, min(s, minfound)


def traverse(path: deque):
    dir = root
    while len(path) > 0:
        dir = dir[path.popleft()]
    return dir  # return pointer to folder one level above path


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        lines = [l.strip() for l in f.readlines()]

    root = {}
    pwd = deque()

    storageused = 0
    for l in lines:
        if l.find("$ cd") >= 0:
            folder = l.split(" ")[-1]
            if folder == "/":
                pwd = deque()
            elif folder == "..":
                pwd.pop()
            else:
                s = pwd.copy()
                pwd.append(folder)

        elif l.find("dir") >= 0:
            dir = l.split(" ")[1]
            addFolder(pwd.copy(), dir)

        elif l.find("$ ls") >= 0:
            continue

        else:  # SIZE FILENAME
            size, file = l.split(" ")
            addFile(pwd.copy(), file, int(size))
            storageused += int(size)

    storage_used, hundredk = sumFiles_1(root)
    free_storage = 70000000 - storage_used
    needed_storage = 30000000 - free_storage
    _, t2 = sumFiles_2(root, needed_storage, storage_used)

    print(hundredk, t2)
