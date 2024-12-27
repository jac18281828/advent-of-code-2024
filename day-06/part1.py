import sys
import os

GUARD_CHARS = ['^', 'v', '<', '>']

def is_obstacle(c: str) -> bool:
    return c == '#'

def index(x: int, y: int, width: int) -> int:
    return y * width + x

def step_guard(x: int, y: int, c: str) -> (int, int):
    if c == '^':
        return x, y - 1
    elif c == 'v':
        return x, y + 1
    elif c == '<':
        return x - 1, y
    elif c == '>':
        return x + 1, y
    
def turn_right(c: str) -> str:
    if c == '^':
        return '>'
    elif c == '>':
        return 'v'
    elif c == 'v':
        return '<'
    elif c == '<':
        return '^'
    
def is_in_bounds(x: int, y: int, width: int, height: int) -> bool:
    return x >= 0 and x < width and y >= 0 and y < height

def find_guard(chararray: list) -> (int, int, str):
    width = len(chararray[0])
    height = len(chararray)
    for y in range(height):
        for x in range(width):
            if chararray[y][x] in GUARD_CHARS:
                return x, y, chararray[y][x]

def print_grid(chararray: list):
    for line in chararray:
        print(''.join(line))
    print()

if __name__ == '__main__':
    lines = sys.stdin.readlines()
    chararray = [list(line.strip()) for line in lines]
    width = len(chararray[0])
    height = len(chararray)
    is_blocked = False
    x, y, c = find_guard(chararray)
    step_count = 0
    while not is_blocked:
        nx, ny = step_guard(x, y, c)
        if is_in_bounds(nx, ny, width, height):
            if is_obstacle(chararray[ny][nx]):
                c = turn_right(c)
                nx, ny = step_guard(x, y, c)
                if is_in_bounds(nx, ny, width, height) and is_obstacle(chararray[ny][nx]):
                    is_blocked = True
                    break
                chararray[ny][nx] = c
            step_count += 1
            chararray[y][x] = 'X'
            chararray[ny][nx] = c
            x, y = nx, ny
        else:
            is_blocked = True
    print(step_count)
    distinct_places = 1
    for line in chararray:
        for c in line:
            if c == 'X':
                distinct_places += 1
    print(distinct_places)
        
    
    
