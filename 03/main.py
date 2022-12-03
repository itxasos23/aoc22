from rich import print


class AOC2203:
    def __init__(self, input_file):
        with open(input_file) as fd:
            self.rows = fd.read().strip().split('\n')

    def get_priority(self, char):
        return ord(char) - 96 if char.islower() else ord(char) - 38

    def part_1(self):
        return sum(self.get_priority(list(set(row[:int(len(row) / 2)]).intersection(set(row[int(len(row) / 2):])))[0]) for row in self.rows)

    def part_2(self):
        return sum(self.get_priority(list(set(self.rows[group_idx * 3]).intersection(set(self.rows[group_idx * 3 + 1])).intersection(set(self.rows[group_idx * 3 + 2])))[0]) for group_idx in range(len(self.rows)//3))


if __name__ == "__main__":
    aoc = AOC2203("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2203("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')
