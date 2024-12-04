import re
import sys

if __name__ == "__main__":
    # read full input from standard in
    lines = sys.stdin.readlines()

    # remove newline characters
    lines = [line.strip() for line in lines]

    # join lines into one string
    lines = "".join(lines)

    # remove data that does not match form mul(x, y)
    pattern = r"mul\((\d+),(\d+)\)"

    matches = re.findall(pattern, lines)

    product = 0

    for match in matches:
        product += int(match[0]) * int(match[1])

    print (product)

    