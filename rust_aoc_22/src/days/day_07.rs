mod day_07 {

    fn get_file_contents() -> String {
        let input_file_path = "input/07.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn get_position_of_first_distinct_n_gram(message: &str, n: usize) -> Option<usize>{todo!();} 


    pub fn day_07_1() -> Option<usize> {
        let message_string = get_file_contents();
        return get_position_of_first_distinct_n_gram(&message_string, 4);
    }

    pub fn day_07_2() -> Option<usize> {
        todo!()
    }
}

pub fn day_07() {
    println!("Day 07-1: {}", day_07::day_07_1().unwrap());
    println!("Day 07-2: {}", day_07::day_07_2().unwrap());
}
