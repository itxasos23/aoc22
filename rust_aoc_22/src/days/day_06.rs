mod day_06 {
    use std::collections::HashSet;
    use std::fs;

    fn get_file_contents() -> String {
        let input_file_path = "input/06.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn get_position_of_first_distinct_n_gram(message: &str, n: usize) -> Option<usize> {
        let mut running_hash_set = HashSet::new();

        for (idx, m_char) in message.as_bytes().into_iter().enumerate() {
            if idx < n {
                running_hash_set.insert(m_char);
            } else {
                let char_to_remove = message.as_bytes()[idx - n];

                if !message.as_bytes()[idx + 1 - n..idx - 1].contains(&char_to_remove) {
                    running_hash_set.remove(&message.as_bytes()[idx - n]);
                }

                running_hash_set.insert(m_char);
            }

            if running_hash_set.len() == n {
                return Some(idx + 1);
            }
        }
        return None;
    }

    pub fn day_06_1() -> Option<usize> {
        let message_string = get_file_contents();
        return get_position_of_first_distinct_n_gram(&message_string, 4);
    }

    pub fn day_06_2() -> Option<usize> {
        let message_string = get_file_contents();
        return get_position_of_first_distinct_n_gram(&message_string, 14);
    }
}

pub fn day_06() {
    println!("Day 06-1: {}", day_06::day_06_1().unwrap());
    println!("Day 06-2: {}", day_06::day_06_2().unwrap());
}
