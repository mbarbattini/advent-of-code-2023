import numpy as np


VALUE_DICT_ACE_HIGH = {'2': 0, '3': 1, '4': 2, '5': 3, '6': 4, '7': 5, '8': 6, '9': 7, 'T': 8, 'J': 9, 'Q': 10, 'K': 11, 'A': 12}
VALUE_DICT_WITH_JOKER = {'J': 0, '2': 1, '3': 2, '4': 3, '5': 4, '6': 5, '7': 6, '8': 7, '9': 8, 'T': 9, 'Q': 10, 'K': 11, 'A': 12}

"""
1. Build an array of the rankings.
    6: five of a kind
    5: four of a kind
    4: full house
    3: three of a kind
    2: two pair
    1: one pair
    0: high card
2. Sort the array based on the ranking. Deal with any conflicts
3. Sort the score array the same way as the ranking array
4. perform the final calculation
"""


def read_input():
    arr = None
    with open(r"C:\Users\H457071\Desktop\AdventOfCode2023\Day_7\input.txt") as f:
    # with open(r"test_input_1.txt") as f:
        arr = f.readlines()
    f.close()
    return arr

lines = read_input()
all_rankings = []
all_hands = []
all_bids = []

    

for i, l in enumerate(lines):

    ranking = None
    hand, num = l.split()

    all_hands.append(hand)
    all_bids.append((num))

    all_values = [v for v in hand]
    unique_values = set(all_values)


    pairs = [v for v in unique_values if hand.count(v) == 2]
    two_pair = [] if len(pairs) < 2 else [1]

    three_of_a_kind = [v for v in unique_values if hand.count(v) == 3]
    four_of_a_kind = [v for v in unique_values if hand.count(v) == 4]
    five_of_a_kind = [v for v in unique_values if hand.count(v) == 5]


    full_house = []
    for u1 in unique_values:
        if all_values.count(u1) == 2:
            for u2 in unique_values:
                if all_values.count(u2) == 3:
                    full_house = [1]

    ranking = 0
    if pairs:
        ranking = 1
    if two_pair:
        ranking = 2
    if three_of_a_kind:
        ranking = 3
    if full_house:
        ranking = 4
    if four_of_a_kind:
        ranking = 5
    if five_of_a_kind:
        ranking = 6

    all_rankings.append(ranking)


all_data = []
# index, hand, rank, bid 
for i in range(len(all_rankings)):
    all_data.append((i, all_hands[i], all_rankings[i], all_bids[i]))


def hand_heiarchy(hand):
    """
    returns an array that contains the number rank
    of all 5 cards in the hand from left to right.
    When used as a key in sorted(), it will choose 
    the hand that has the highest card, and continue
    with a tiebreaker by picking the highest card from
    left to right
    Ex. [2, 4, 6, 2, 12] for '4684A'
    """
    rankings = []
    for c in hand:
        rankings.append(VALUE_DICT_ACE_HIGH[c])
    return rankings


# sort the array. First by the hand type, then by the highest card from left to right
sorted_hands = sorted(all_data, key=lambda h: (h[2], hand_heiarchy(h[1])))

total = 0
for i in range(len(sorted_hands)):
    score = (i+1) * int(sorted_hands[i][3])
    print(sorted_hands[i])
    print(score)
    total += score

print(f"The answer is: {total}")