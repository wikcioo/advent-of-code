import sys
import math

input = open(sys.argv[1]).read().strip().split('\n')

mapping = []
l1 = list(filter(lambda x: x != "", input[0].split(' ')))
l2 = list(filter(lambda x: x != "", input[1].split(' ')))

for i in range(1, len(l1)):
    mapping.append((int(l1[i]), int(l2[i])))

p1 = []
for time, dist in mapping:
    wins = 0
    for i in range(1, time):
        if (time - i) * i > dist:
            wins += 1
    p1.append(wins)

print("part1:", math.prod(p1))

time = int(''.join(input[0].split(':')[1].split()))
dist = int(''.join(input[1].split(':')[1].split()))

p2 = 0
for i in range(1, time):
    if (time - i) * i > dist:
        p2 += 1

print("part2:", p2)
