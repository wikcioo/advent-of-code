import sys

input = open(sys.argv[1]).read().strip().split("\n")
# Rotated 90deg left, north is now west
platform = [list(input[y][x] for y in range(len(input))) for x in range(len(input[0]))]

def get_empty_spot(row, o_idx):
    for i in range(o_idx, -1, -1):
        if row[i] == "#":
            for j in range(i+1, len(row)):
                if row[j] == ".":
                    return j
    for i in range(len(row)):
        if row[i] == ".":
            return i

def p1():
    for row_idx in range(len(platform)):
        for col_idx in range(len(platform[row_idx])):
            if platform[row_idx][col_idx] == "O":
                platform[row_idx][col_idx] = "."
                insert_index = get_empty_spot(platform[row_idx], col_idx)
                platform[row_idx][insert_index] = "O"
    sum = 0
    num_cols = len(platform[0])
    for row in platform:
        for idx, c in enumerate(row):
            if c == "O":
                sum += num_cols - idx
    print("part1:", sum)

p1()
