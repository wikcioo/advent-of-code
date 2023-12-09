import sys
from typing import List

input = open(sys.argv[1]).read().strip().split("\n")
report: List[List[int]] = [list(map(int, line.split(" "))) for line in input]

def calc_diff(nums: List[int]) -> List[int]:
    diff: List[int] = []
    for i in range(len(nums)-1):
        diff.append(nums[i+1] - nums[i])
    return diff

def get_diff_list(history: List[int], idx: int) -> List[int]:
    last_diffs: List[int] = []
    diff = calc_diff(history)
    last_diffs.append(diff[idx])
    while not all(d == 0 for d in diff):
        diff = calc_diff(diff)
        last_diffs.append(diff[idx])
    return last_diffs

def predict_next_value(history: List[int]) -> int:
    last_diffs = get_diff_list(history, -1)
    for i in range(len(last_diffs)-1, 0, -1):
        last_diffs[i-1] += last_diffs[i]
    return history[-1] + last_diffs[0]

def predict_prev_value(history: List[int]) -> int:
    first_diffs = get_diff_list(history, 0)
    for i in range(len(first_diffs)-1, 0, -1):
        first_diffs[i-1] -= first_diffs[i]
    return history[0] - first_diffs[0]

def p1():
    ps = sum([predict_next_value(history) for history in report])
    print("part1:", ps)

def p2():
    ps = sum([predict_prev_value(history) for history in report])
    print("part2:", ps)
