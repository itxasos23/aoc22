from rich import print
from dataclasses import dataclass
from typing import List
from functools import reduce


@dataclass
class Operation:
    symbol: str # + or *
    first_divisor: int = None  # None means old here; an int represents itself
    second_divisor: int = None


@dataclass
class Test:
    divisor: int
    target_if_true: int
    target_if_false: int


@dataclass
class Monkey:
    items: List[int]
    operation: Operation
    test: Test
    business: int


class AOC2211:
    def __init__(self, input_file):
        with open(input_file) as fd:
            data = fd.read()

        self.monkey_strs = list(map(lambda x: x.strip(), data.split('\n\n')))
        self.monkeys = []
        self.items = []

        for monkey_str in self.monkey_strs:
            lines = monkey_str.splitlines()
            if not lines:
                continue

            monkey_id = int(lines[0].split('Monkey ')[1].split(":")[0])
            starting_items = list(map(lambda x: int(x), lines[1].split("Starting items: ")[1].strip().split(', ')))

            operation = lines[2].split("Operation: new = ")[1].strip()
            first_operand, symbol, second_operand = operation.split(' ')

            first_operand = None if first_operand == "old" else int(first_operand)
            second_operand = None if second_operand == "old" else int(second_operand)

            test_divisor = int(lines[3].split("Test: divisible by ")[1].strip())
            target_if_true = int(lines[4].split("throw to monkey ")[1].strip())
            target_if_false = int(lines[5].split("throw to monkey ")[1].strip())

            self.monkeys.append(
                Monkey(
                    items=starting_items,
                    operation=Operation(
                        symbol=symbol,
                        first_divisor=first_operand,
                        second_divisor=second_operand
                    ),
                    test=Test(
                        divisor=test_divisor,
                        target_if_true=target_if_true,
                        target_if_false=target_if_false
                    ),
                    business=0
                )
            )
        self.global_modulus = reduce( lambda a,b: a*b, [m.test.divisor for m in self.monkeys])

    def update_value_part_1(self, old_value, operation):
        first_operand = operation.first_divisor or old_value
        second_operand = operation.second_divisor or old_value

        if operation.symbol == "+":
            return (first_operand + second_operand) // 3
        else:
            return (first_operand * second_operand) // 3

    def update_value_part_2(self, old_value, operation):
        first_operand = operation.first_divisor or old_value
        second_operand = operation.second_divisor or old_value
        if operation.symbol == "+":
            new_value = (first_operand + second_operand) % self.global_modulus
        else:
            new_value = (first_operand * second_operand) % self.global_modulus
        return new_value

    def play_rounds(self, rounds=20, divide_by_3=False):
        update_method = self.update_value_part_1 if not divide_by_3 else self.update_value_part_2

        for _ in range(rounds):
            for monkey in self.monkeys:
                for item in monkey.items:
                    item_new_value = update_method(item, monkey.operation)
                    test_outcome = item_new_value % monkey.test.divisor == 0
                    monkey.business += 1

                    if test_outcome:
                        self.monkeys[monkey.test.target_if_true].items.append(item_new_value)
                    else:
                        self.monkeys[monkey.test.target_if_false].items.append(item_new_value)
                monkey.items = []

    def part_1(self):
        self.play_rounds()
        self.monkeys.sort(key=lambda x: -x.business)
        return self.monkeys[0].business * self.monkeys[1].business

    def part_2(self):
        self.play_rounds(10000, True)
        self.monkeys.sort(key=lambda x: -x.business)
        return self.monkeys[0].business * self.monkeys[1].business


if __name__ == "__main__":
    aoc = AOC2211("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2211("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2211("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2211("input.txt")
    print(f'Part 2: {aoc.part_2()}')
q