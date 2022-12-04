from rich import print


class AOC2204:
    def __init__(self, input_file):
        with open(input_file) as fd:
            self.rows = fd.read().strip().split('\n')
            self.pairs = [[list(map(int, elf.split('-'))) for elf in row.split(',')] for row in self.rows]

    def is_one_fully_contained(self, range_0_from, range_0_to, range_1_from, range_1_to):
        return (
                (range_0_from >= range_1_from and range_0_to <= range_1_to) or
                (range_1_from >= range_0_from and range_1_to <= range_0_to)
        )

    def do_pairs_overlap(self, range_0_from, range_0_to, range_1_from, range_1_to):
        return (
            (range_1_from <= range_0_from <= range_1_to) or
            (range_1_from <= range_0_to <= range_1_to) or
            (range_0_from <= range_1_from <= range_0_to) or
            (range_0_from <= range_1_to <= range_0_to))

    def part_1(self):
        return len(list(filter(lambda p: self.is_one_fully_contained(p[0][0], p[0][1], p[1][0], p[1][1]), self.pairs)))

    def part_2(self):
        return len(list(filter(lambda p: self.do_pairs_overlap(p[0][0], p[0][1], p[1][0], p[1][1]), self.pairs)))


if __name__ == "__main__":
    aoc = AOC2204("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2204("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')
