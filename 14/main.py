from rich import print
from itertools import chain


class AOC2214:
    def __init__(self, filename):
        data = open(filename).read().strip().splitlines()
        self.rocks = [
            [tuple(map(int, point.split(","))) for point in row.split(" -> ")]
            for row in data
        ]

        self.rock_edges = list(chain(*self.rocks))

        min_col = min(list(map(lambda x: x[0], self.rock_edges)) + [500]) - 2
        min_row = min(list(map(lambda x: x[1], self.rock_edges)) + [-2])
        max_col = max(list(map(lambda x: x[0], self.rock_edges)) + [500])
        max_row = max(list(map(lambda x: x[1], self.rock_edges)) + [0])

        self.min_col = min_col

        # encoding:
        # 0 -> air
        # 1 -> rock
        # 2 -> sand

        self.cave = []
        for row_idx in range(min_row - 1, max_row + 2):
            row = []
            for col_idx in range(min_col - 1, max_col + 2):
                row.append(0)

            self.cave.append(row)

        # poulate rock
        for rock in self.rocks:
            for idx, rock_node in enumerate(rock[:-1]):
                _from = rock_node
                _to = rock[idx + 1]

                # print(f"From {_from} to {_to}")

                # do we move horizontally?
                if _from[1] == _to[1]:
                    row = _from[1]

                    is_reversed = _to[0] < _from[0]
                    for col in range(
                        _from[0],
                        _to[0] + (-1 if is_reversed else 1),
                        -1 if is_reversed else 1,
                    ):
                        col_idx = col - min_col
                        row_idx = row - min_row

                        # print(f"Populating {[row_idx, col_idx]}")
                        self.cave[row_idx][col_idx] = 1

                # do we move vertically?
                if _from[0] == _to[0]:
                    col = _from[0]

                    is_reversed = _to[1] < _from[1]
                    for row in range(
                        _from[1],
                        _to[1] + (-1 if is_reversed else 1),
                        -1 if is_reversed else 1,
                    ):
                        col_idx = col - min_col
                        row_idx = row - min_row

                        # print(f"Populating {[row_idx, col_idx]}")
                        self.cave[row_idx][col_idx] = 1

        self.cave[0][500-min_col] = 3

    def print_cave(self):
        print(
            "\n".join(
                list(
                    map(
                        lambda row: "".join(
                                ["." if x == 0 else (
                                    "#" if x == 1 else ( 
                                    "o" if x == 2 else "+")) for x in row]),
                        self.cave,
                    )
                )
            )
        )

    def part_1(self):
        fallen_off_the_void = False

        sand = 0
        while not fallen_off_the_void:
            come_to_rest = False
            new_sand_unit = [0, 500 - self.min_col]
            sand += 1
            # print(f"{sand} sand in cave.")

            while not come_to_rest:
                # is next step down free?
                if new_sand_unit[0] == len(self.cave) - 1:
                    fallen_off_the_void = True
                    break

                # print(f"Sand at {new_sand_unit}")
                # print(
                # f"below are {self.cave[new_sand_unit[0]+1][new_sand_unit[1]]-1}, {self.cave[new_sand_unit[0]+1][new_sand_unit[1]]} and {self.cave[new_sand_unit[0]+1][new_sand_unit[1]+1] }"
                # )
                if self.cave[new_sand_unit[0] + 1][new_sand_unit[1]] == 0:
                    new_sand_unit[0] += 1
                    continue

                elif self.cave[new_sand_unit[0] + 1][new_sand_unit[1] - 1] == 0:
                    new_sand_unit[0] += 1
                    new_sand_unit[1] -= 1
                    continue

                elif self.cave[new_sand_unit[0] + 1][new_sand_unit[1] + 1] == 0:
                    new_sand_unit[0] += 1
                    new_sand_unit[1] += 1
                    continue

                else:
                    come_to_rest = True
                    self.cave[new_sand_unit[0]][new_sand_unit[1]] = 2
                    continue

        # self.print_cave()
        return sand - 1

    def part_2(self):
        max_row = max(map(lambda x: x[1], self.rock_edges))

        new_cave = []
        for row in self.cave:
            new_cave_row = [0 for _ in range(len(self.cave)*3//4)]

            for item in row:
                new_cave_row.append(item)

            new_cave_row.extend(0 for _ in range(len(self.cave)*3//4))
            new_cave.append(new_cave_row)

        self.min_col = self.min_col - len(self.cave)
        self.cave = new_cave

        for idx, _ in enumerate(self.cave[0]):
            self.cave[max_row + 2][idx] = 1

        # self.print_cave()

        source_blocked = False

        sand = 0
        while not source_blocked:
            come_to_rest = False
            new_sand_unit = [0, 500 - self.min_col]
            sand += 1
            # print(f"{sand} sand in cave.")

            while not come_to_rest:
                # is next step down free?
                if self.cave[0][500 - self.min_col] == 2 and self.cave[1][500-self.min_col-1] == 2 and self.cave[1][500-self.min_col]and self.cave[1][500-self.min_col+1]:
                    source_blocked = True
                    break

                # print(f"Sand at {new_sand_unit}")
                # print(
                # f"below are {self.cave[new_sand_unit[0]+1][new_sand_unit[1]]-1}, {self.cave[new_sand_unit[0]+1][new_sand_unit[1]]} and {self.cave[new_sand_unit[0]+1][new_sand_unit[1]+1] }"
                # )
                if self.cave[new_sand_unit[0] + 1][new_sand_unit[1]] == 0:
                    new_sand_unit[0] += 1
                    continue

                elif self.cave[new_sand_unit[0] + 1][new_sand_unit[1] - 1] == 0:
                    new_sand_unit[0] += 1
                    new_sand_unit[1] -= 1
                    continue

                elif self.cave[new_sand_unit[0] + 1][new_sand_unit[1] + 1] == 0:
                    new_sand_unit[0] += 1
                    new_sand_unit[1] += 1
                    continue

                else:
                    come_to_rest = True
                    self.cave[new_sand_unit[0]][new_sand_unit[1]] = 2
                    continue

        self.print_cave()
        return sand



if __name__ == "__main__":
    aoc = AOC2214("input_ex.txt")
    print(aoc.part_1())
    aoc = AOC2214("input_ex.txt")
    print(aoc.part_2())
    aoc = AOC2214("input.txt")
    print(aoc.part_1())
    aoc = AOC2214("input.txt")
    print(aoc.part_2())

