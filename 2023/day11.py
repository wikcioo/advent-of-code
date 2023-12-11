import sys

class Point:
    x: int
    y: int

    def __init__(self, x, y):
        self.x = x
        self.y = y

input = open(sys.argv[1]).read().strip().split("\n")
universe = [list(row) for row in input]

expand_y = [set(universe[row]) == set(".") for row in range(len(universe))]
expand_x = [set(universe[col][row] for col in range(len(universe[0]))) == set(".") for row in range(len(universe))]

galaxies_pos = []
for row_idx, row in enumerate(universe):
    for col_idx, col in enumerate(row):
        if col == "#":
            galaxies_pos.append(Point(col_idx, row_idx))

def dist(p1, p2, expand_value):
    d = 0
    x_dir = 1 if p1.x <= p2.x else -1
    for x in range(p1.x, p2.x, x_dir):
        if expand_x[x]:
            d += expand_value
        d += 1
    y_dir = 1 if p1.y <= p2.y else -1
    for y in range(p1.y, p2.y, y_dir):
        if expand_y[y]:
            d += expand_value
        d += 1
    return d

def p1():
    sum = 0
    for i in range(len(galaxies_pos) - 1):
        for j in range(i + 1, len(galaxies_pos)):
            sum += dist(galaxies_pos[i], galaxies_pos[j], 1)
    print("part1:", sum)

def p2():
    sum = 0
    for i in range(len(galaxies_pos) - 1):
        for j in range(i + 1, len(galaxies_pos)):
            sum += dist(galaxies_pos[i], galaxies_pos[j], 1000000-1)
    print("part2:", sum)
