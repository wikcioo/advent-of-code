import sys
import math

input = open(sys.argv[1]).read().strip().split("\n")

directions = input[0]
dirs_len = len(directions)
mapping = {}

for line in input[2:]:
    start_node, end_nodes = line.split(" = ")
    en1, en2 = end_nodes.strip("()").split(", ")
    mapping[start_node] = (en1, en2)

def p1():
    p1 = 0
    current_node = "AAA"
    while current_node != "ZZZ":
        dir = directions[p1 % dirs_len]
        current_node = mapping[current_node][dir == "R"]
        p1 += 1
    print("part1:", p1)

def nr_steps(start):
    n = 0
    while not start.endswith("Z"):
        dir = directions[n % dirs_len]
        start = mapping[start][dir == "R"]
        n += 1
    return n

def p2():
    starting_nodes = [key for key in mapping.keys() if key.endswith("A")]
    steps = [nr_steps(node) for node in starting_nodes]
    print("part2:", math.lcm(*steps))
