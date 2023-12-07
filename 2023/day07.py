import sys
from collections import defaultdict
from functools import cmp_to_key

JOKER_ENABLED: bool

input = open(sys.argv[1]).read().strip().split('\n')

strength_order1 = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
strength_order2 = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']


def get_hand_value1(cards: str):
    hand = defaultdict(int)
    for card in cards:
        hand[card] += 1
    
    unique_cards = len(hand.keys())
    if unique_cards == 1:
        return 7
    elif unique_cards == 2:
        if 4 in hand.values():
            return 6
        else:
            return 5
    elif unique_cards == 3:
        if 3 in hand.values():
            return 4
        else:
            return 3
    elif unique_cards == 4:
        return 2
    elif unique_cards == 5:
        return 1
    return 0


def get_hand_value2(cards: str):
    hand = defaultdict(int)
    for card in cards:
        hand[card] += 1
    
    unique_cards = len(hand.keys())
    joker_count = hand['J']
    if unique_cards == 1:
        return 7
    elif unique_cards == 2:
        if 4 in hand.values():
            return min(7, 6 + joker_count)
        else:
            return min(7, 5 + joker_count)
    elif unique_cards == 3:
        if 3 in hand.values():
            if joker_count > 0:
                return 6
            else:
                return 4
        else:
            if joker_count == 2:
                return 6
            elif joker_count == 1:
                return 5
            else:
                return 3
    elif unique_cards == 4:
        if joker_count > 0:
            return 4
        else:
            return 2
    elif unique_cards == 5:
        if joker_count > 0:
            return 2
        else:
            return 1
    return 0


def compare_hand_values(hand1: str, hand2: str):
    hand1 = hand1.split(' ')[0]
    hand2 = hand2.split(' ')[0]

    value_func = get_hand_value2 if JOKER_ENABLED else get_hand_value1
    v1 = value_func(hand1)
    v2 = value_func(hand2)

    strength_list = strength_order2 if JOKER_ENABLED else strength_order1

    if v1 < v2:
        return -1
    elif v1 > v2:
        return 1
    else:
        for c1, c2 in zip(hand1, hand2):
            if c1 != c2:
                i1 = strength_list.index(c1)
                i2 = strength_list.index(c2)
                if i1 > i2:
                    return -1
                elif i1 < i2:
                    return 1
        return 0


JOKER_ENABLED = False
sorted_input = sorted(input, key=cmp_to_key(compare_hand_values))

p1 = 0
for index, line in enumerate(sorted_input):
    p1 += int(line.split(' ')[1]) * (index + 1)

print("part1:", p1)


JOKER_ENABLED = True
sorted_input = sorted(input, key=cmp_to_key(compare_hand_values))

p2 = 0
for index, line in enumerate(sorted_input):
    p2 += int(line.split(' ')[1]) * (index + 1)

print("part2:", p2)
