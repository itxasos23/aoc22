from rich import print
from dataclasses import dataclass


@dataclass
class Move:
    amount: int
    _from: int
    _to: int


class Stack:
    def __init__(self):
        self.stack = []

    def add(self, item):
        self.stack.append(item)

    def remove(self):
        return self.stack.pop()


class AOC2205:
    def __init__(self, input_file):
        with open(input_file) as fd:
            stack_txt, moves_txt = fd.read().rstrip().split('\n\n')

        stacks_num = int(stack_txt.strip().splitlines()[-1].split(' ')[-1])
        self.stacks = [Stack() for _ in range(stacks_num)]

        boxes_txt = stack_txt.split('\n')[:-1]
        for idx, stack in enumerate(self.stacks):
            letter_idx = 1 + 4 * idx
            for box_row in boxes_txt[::-1]:
                if letter_idx > len(box_row) - 1 or box_row[letter_idx] == ' ':
                    break
                stack.add(box_row[letter_idx])

        self.moves = [
            Move(
                amount=int(move_txt.split('move ')[1].split(' from')[0]),
                _from=int(move_txt.split('from ')[1].split(' to')[0]),
                _to=int(move_txt.split('to ')[1])
            ) for move_txt in moves_txt.split('\n')]

    def part_1(self):
        [self.stacks[move._to - 1].add(self.stacks[move._from - 1].remove()) for move in self.moves for _ in range(move.amount) ]
        return ''.join([stack.stack[-1] for stack in self.stacks])

    def part_2(self):
        for move in self.moves:
            buffer = [self.stacks[move._from - 1].remove() for _ in range(move.amount)]

            for element in buffer[::-1]:
                self.stacks[move._to - 1].add(element)

        return ''.join([stack.stack[-1] for stack in self.stacks])


if __name__ == "__main__":
    aoc = AOC2205("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')
    aoc = AOC2205("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2205("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    aoc = AOC2205("input.txt")
    print(f'Part 2: {aoc.part_2()}')
