from rich import print
from dataclasses import dataclass
from typing import List, Optional


@dataclass
class FileNode:
    size: Optional[int]
    is_dir: bool
    name: str
    children: List["FileNode"]


class AOC2207:
    def __init__(self, input_file):
        self.filetree = FileNode(
            name='/',
            is_dir=True,
            size=None,
            children=[]
        )

        self.pwd = self.filetree
        self.pwd_path = '/'

        with open(input_file) as fd:
            self.commands = fd.read().rstrip().split('$ ')

    def update_pwd(self, command):
        new_pwd = command.splitlines()[0].split('cd ')[1]

        # the new pwd should already exist in the filetree
        if new_pwd == '/':
            self.pwd, self.pwd_path = self.filetree, ''
        elif new_pwd == '..':
            dot_dot = self.filetree
            for level in self.pwd_path.split('/')[1:-1]:
                dot_dot = list(filter(lambda x: x.name == level, dot_dot.children))[0]

            self.pwd, self.pwd_path = dot_dot, '/'.join(self.pwd_path.split('/')[:-1])

        else:
            self.pwd = list(filter(lambda x: x.name == new_pwd, self.pwd.children))[0]
            self.pwd_path += f'/{new_pwd}'

    def update_file_tree(self, command):
        command_line, new_items = command.splitlines()[0], command.splitlines()[1:]

        for item in new_items:
            type_or_size, name = item.split(' ')
            self.pwd.children.append(FileNode(name=name, is_dir=type_or_size == 'dir', children=[],
                                              size=int(type_or_size) if type_or_size != 'dir' else None))

    def get_and_populate_size_of_node(self, node):
        if node.size:
            return node.size

        size_sum = sum(self.get_and_populate_size_of_node(child) for child in node.children)
        node.size = size_sum
        return size_sum

    def get_dirs_up_to_size(self, node, max_size):
        valid_dirs = []
        if node.size <= max_size and node.is_dir:
            valid_dirs.append(node.name)

        [valid_dirs.extend(self.get_dirs_up_to_size(child, max_size)) for child in node.children]
        return valid_dirs

    def sum_sizes_of_dirs_up_to_size(self, node, max_size):
        running_sum = sum(self.sum_sizes_of_dirs_up_to_size(child, max_size) for child in node.children)
        running_sum += node.size if node.size <= max_size and node.is_dir else 0
        return running_sum

    def get_min_dir_of_size(self, node, min_size):
        sizes_around = []

        if node.size > min_size and node.is_dir:
            sizes_around.append(node.size)

        for child in node.children:
            child_candidate = self.get_min_dir_of_size(child, min_size)
            if child_candidate:
                sizes_around.append(child_candidate)

        return min(sizes_around) if sizes_around else None

    def part_1(self):
        for command in self.commands:
            if command.startswith("cd"):
                self.update_pwd(command)

            elif command.startswith("ls"):
                self.update_file_tree(command)

        self.get_and_populate_size_of_node(self.filetree)
        return self.sum_sizes_of_dirs_up_to_size(self.filetree, 100000)

    def part_2(self):
        total_disk = 7 * 10 ** 7
        need_to_be_free = 3 * 10 ** 7

        for command in self.commands:
            if command.startswith("cd"):
                self.update_pwd(command)

            elif command.startswith("ls"):
                self.update_file_tree(command)

        self.get_and_populate_size_of_node(self.filetree)
        need_to_free = need_to_be_free - (total_disk - self.filetree.size)

        return self.get_min_dir_of_size(self.filetree, need_to_free)


if __name__ == "__main__":
    aoc = AOC2207("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2207("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input.txt")
    print(f'Part 2: {aoc.part_2()}')
