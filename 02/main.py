from rich import print


class AOC2202:
    scores_play = {"R": 1, "P": 2, "S": 3}
    scores_outcome = {"lose": 0, "tie": 3, "win": 6}
    player_1_map = {"A": "R", "B": "P", "C": "S"}
    part_1_map = {"X": "R", "Y": "P", "Z": "S"}
    part_2_map = {"X": "lose", "Y": "tie", "Z": "win"}
    wins = {"R": "S", "S": "P", "P": "R"}

    def __init__(self, input_file):
        with open(input_file) as fd:
            data = fd.read().strip()

        self.games_part_1 = [
            [self.player_1_map[game.split(' ')[0]], self.part_1_map[game.split(' ')[1]]]
            for game in data.splitlines()
        ]

        self.games_part_2 = [
            [self.player_1_map[game.split(' ')[0]], self.part_2_map[game.split(' ')[1]]]
            for game in data.splitlines()
        ]

    def part_1(self):
        return sum(
            self.scores_play[game[1]] + self.scores_outcome[self.get_outcome(game)]
            for game in self.games_part_1
        )

    def part_2(self):
        return sum(
            self.scores_play[self.decider(game[0], game[1])] + self.scores_outcome[game[1]]
            for game in self.games_part_2
        )

    def get_outcome(self, game):
        play_1, play_2 = game
        if play_1 == play_2:
            return "tie"

        if play_2 == self.wins[play_1]:
            return "lose"
        return "win"

    def decider(self, play_1, expected_outcome):
        if expected_outcome == 'tie':
            return play_1

        if expected_outcome == 'lose':
            return self.wins[play_1]

        if expected_outcome == 'win':
            for p1, p2 in self.wins.items():
                if p2 == play_1:
                    return p1


if __name__ == "__main__":
    aoc = AOC2202("input.txt")
    print(f'Part 1: {aoc.part_1()}')
    print(f'Part 2: {aoc.part_2()}')
