mod day_02 {
    use std::collections::HashSet;
    use std::fs;

    fn get_file_contents() -> String {
        let input_file_path = "input/03.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn get_priority(char: &char) -> u32 {
        if char.is_lowercase() {
            let char_code = char.clone() as u32;
            char_code - 96
        } else {
            let char_code = char.clone() as u32;
            char_code - 38
        }
    }

    fn get_hash_set_of_string(str: &str) -> HashSet<char> {
        let mut set = HashSet::new();
        for char in str.chars().into_iter() {
            set.insert(char.clone());
        }
        set
    }

    fn get_intersection_set_of_sets(
        set_1: HashSet<char>,
        set_2: HashSet<char>,
        set_3: HashSet<char>,
    ) -> HashSet<char> {
        let mut inter_set_1_2 = HashSet::<char>::new();
        for item in set_1.intersection(&set_2).into_iter() {
            inter_set_1_2.insert(item.clone());
        }

        let mut inter_set = HashSet::<char>::new();
        for item in inter_set_1_2.intersection(&set_3).into_iter() {
            inter_set.insert(item.clone());
        }
        inter_set
    }

    fn get_intersection_set_of_list_of_sets(list_of_sets: Vec<HashSet<char>>) -> HashSet<char> {
        let mut last_set_ref = &list_of_sets[0];

        let mut new_set = HashSet::<char>::new();
        let mut last_set = HashSet::<char>::new();

        for set_idx in 1..list_of_sets.len() {
            new_set.retain(|_| false);

            for item in last_set_ref.intersection(&list_of_sets[set_idx]) {
                new_set.insert(item.clone());
            }
            last_set = new_set.clone();
            last_set_ref = &last_set;
        }
        last_set
    }

    pub fn day_03_1() -> u32 {
        let contents = get_file_contents();
        let rows = contents.lines();

        let mut sum_priorities: u32 = 0;

        for row in rows {
            let half_0 = &row[..row.len() / 2];
            let half_1 = &row[row.len() / 2..];

            let mut set_0 = get_hash_set_of_string(half_0);
            let mut set_1 = get_hash_set_of_string(half_1);

            let inter = &set_0.intersection(&set_1);
            let intersection_item = inter.clone().into_iter().next().unwrap();
            let priority = get_priority(intersection_item);

            sum_priorities += priority;
        }
        return sum_priorities;
    }

    pub fn day_03_2() -> u32 {
        let contents = get_file_contents();
        let rows: Vec<&str> = contents.lines().collect();
        let mut sum_priorities: u32 = 0;

        for group_idx in 0..(rows.len() / 3) {
            let group_rows = &rows[group_idx * 3..(group_idx + 1) * 3];
            let group_sets = group_rows
                .into_iter()
                .map(|row| get_hash_set_of_string(row))
                .collect();
            let inter = get_intersection_set_of_list_of_sets(group_sets);

            let intersection_item = inter.into_iter().next().unwrap();
            let priority = get_priority(&intersection_item);

            sum_priorities += priority;
        }
        return sum_priorities;
    }
}

pub fn day_03() {
    println!("Day 03-1: {}", day_02::day_03_1());
    println!("Day 03-2: {}", day_02::day_03_2());
}
