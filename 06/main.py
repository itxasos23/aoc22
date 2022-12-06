from rich import print
from dataclasses import dataclass


class AOC2206:
    def __init__(self, input_file):
        with open(input_file) as fd:
            self.row = fd.read().rstrip()

    def part_1(self):
        for idx, char in enumerate(self.row[3:]):
            pos = idx + 3

            if len(set(self.row[pos-3:pos+1])) == 4:
                return pos +1

    def part_2(self):
        for idx, char in enumerate(self.row[13:]):
            pos = idx + 13

            if len(set(self.row[pos-13:pos+1])) == 14:
                return pos + 1


if __name__ == "__main__":
    aoc = AOC2206("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2206("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')
