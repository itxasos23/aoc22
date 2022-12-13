from typing import List, Tuple, Union
from dataclasses import dataclass

@dataclass
class Message:
    contents: List
    def __lt__(self, other):
        return compare_elements(self.contents, other.contents)


def compare_elements(l0, l1):
    if isinstance(l0, int) and isinstance(l1, int):
        if l1 != l0:
            return l0 < l1
        return None

    if not isinstance(l1, list):
        l1 = [l1]

    if not isinstance(l0, list):
        l0 = [l0]

    for i in range(min(len(l0), len(l1))):
        if l1[i] == l0[i]:
            continue
        comparison_result = compare_elements(l0[i], l1[i])
        if comparison_result is not None:
            return comparison_result

    return len(l0) < len(l1) if len(l0) != len(l1) else None


def find_matching_delimiter(string):
    matching_delimiter_position = -1
    depth = 1
    for idx, element in enumerate(string[1:]):
        depth += 1 if element == '[' else (-1 if element == ']' else 0)

        if depth == 0:
            matching_delimiter_position = idx+1
            break

    if matching_delimiter_position == -1:
        raise ValueError("No matching delimiter found for str {}".format(string))
    return matching_delimiter_position


def parse_next_item(string: str) -> Union[List, int]:
    if string.startswith("["):
        string = string[1:-1]

    done = False
    items = []
    while not done and string:
        if string.startswith("["):
            matching_delimiter_position = find_matching_delimiter(string)
            items.append(parse_next_item(string[:matching_delimiter_position+1]))
            string = string[matching_delimiter_position+2:]

        else:
            pos = string.find(",")
            if pos == -1:
                items.append(int(string.strip()))
                done = True
            else:
                items.append(int(string[:pos].strip()))
                string = string[pos+1:].strip()

    return items


class AOC2213:
    def __init__(self, filename):
        data = open(filename).read().strip()
        self.pairs = [list(map(parse_next_item, pair.split('\n')))for pair in data.split('\n\n')]

        data = data.replace("\n\n", "\n")
        self.messages = [Message(contents=parse_next_item(line)) for line in data.splitlines()]

    def test(self, pair):
        return compare_elements(pair[0], pair[1])

    def part_1(self):
        return sum((pair_idx+1) * self.test(pair) for pair_idx, pair in enumerate(self.pairs))

    def part_2(self):
        self.messages.extend([Message(contents=[[2]]), Message(contents=[[6]])])
        self.messages.sort()

        result = 1
        for (idx, message) in enumerate(self.messages):
            if message.contents in ([[2]], [[6]]):
                result *= idx +1
        return result


if __name__ == "__main__":
    aoc = AOC2213("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2213("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2213("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2213("input.txt")
    print(f'Part 2: {aoc.part_2()}')
