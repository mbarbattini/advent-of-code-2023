"""
#   for each char in string
SYMBOL ON LEFT SIDE
if c is a symbol
//    if the next c is a number
//       build the number until there is no more numbers
//       if c ends with a symbol, valid number, add to total
//          Then this number might be double counted: &3423@
//    else continue (symbol surrounded by periods)
// SYMBOL ON RIGHT SIDE
// if c is a number
//    build the number until there is no more numbers
//    if c ends with a symbol, valid number, add to total
"""

def main():


    symbols = [
        "@",
        "#",
        "$",
        "%",
        "&",
        "*"
    ]
    total = 0
    index = -1

    text = open("/Users/matthewbarbattini/Desktop/AdventOfCode2023/day_3/src/input.txt", "r").read()
    # print(len(text))
    number = ""
    while index is not None:
            # print(f"Index: {index}")

    # for line in text:
        # for c in line:
            index += 1
            if index == len(text) - 1:
                break
            c = text[index]
            # print(c, end="\n")
            if c in symbols:
                if text[index + 1].isdigit():
                    number = ""
                    while text[index + 1].isdigit():
                        number += text[index + 1]
                        index += 1
                    # good number
                    # print(number)
                    total += int(number)
                    # symbol wraps both sides
                    if text[index + 1] in symbols:
                        pass
            # found a number
            elif c.isdigit():
                total_digits = 1
                number = ""
                number += text[index]
                while text[index + 1].isdigit():
                    total_digits += 1
                    number += text[index + 1]
                    index += 1
                # symbol on the right side only
                if text[index + 1] in symbols:
                    # good number
                    # print(number)
                    total += int(number)
                else:
                    # bad number
                    number = ""
                # skip by the amount of numbers found
                # index += total_digits - 1
                total_digits = 0
                    
    print(total)
main()
