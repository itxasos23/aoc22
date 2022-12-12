from rich import print
import numpy as np
from collections import defaultdict


class AOC2212:
    def __init__(self, input_file):
        with open(input_file) as fd:
            data = fd.read()
        self.start_node, self.end_node, self.height_map = self._parse_data(data)

    def _parse_data(self, data):
        height_map = np.zeros([len(data.splitlines()), len(data.splitlines()[0])], dtype=int)
        for row_idx, row in enumerate(data.splitlines()):
            for col_idx, cell in enumerate(row):
                if cell == "S":
                    start_node = [row_idx, col_idx]
                    cell = "a"
                elif cell == "E":
                    end_node = [row_idx, col_idx]
                    cell = "z"

                height_map[row_idx][col_idx] = ord(cell) - 97

        return start_node, end_node, height_map

    def _build_connectivity_matrix(self, height_map):
        conn_matrix_height = height_map.shape[0] * height_map.shape[1]
        conn_matrix = np.zeros([conn_matrix_height, conn_matrix_height], dtype=int)

        for row_idx, row in enumerate(height_map):
            for col_idx, height in enumerate(row):
                conn_matrix_idx = row_idx * len(row) + col_idx

                # right
                if col_idx != len(row) -1:
                    right_idx = row_idx * len(row) + col_idx + 1
                    next_height = height_map[row_idx][col_idx+1]
                    if next_height <= height + 1:
                        conn_matrix[conn_matrix_idx][right_idx] = 1

                # top
                if row_idx != 0:
                    top_idx = (row_idx - 1) * len(row) + col_idx
                    next_height = height_map[row_idx-1][col_idx]
                    if next_height <= height + 1:
                        conn_matrix[conn_matrix_idx][top_idx] = 1

                # left
                if col_idx != 0:
                    left_idx = (row_idx) * len(row) + col_idx - 1
                    next_height = height_map[row_idx][col_idx - 1]
                    if next_height <= height + 1:
                        conn_matrix[conn_matrix_idx][left_idx] = 1

                # bottom
                if row_idx != len(height_map) -1:
                    bottom_idx = (row_idx + 1) * len(row) + col_idx
                    next_height = height_map[row_idx+1][col_idx]
                    if next_height <= height + 1:
                        conn_matrix[conn_matrix_idx][bottom_idx] = 1

        return conn_matrix

    def _build_heuristic_cost_matrix(self, height_map):
        h_cost_matrix = np.zeros(height_map.shape, dtype=float)

        for row_idx, row in enumerate(height_map):
            for col_idx, height in enumerate(row):

                cost = abs(self.end_node[0] - row_idx) + abs(self.end_node[1] - col_idx)


                h_cost_matrix[row_idx][col_idx] = cost

        return h_cost_matrix

    def h(self, node):
        if node is None:
            return float("inf")
        return self.h_cost_matrix[node[0]][node[1]]

    def get_neighbours(self, node):
        conn_matrix_idx = node[0] * self.height_map.shape[1] + node[1]
        row = self.conn_matrix[conn_matrix_idx]

        neighbour_nodes = []
        for neighbour_idx, element in enumerate(row):
            if element == 0:
                continue

            n_0 = neighbour_idx // self.height_map.shape[1]
            n_1 = neighbour_idx % self.height_map.shape[1]

            neighbour_nodes.append([[n_0, n_1], 1])

        return neighbour_nodes

    def stringify_node(self, node):
        return '-'.join(map(lambda x: str(x), node))

    def unstringify_node(self, node_str):
        return list(map(int, node_str.split('-')))

    def a_star(self):
        open_set = set([self.stringify_node(self.start_node)])
        closed_set = set()

        g = {}
        parents = {}

        g[self.stringify_node(self.start_node)] = 0
        parents[self.stringify_node(self.start_node)] = 0

        while len(open_set) > 0:
            current_node = None

            for neighbour in open_set:
                neighbour = self.unstringify_node(neighbour)

                if neighbour is None or \
                        g[self.stringify_node(neighbour)] + self.h(neighbour) < \
                        g[self.stringify_node(neighbour)] + self.h(current_node):
                    current_node = neighbour

            if current_node == self.end_node or not self.get_neighbours(current_node):
                pass

            else:
                for (new_neighbour, weight) in self.get_neighbours(current_node):

                    # We got a new node!
                    if self.stringify_node(new_neighbour) not in open_set and \
                            self.stringify_node(new_neighbour) not in closed_set:

                        open_set.add(self.stringify_node(new_neighbour))
                        parents[self.stringify_node(new_neighbour)] = current_node
                        g[self.stringify_node(new_neighbour)] = g[self.stringify_node(current_node)] + weight

                    else:
                        # We already know this node! Maybe this way is shorter?
                        if g[self.stringify_node(new_neighbour)] > g[self.stringify_node(current_node)] + weight:

                            # It is!
                            g[self.stringify_node(new_neighbour)] = g[self.stringify_node(current_node)] + weight
                            parents[self.stringify_node(new_neighbour)] = current_node
                            if self.stringify_node(new_neighbour) in closed_set:
                                closed_set.remove(self.stringify_node(new_neighbour))
                                open_set.add(self.stringify_node(new_neighbour))

            if current_node is None:
                print("Path does not exist!")

            if current_node == self.end_node:
                path = []
                while parents[self.stringify_node(current_node)] != 0:
                    path.append(current_node)
                    current_node = parents[self.stringify_node(current_node)]

                path.append(self.start_node)
                path.reverse()
                return path

            open_set.remove(self.stringify_node(current_node))
            closed_set.add(self.stringify_node(current_node))

        print("Path does not exist!")
        return None

    def build_paths_bfs(self, start, reverse=False):
        # inspired by https://github.com/timfennis/advent-of-code-2022
        paths = defaultdict(lambda: 10e15)
        paths[(start[0], start[1])] = 0

        to_search = [(start[0], start[1], 0)]

        while to_search:
            row, col, steps = to_search[0]
            to_search = to_search[1:]
            node_height = self.height_map[row][col]

            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                new_row = row + dr
                new_col = col + dc

                if not 0 <= new_row < self.height_map.shape[0] or not 0 <= new_col < self.height_map.shape[1]:
                    continue

                to_height = self.height_map[new_row][new_col]

                if not reverse and not node_height - to_height >= -1:
                    continue

                if reverse and not node_height - to_height <= 1:
                    continue

                if paths[(new_row, new_col)] > steps + 1:
                    paths[(new_row, new_col)] = steps + 1
                    to_search.append((new_row, new_col, steps+1))
        return paths

    def visualize_path(self):
        visualization_matix = [['.' for _ in range(self.height_map.shape[1])] for _ in range(self.height_map.shape[0])]
        for node_idx, node in enumerate(self.path[:-1]):
            next_node = self.path[node_idx+1]

            if next_node[0] > node[0]:
                _char = 'v'
            elif next_node[0] < node[0]:
                _char = '^'
            elif next_node[1] > node[1]:
                _char = '>'
            elif next_node[1] < node[1]:
                _char = '<'
            else:
                Exception("Invalid path!")

            visualization_matix[node[0]][node[1]] = _char

        print('\n'.join(''.join(element for element in row) for row in visualization_matix))


    def part_1(self):
        self.conn_matrix = self._build_connectivity_matrix(self.height_map)
        self.h_cost_matrix = self._build_heuristic_cost_matrix(self.height_map)
        self.path = self.build_paths_bfs(self.start_node)[(self.end_node[0], self.end_node[1])]
        return self.path

    def part_2(self):
        starting_node_list = []
        for row_idx, row in enumerate(self.height_map):
            for col_idx, height in enumerate(row):
                if height == 0:
                    starting_node_list.append((row_idx, col_idx))

        path_lengths = []
        for node in starting_node_list:
            path_len = self.build_paths_bfs(node)[(self.end_node[0], self.end_node[1])]
            path_lengths.append(path_len)

        return min(path_lengths)


if __name__ == "__main__":
    aoc = AOC2212("input_ex.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2212("input_ex.txt")
    print(f'Part 2: {aoc.part_2()}')

    aoc = AOC2212("input.txt")
    print(f'Part 1: {aoc.part_1()}')

    aoc = AOC2212("input.txt")
    print(f'Part 2: {aoc.part_2()}')
