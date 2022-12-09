from rich import print
from dataclasses import dataclass

@dataclass
class Move:
    direction: str
    amount: int


class AOC2207:
    def __init__(self, input_file):
        with open(input_file) as fd:
            rows = list(map(lambda x: x.strip(), fd.readlines()))
        self.moves = []
        for row in rows:
            self.moves.append(Move(direction=row.split(' ')[0], amount=int(row.split(' ')[1])))

        self.head_position = [0, 0]
        self.tail_position = [0, 0]

        self.link_positions = [[0, 0] for _ in range(10)]

        self.link_positions[0] = self.head_position
        self.link_positions[-1] = self.tail_position

        self.squares_visited_by_tail = set(['0,0'])

    def move_head(self, move):
        for _ in range(move.amount):
            self.move_head_once(move)

    def move_head_once(self, move):
        self.head_position[0] += 1 if move.direction == 'R' else (-1 if move.direction == 'L' else 0)
        self.head_position[1] += 1 if move.direction == 'U' else (-1 if move.direction == 'D' else 0)

    def move_tail(self):
        self.tail_position = self._get_new_link_position(self.tail_position, self.head_position)

    def update_all_links(self, move):
        for _ in range(move.amount):
            self.move_head_once(move)
            self.update_long_tail(move)
            self.squares_visited_by_tail.add(','.join(map(str, self.link_positions[-1])))

    def update_long_tail(self, move):
        for idx, link_pos in enumerate(self.link_positions[1:]):
            idx = idx + 1
            prev_link_pos = self.link_positions[idx-1]
            link_pos = self._get_new_link_position(link_pos, prev_link_pos)

    @staticmethod
    def _get_new_link_position(link_pos, prev_link_pos):
        if abs(prev_link_pos[0] - link_pos[0]) <= 1 and abs(prev_link_pos[1] - link_pos[1]) <= 1:
            return link_pos

        for i in [0, 1]:
            if prev_link_pos[i] != link_pos[i]:
                link_pos[i] += 1 if prev_link_pos[i] > link_pos[i] else -1

        return link_pos

    def visualize_as_description(self):
        max_x = max(map(lambda x: x[0], self.link_positions))
        min_x = min(map(lambda x: x[0], self.link_positions))
        max_y = max(map(lambda x: x[1], self.link_positions))
        min_y = min(map(lambda x: x[1], self.link_positions))

        max_x = max([max_x, 10])
        max_y = max([max_y, 10])
        min_x = min([min_x, 0])
        min_y = min([min_y, 0])

        for y in range(max_y+1, min_y-1, -1):
            row_chars = []
            for x in range(min_x, max_x+1):
                found = False
                for idx, link in enumerate(self.link_positions):
                    if link == [x, y]:
                        row_chars.append(str(idx))
                        found = True
                        break

                if not found:
                    row_chars.append('.')

            print(''.join(row_chars))

    def part_1(self):
        for move in self.moves:
            for _ in range(move.amount):
                #print(f'{move.direction}-{move.amount}')
                self.move_head_once(move)
                self.move_tail()
                self.squares_visited_by_tail.add(','.join(map(str, self.tail_position)))

        return len(self.squares_visited_by_tail)

    def part_2(self):
        for move in self.moves:
            # print(f'{move.direction}-{move.amount}')
            self.update_all_links(move)
            # self.visualize_as_description()

        return len(self.squares_visited_by_tail)


if __name__ == "__main__":
    aoc = AOC2207("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input_ex2.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2207("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input.txt")
    print(f'Part 2: {aoc.part_2()}')
