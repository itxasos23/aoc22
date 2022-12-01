from rich import print


class AOC2201:
    def __init__(self, input_file):
        with open(input_file) as fd:
            self.data = fd.read().strip()

        self.new_blocks = [block.split('\n') for block in self.data.split('\n\n')]
        self.calories = [sum(list(map(int, [row for row in block]))) for block in self.new_blocks]
        self.calories.sort(key=lambda x: -x)

    def part_1(self):
        return self.calories[0]

    def part_2(self):
        return sum(self.calories[0:3])


if __name__ == "__main__":
    aoc = AOC2201("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')
