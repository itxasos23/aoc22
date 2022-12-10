from rich import print
from dataclasses import dataclass

@dataclass
class Move:
    direction: str
    amount: int


class AOC2210:
    def __init__(self, input_file):
        with open(input_file) as fd:
            self.data = list(map(lambda x: x.strip(), fd.readlines()))

        self.signal_strenghts = []
        self.crt_output = []

    def register_value_if_applicable(self, cycle_counter, register_x):
        if (cycle_counter+20) % 40 == 0:
            # print(f"Cycle: {cycle_counter}: {register_x}")
            self.signal_strenghts.append(register_x * cycle_counter)

    def compute_next_crt_value(self, register_x):
        self.crt_output.append(1 if len(self.crt_output) -1 <= register_x <= len(self.crt_output) + 1 else 0)

    def visualize_output(self):
        print('\n'.join([''.join('#' if item else '.' for item in self.crt_output[i*40:(i+1)*40]) for i in range(6)]))

    def part_1(self):
        cycle_counter = 0
        register_x = 1
        for command in self.data:
            if command == 'noop':
                cycle_counter += 1
                self.register_value_if_applicable(cycle_counter, register_x)
            else:
                number_to_add = int(command.split(' ')[1])
                cycle_counter += 1
                self.register_value_if_applicable(cycle_counter, register_x)
                cycle_counter += 1
                self.register_value_if_applicable(cycle_counter, register_x)
                register_x += number_to_add

        return sum(self.signal_strenghts)

    def part_2(self):
        register_x = 1
        for command in self.data:
            if len(self.crt_output) % 40 == 0 and self.crt_output:
                register_x += 40

            if command == 'noop':
                self.compute_next_crt_value(register_x)
            else:
                number_to_add = int(command.split(' ')[1])
                self.compute_next_crt_value(register_x)

                if len(self.crt_output) % 40 == 0 and self.crt_output:
                    register_x += 40

                self.compute_next_crt_value(register_x)
                register_x += number_to_add

        self.visualize_output()


if __name__ == "__main__":
    aoc = AOC2210("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2210("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2210("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2210("input.txt")
    print(f'Part 2: {aoc.part_2()}')
