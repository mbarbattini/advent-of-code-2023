from copy import copy

"""
1. Find the differences until you get to all zeros
2. Add the last number in differences to the last number in the parent differences until you get to the input line
3. Store the last computed value in an array.
4. Add all the values
"""


def find_differences_part_1(arr):
    # base case
    if set(arr) == {0}:
        return arr[-1]
    diff = []
    for i in range(len(arr) - 1):
        diff.append(arr[i+1] - arr[i])
    return find_differences_part_1(diff) + diff[-1]


def find_differences_part_2(arr, added_value=None):
    # base case
    if set(arr) == {0}:
        return added_value
    diff = []
    for i in range(len(arr) - 1):
        diff.append(arr[i+1] - arr[i])
    if added_value is not None:
        added_value = arr[0] - added_value
    else:
        added_value = 0
    return find_differences_part_2(diff, added_value=added_value)


lines = None
with open(r"/Users/matthewbarbattini/Desktop/AdventOfCode2023/day_9/input.txt") as f:
    lines = f.readlines()



total = 0
num_string = None
for l in lines:
    nums = []
    num_string = l.split()

    for num in num_string:
        nums.append(int(num))
    nums.reverse()
    score = nums[-1] + find_differences_part_1(nums)
    total += score
    # print(score)

print(f"The answer is {total}")