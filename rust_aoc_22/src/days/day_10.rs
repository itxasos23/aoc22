mod day_10 {
    use std::fs;

    fn read_file_contents(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    fn update_register_counter(
        signal_strength: &mut Vec<isize>,
        instruction_counter: isize,
        register: isize,
    ) {
        if (instruction_counter + 20) % 40 == 0 {
            signal_strength.push(register * instruction_counter);
        }
    }

    pub fn day_10_1(filename: &str) -> isize {
        let contents = read_file_contents(filename);
        let instructions = contents.lines();

        let mut instruction_counter: isize = 0;
        let mut register: isize = 1;

        let mut signal_strength = Vec::new();

        for instruction in instructions {
            if instruction == "noop" {
                instruction_counter += 1;
                update_register_counter(&mut signal_strength, instruction_counter, register);
            } else {
                let number = instruction.split(' ').collect::<Vec<&str>>()[1]
                    .parse::<isize>()
                    .unwrap();

                instruction_counter += 1;
                update_register_counter(&mut signal_strength, instruction_counter, register);

                instruction_counter += 1;
                update_register_counter(&mut signal_strength, instruction_counter, register);

                register += number;
            }
        }
        signal_strength.iter().sum()
    }

    fn compute_next_crt_value(crt_len: usize, register: isize) -> bool {
        isize::try_from(crt_len).unwrap() - 1 <= register
            && register <= isize::try_from(crt_len).unwrap() + 1
    }

    fn update_register_for_new_line_if_applicable(crt_len: usize, register: &mut isize) {
        if crt_len != 0 && crt_len % 40 == 0 {
            *register += 40;
        }
    }

    fn print_crt_output(crt_output: Vec<bool>) {
        let mut output_str = "".to_owned();
        for i in 0..6 {
            for pixel in crt_output[(i*40)..(i+1)*40].iter() {
              output_str.push_str(if *pixel {"#"} else {"."});
            }
            output_str.push_str("\n");
        }
        println!("{}", output_str);
    }

    pub fn day_10_2(filename: &str) {
        let contents = read_file_contents(filename);
        let instructions = contents.lines();

        let mut register: isize = 1;
        let mut crt_output: Vec<bool> = Vec::new();

        for instruction in instructions {
            if instruction == "noop" {
                crt_output.push(compute_next_crt_value(crt_output.len(), register));
                update_register_for_new_line_if_applicable(crt_output.len(), &mut register);
            } else {
                let number = instruction.split(' ').collect::<Vec<&str>>()[1]
                    .parse::<isize>()
                    .unwrap();
                crt_output.push(compute_next_crt_value(crt_output.len(), register));
                update_register_for_new_line_if_applicable(crt_output.len(), &mut register);
                crt_output.push(compute_next_crt_value(crt_output.len(), register));
                update_register_for_new_line_if_applicable(crt_output.len(), &mut register);
                register += number;


            }
        }
        print_crt_output(crt_output);
    }
}

pub fn day_10() {
    println!("Day 10-1: {}", day_10::day_10_1("input/10.txt"));
    println!("Day 10-2:");
    day_10::day_10_2("input/10.txt");
}
