import sys
from typing import List
from pprint import pprint

input = open(sys.argv[1]).read().strip().split("\n")

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y
    
    def __ne__(self, other):
        return not self.__eq__(other)

s = ()
maze: List[List[str]] = []
for idx, line in enumerate(input):
    if "S" in line:
        s = Point(line.index("S"), idx)
    maze.append(list(line))

maze_copy = [['0' for _ in range(len(maze[0]))] for _ in range(len(maze))]

# UP is -1, DOWN is 1
pipes = {
    "|": ((0,-1),(0,1)),
    "-": ((-1,0),(1,0)),
    "L": ((0,-1),(1,0)),
    "J": ((0,-1),(-1,0)),
    "7": ((-1,0),(0,1)),
    "F": ((1,0),(0,1))
}

class Position:
    def __init__(self, curr, prev):
        self.curr = curr
        self.prev = prev
        self.steps = 1

    def next(self):
        initial = Point(self.curr.x, self.curr.y)
        c = maze[self.curr.y][self.curr.x]
        dir1, dir2 = pipes[c]
        if self.curr.x + dir1[0] == self.prev.x and self.curr.y + dir1[1] == self.prev.y:
            self.curr.x += dir2[0]
            self.curr.y += dir2[1]
        else:
            self.curr.x += dir1[0]
            self.curr.y += dir1[1]
        self.prev = initial
        self.steps += 1

def p1():
    pos1 = Position(Point(s.x - 1, s.y), Point(s.x, s.y))
    pos2 = Position(Point(s.x, s.y + 1), Point(s.x, s.y))
    while pos1.curr != pos2.curr:
        pos1.next()
        pos2.next()
    print("part1:", pos1.steps)

p1()
