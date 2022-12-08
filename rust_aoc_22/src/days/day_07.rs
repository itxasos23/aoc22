mod day_07 {
    use std::fs;
    use regex::Regex;
    use std::collections::HashSet;
    use std::fs::File;

    struct FileNode {
        name: String,
        path: String,
        is_dir: bool,
        size: Option<usize>,
        children: Vec<FileNode>
    }

    fn get_file_contents() -> String {
        let input_file_path = "input/07.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn handle_cd_command(root: &mut FileNode, pwd: &mut str, command: &str) {

        for (idx, command_line) in command.lines().into_iter().enumerate(){
            if idx == 0 {
                let (_, filename) = command_line.split(" ").collect(
            }
        }

        for line in command_lines
        {
            for
        }



    }


    pub fn day_07_1() -> Option<usize> {
        let message_string = get_file_contents();
        let mut root = FileNode{
            name: "/".to_string(), path: "/".to_string(), is_dir: true, size: None, children: vec![]
        };

        for command in message_string.split("$ ").into_iter() {
            if command.len() == 0 {continue;}
            if command.starts_with("cd") {
                handle_cd_command(&mut root, &mut pwd, &command);
            } else if command.starts_with("ls") {
                root = handle_ls_command(command)
            }
        }
        Some(0)
    }

    pub fn day_07_2() -> Option<usize> {
        let message_string = get_file_contents();
        Some(0)
   }
}

pub fn day_07() {
    println!("Day 07-1: {}", day_07::day_07_1().unwrap());
    println!("Day 07-2: {}", day_07::day_07_2().unwrap());
}
