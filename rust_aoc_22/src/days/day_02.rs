mod day_02 {
    use std::fs;
    use std::collections::HashMap;

    #[derive(Eq, PartialEq, Hash, Clone)]
    enum PlayOptions {
        Rock,
        Paper,
        Scissors
    }

    #[derive(Eq, PartialEq, Hash, Clone)]
    enum Outcomes {
        Win,
        Lose,
        Tie
    }
    
    fn get_file_contents() -> String {
        let input_file_path = "input/02.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }
    fn get_game_outcome(play_1: &PlayOptions, play_2: &PlayOptions) -> Outcomes {
        let wins = HashMap::from([(PlayOptions::Rock, PlayOptions::Scissors), (PlayOptions::Scissors, PlayOptions::Paper), (PlayOptions::Paper, PlayOptions::Rock)]);

        if play_1 == play_2 {
            return Outcomes::Tie;
        }
        if play_2 == &wins[play_1]{
            return Outcomes::Lose;
        }
        return Outcomes::Win;
    }
    fn get_play_2(play_1: &PlayOptions, expected_outcome: &Outcomes) -> PlayOptions {
        let wins = HashMap::from([(PlayOptions::Rock, PlayOptions::Scissors), (PlayOptions::Scissors, PlayOptions::Paper), (PlayOptions::Paper, PlayOptions::Rock)]);

        if expected_outcome == &Outcomes::Tie {
            return play_1.clone();
        } else if expected_outcome == &Outcomes::Lose {
            return wins[&play_1].clone();
        } else if expected_outcome == &Outcomes::Win {
            for (k, v) in wins {
                if &v == play_1 {
                return k;
            }
            }
        }

        panic!("Wrong value");
    }

    pub fn day_02_1() -> i32 {
        let scores_play = HashMap::from([
            (PlayOptions::Rock, 1), 
            (PlayOptions::Paper, 2), 
            (PlayOptions::Scissors, 3)
        ]);
        
        let scores_outcome = HashMap::from([
            (Outcomes::Lose, 0),
            (Outcomes::Tie, 3),
            (Outcomes::Win, 6)
        ]);

        let player_1_map = HashMap::from([
            ("A", PlayOptions::Rock),
            ("B", PlayOptions::Paper),
            ("C", PlayOptions::Scissors)
        ]);

        let part_1_map = HashMap::from([
            ("X", PlayOptions::Rock),
            ("Y", PlayOptions::Paper),
            ("Z", PlayOptions::Scissors)
        ]);

        let contents = get_file_contents();
        let contents_fix = contents.strip_suffix('\n').unwrap();
        let games = contents_fix.split('\n').collect::<Vec<&str>>();
        let parsed_games: Vec<Vec<&str>> = games.into_iter() .map(|x| x.split(' ').collect()).into_iter().collect();

        let mut score = 0;
        for game in parsed_games {
            let play_1 = &player_1_map[game[0]];
            let play_2 = &part_1_map[game[1]];

            score += scores_play[play_2];
            score += scores_outcome[&get_game_outcome(&play_1, &play_2)];
        }

        return score;
    }



    pub fn day_02_2() -> i32{
        let scores_play = HashMap::from([
            (PlayOptions::Rock, 1),
            (PlayOptions::Paper, 2),
            (PlayOptions::Scissors, 3)
        ]);

        let scores_outcome = HashMap::from([
            (Outcomes::Lose, 0),
            (Outcomes::Tie, 3),
            (Outcomes::Win, 6)
        ]);

        let player_1_map = HashMap::from([
            ("A", PlayOptions::Rock),
            ("B", PlayOptions::Paper),
            ("C", PlayOptions::Scissors)
        ]);

        let part_2_map = HashMap::from([
            ("X", Outcomes::Lose),
            ("Y", Outcomes::Tie),
            ("Z", Outcomes::Win)
        ]);

        let contents = get_file_contents();
        let contents_fix = contents.strip_suffix('\n').unwrap();
        let games = contents_fix.split('\n').collect::<Vec<&str>>();
        let parsed_games: Vec<Vec<&str>> = games.into_iter() .map(|x| x.split(' ').collect()).into_iter().collect();

        let mut score = 0;
        for game in parsed_games {
            let play_1 = player_1_map[game[0]].clone();
            let play_2 = get_play_2(&play_1, &part_2_map[game[1]]);

            score += scores_play[&play_2];
            score += scores_outcome[&get_game_outcome(&play_1, &play_2)];
        }

        return score;
    }

}


pub fn day_02() {
    println!("Part1: {}", day_02::day_02_1());
    println!("Part2: {}", day_02::day_02_2());
}

