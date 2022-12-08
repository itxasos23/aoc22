from rich import print
from dataclasses import dataclass
from typing import Optional


@dataclass
class Tree:
    size: Optional[int]
    is_visible: Optional[bool] = None
    is_visible_top: Optional[bool] = None
    is_visible_bottom: Optional[bool] = None
    is_visible_left: Optional[bool] = None
    is_visible_right: Optional[bool] = None
    scenic_score: Optional[int] = 0


class AOC2207:
    def __init__(self, input_file):
        with open(input_file) as fd:
            rows = list(map(lambda x: x.strip(), fd.readlines()))
        self.trees = [list(map(lambda x: Tree(size=int(x)), row)) for row in rows]

    def populate_visibility_on_trees(self):
        for row_idx, row in enumerate(self.trees):
            for col_idx, tree in enumerate(row):
                tree.is_visible_left = max(list(map(lambda x: x.size, row[:col_idx])) or [-1]) < tree.size
                tree.is_visible_right = max(list(map(lambda x: x.size, row[col_idx+1:])) or [-1]) < tree.size
                tree.is_visible_top = max(list(map(lambda x: x.size, [row[col_idx] for row in self.trees[:row_idx]])) or [-1]) < tree.size
                tree.is_visible_bottom = max(list(map(lambda x: x.size, [row[col_idx] for row in self.trees[row_idx+1:]])) or [-1]) < tree.size

        for row in self.trees:
            for tree in row:
                tree.is_visible = any([tree.is_visible_left, tree.is_visible_top, tree.is_visible_bottom, tree.is_visible_right])

    def get_visibility_str(self):
        return '\n'.join([''.join([str(1) if x.is_visible_right else str(0) for x in row]) for row in self.trees])

    def count_view(self, size, tree_sizes):
        view = -1
        for idx, tree_size in enumerate(tree_sizes):
            if tree_size >= size:
                view = idx + 1
                break

        if view == -1:
            return len(tree_sizes)
        return view

    def new_method():
        print()

    def populate_scenic_score_on_trees(self):
        for row_idx, row in enumerate(self.trees):
            for col_idx, tree in enumerate(row):
                view_left = self.count_view(tree.size, [row[col_idx].size for row in self.trees[row_idx-1::-1]])
                view_right = self.count_view(tree.size, [row[col_idx].size for row in self.trees[row_idx+1:]])
                view_top = self.count_view(tree.size, list(map(lambda x: x.size, row[col_idx-1::-1])))
                view_bottom = self.count_view(tree.size, list(map(lambda x: x.size, row[col_idx+1:])))

                tree.scenic_score = view_left*view_right*view_top*view_bottom

    def part_1(self):
        self.populate_visibility_on_trees()
        return len(list(filter(lambda x: x.is_visible, [x for row in self.trees for x in row])))

    def part_2(self):
        self.populate_visibility_on_trees()
        self.populate_scenic_score_on_trees()

        return max(x.scenic_score for row in self.trees for x in row)


if __name__ == "__main__":
    aoc = AOC2207("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2207("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2207("input.txt")
    print(f'Part 2: {aoc.part_2()}')
