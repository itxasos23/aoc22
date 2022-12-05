mod day_05 {
    use std::fs;
    use regex::Regex;

    #[derive(Debug)]
    struct Move {
        amount: usize,
        from: usize,
        to: usize,
    }

    #[derive(Debug)]
    struct Stack {
        store: Vec<char>,
    }

    impl Stack {
        fn add(&mut self, item: char) {
            self.store.push(item);
        }
        fn remove(&mut self) -> char {
            self.store.pop().unwrap()
        }
    }

    fn get_file_contents() -> String {
        let input_file_path = "input/05.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn parse_file_contents(file_contents: &str) -> (Vec<Stack>, Vec<Move>) {
        let halves: Vec<&str> = file_contents.split("\n\n").collect();
        let (stack_txt, moves_txt) = (halves[0], halves[1]);

        let stacks_num_lines = stack_txt.lines().collect::<Vec<&str>>();
        let stacks_num_line = stacks_num_lines[stacks_num_lines.len() - 1];
        let stack_ids = stacks_num_line.split(" ").collect::<Vec<&str>>();
        let stacks_num = stack_ids[stack_ids.len() - 1].parse::<usize>().unwrap();

        let mut stacks: Vec<Stack> = vec![];
        for _ in 0..stacks_num {
            stacks.push(Stack { store: vec![] });
        }

        let boxes_txt: Vec<&&str> = stacks_num_lines[..stacks_num_lines.len() - 1].iter().collect();

        for (idx, stack) in stacks.iter_mut().enumerate() {
            let letter_idx = 1 + 4 * idx;

            for box_row in boxes_txt.iter().rev() {
                if letter_idx > box_row.len() - 1 ||
                    box_row.chars().collect::<Vec<char>>()[letter_idx] == ' ' {
                    break;
                }
                stack.add(box_row.chars().collect::<Vec<char>>()[letter_idx]);
            }
        }

        let move_parser_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();


        let mut moves: Vec<Move> = vec![];

        for move_txt in moves_txt.lines().into_iter() {
            let captures = move_parser_regex.captures(move_txt).unwrap();
            let amount = (&captures[1].parse::<usize>().unwrap()).clone();
            let from = (&captures[2].parse::<usize>().unwrap()).clone();
            let to = (&captures[3].parse::<usize>().unwrap()).clone();
            moves.push(Move {amount, from, to})
        }

        (stacks, moves)
    }

    fn format_result(stacks: Vec<Stack>) -> String {
        let tops_of_stacks = stacks.into_iter().map(|s| s.store[s.store.len()-1]).collect::<Vec<char>>();

        let mut return_string = String::new();
        for element in tops_of_stacks.into_iter() {
            return_string.push(element);
        }
        return_string
    }


    pub fn day_05_1() -> String {
        let contents = get_file_contents();
        let (mut stacks, moves) = parse_file_contents(&contents);

        for m in moves {
            for _ in 0..m.amount {
                let item = stacks[m.from-1].remove();
                stacks[m.to-1].add(item);
            }
        }

        format_result(stacks)
    }

    pub fn day_05_2() -> String {
        let contents = get_file_contents();
        let (mut stacks, moves) = parse_file_contents(&contents);

        for m in moves {
            let mut buffer: Vec<char> = vec![];

            for _ in 0..m.amount {
                buffer.push(stacks[m.from-1].remove());
            }
            for element in buffer.into_iter().rev() {
                stacks[m.to-1].add(element)
            }

        }
        format_result(stacks)
    }
}

pub fn day_05() {
    println!("Day 05-1: {}", day_05::day_05_1());
    println!("Day 05-2: {}", day_05::day_05_2());
}
