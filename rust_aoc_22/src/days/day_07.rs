mod day_07 {
    use regex::Regex;
    use std::collections::HashSet;
    use std::fs;
    use std::fs::File;

    struct FileNode {
        name: String,
        path: String,
        is_dir: bool,
        size: Option<usize>,
        children: Vec<FileNode>,
    }

    fn get_file_contents() -> String {
        let input_file_path = "input/07.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    pub fn day_07_1() -> Option<usize> {
        None
    }

    pub fn day_07_2() -> Option<usize> {
        let message_string = get_file_contents();
        Some(0)
    }
}

pub fn day_07() {
    println!("Day 07-1: todo");
    println!("Day 07-2: todo");
}
