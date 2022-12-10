mod day_07 {
    use std::fs;

    #[derive(Debug, Clone)]
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

    fn get_cwd_node<'a>(root: &'a FileNode, cwd: &str) -> &'a FileNode {
        let cwd_clone = cwd.clone();
        let all_cwd_items = &cwd_clone.split("/").collect::<Vec<&str>>();
        //println!("DEBUG -- all_cwd_items {:?}", all_cwd_items);
        let cwd_items = if cwd == "/" {
            all_cwd_items[1..all_cwd_items.len() - 1].to_vec()
        } else {
            all_cwd_items[1..all_cwd_items.len()].to_vec()
        };
        let mut cwd_node = root;
        for path_section in cwd_items.iter() {
            let children_names = cwd_node
                .children
                .iter()
                .map(|x| x.name.as_str())
                .collect::<Vec<&str>>();
            if !children_names.contains(path_section) {
                panic!(
                    "Trying to access {} child among {:?} in cwd {}",
                    path_section, children_names, cwd
                );
            }

            cwd_node = cwd_node
                .children
                .iter()
                .filter(|x| x.name == *path_section)
                .next()
                .unwrap();
        }
        cwd_node
    }

    fn perform_cd_command(_root: &mut FileNode, cwd: &mut String, command_str: &str) {
        //println!("DEBUG - Handling cd command.");
        //println!("DEBUG -- cwd before: {}", cwd);

        if !command_str.starts_with("cd ..") {
            let cd_target = command_str.split(" ").collect::<Vec<&str>>()[1];
            *cwd = cwd.to_owned() + if cwd != "/" { "/" } else { "" } + cd_target;
        } else {
            //println!("DEBUG -- cwd before: {}", cwd);
            let cwd_clone = cwd.clone();
            let all_cwd_items = &cwd_clone.split("/").collect::<Vec<&str>>();

            //println!("DEBUG -- all_cwd_items: {:?}", all_cwd_items);

            let cwd_items = if cwd == "/" {
                all_cwd_items[1..all_cwd_items.len() - 1].to_vec()
            } else {
                all_cwd_items[1..all_cwd_items.len()].to_vec()
            };

            *cwd = String::from("");
            let path_items = if cwd_items.len() > 0 {
                cwd_items[..cwd_items.len() - 1].to_vec()
            } else {
                [].to_vec()
            };
            if path_items.len() > 0 {
                for cwd_item in cwd_items[..cwd_items.len() - 1].iter() {
                    *cwd = cwd.to_owned() + "/" + cwd_item;
                }
            }
        }
        //println!("DEBUG -- cwd after: {}", cwd);
    }
    fn perform_ls_command(root: &mut FileNode, cwd: &mut String, command_str: &str) {
        let command_lines = command_str.lines().collect::<Vec<&str>>();

        let contents = &command_lines[1..];

        let cwd_clone = cwd.clone();
        let all_cwd_items = &cwd_clone.split("/").collect::<Vec<&str>>();
        //println!("DEBUG -- all_cwd_items {:?}", all_cwd_items);
        let cwd_items = if cwd == "/" {
            all_cwd_items[1..all_cwd_items.len() - 1].to_vec()
        } else {
            all_cwd_items[1..all_cwd_items.len()].to_vec()
        };

        //println!("DEBUG - Handling ls command.");
        //println!("DEBUG - cwd_items is {:?}", cwd_items);

        let mut cwd_node = root;

        for path_section in cwd_items.iter() {
            let children_names = cwd_node
                .children
                .iter()
                .map(|x| x.name.as_str())
                .collect::<Vec<&str>>();

            if !children_names.contains(path_section) {
                panic!(
                    "Trying to access {} child among {:?} in cwd {}",
                    path_section, children_names, cwd
                );
            }

            cwd_node = cwd_node
                .children
                .iter_mut()
                .filter(|x| x.name == *path_section)
                .next()
                .unwrap();
        }

        //println!("DEBUG --- cwd is {} for cwd_node {}", cwd, cwd_node.path);

        for row in contents.iter() {
            let mut row_parts = row.split(" ");
            let size_or_dir = row_parts.next().unwrap();
            let node_name: &str = row_parts.next().unwrap();

            let path = if cwd != "/" {
                cwd.clone() + "/" + node_name
            } else {
                cwd.clone() + node_name
            }
            .to_owned();

            if size_or_dir == "dir" {
                cwd_node.children.push(FileNode {
                    name: node_name.to_owned(),
                    path,
                    is_dir: true,
                    size: None,
                    children: vec![],
                });
                //println!("DEBUG -- Created dir {:?}", cwd_node.children.last().unwrap().path);
            } else {
                let filename = node_name;
                let filesize = size_or_dir
                    .parse::<usize>()
                    .expect(&format!("Could not parse size of file: {}", size_or_dir).to_owned());
                cwd_node.children.push(FileNode {
                    name: filename.to_owned(),
                    path,
                    is_dir: false,
                    size: Some(filesize),
                    children: vec![],
                });
                //println!("DEBUG -- Created file {:?}", cwd_node.children.last().unwrap().path);
            }
        }
        //println!("DEBUG -- node {} has children {:?}", cwd_node.path, cwd_node.children.iter().map(|x| x.name.as_str()).collect::<Vec<&str>>().join(","));
    }

    fn populate_dir_sizes(file_node: &mut FileNode) -> usize {
        if !file_node.is_dir {
            //println!(
            //    "DEBUG - File: {}: {}",
            //    file_node.path,
            //    file_node.size.unwrap()
            //);
            file_node
                .size
                .expect(format!("Node {} is a file with no size.", file_node.path).as_str())
        } else {
            let mut subfolders_sum: usize = 0;
            for child in file_node.children.iter_mut() {
                subfolders_sum += populate_dir_sizes(child);
            }

            file_node.size = Some(subfolders_sum);

            //println!(
            //    "DEBUG -- Setting size of dir {}: {}",
            //    file_node.path,
            //    file_node.size.unwrap()
            //);
            subfolders_sum
        }
    }

    fn find_sub_eq_100k_dirs(file_node: &FileNode) -> Vec<&FileNode> {
        let mut sub_eq_100k_dirs_from_children = vec![];

        for child in file_node.children.iter() {
            sub_eq_100k_dirs_from_children.extend(find_sub_eq_100k_dirs(child));
        }

        if file_node.is_dir && file_node.size.unwrap() <= 100000 {
            sub_eq_100k_dirs_from_children.push(&file_node);
        }

        return sub_eq_100k_dirs_from_children;
    }

    fn populate_tree(root: &mut FileNode, commands_str: Vec<&str>) {
        let mut cwd = "/".to_owned();
        for command_str in commands_str.iter() {
            let command_line: &str = command_str.lines().next().unwrap();
            if command_line.starts_with("cd") {
                perform_cd_command(root, &mut cwd, &command_str);
            } else if command_line.starts_with("ls") {
                perform_ls_command(root, &mut cwd, &command_str);
            } else {
                panic!("Unrecognized command: {}", command_line);
            }
        }
        populate_dir_sizes(root);
    }

    fn find_min_dir_to_delete_over_size(node: &FileNode, lower_limit: usize) -> Option<&FileNode> {
        if !node.is_dir {
            return None;
        }

        let mut mins_of_children: Vec<&FileNode> = vec![];

        for child in node.children.iter() {
            let min_of_child = match find_min_dir_to_delete_over_size(child, lower_limit) {
                Some(value) => vec![value].clone(),
                None => vec![].clone(),
            };

            mins_of_children.extend(min_of_child);
        }

        if node.size.unwrap() > lower_limit {
            mins_of_children.push(node);
        }

        if mins_of_children.len() > 0 {
            let mut min_sized_node = mins_of_children.first().unwrap();
            for child in mins_of_children[1..].iter() {
                if child.size.unwrap() < min_sized_node.size.unwrap() {
                    min_sized_node = child;
                };
            }

            return Some(min_sized_node);
        }

        None
    }

    pub fn day_07_1() -> usize {
        let contents = get_file_contents();
        let commands_str: Vec<&str> = contents.split("$ ").collect::<Vec<&str>>()[2..]
            .to_vec()
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut file_tree = FileNode {
            name: "/".to_string(),
            path: "/".to_string(),
            is_dir: true,
            size: None,
            children: vec![],
        };

        populate_tree(&mut file_tree, commands_str);
        find_sub_eq_100k_dirs(&file_tree)
            .iter()
            .map(|x| get_cwd_node(&file_tree, x.path.as_str()).size.unwrap())
            .sum::<usize>()
    }

    pub fn day_07_2() -> usize {
        let contents = get_file_contents();
        let commands_str: Vec<&str> = contents.split("$ ").collect::<Vec<&str>>()[2..]
            .to_vec()
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut root = FileNode {
            name: "/".to_string(),
            path: "/".to_string(),
            is_dir: true,
            size: None,
            children: vec![],
        };

        populate_tree(&mut root, commands_str);
        let total_space = 70000000;
        let remaining_space = total_space - root.size.unwrap();
        let space_to_free = 30000000 - remaining_space;

        let min_dir_to_delete = find_min_dir_to_delete_over_size(&root, space_to_free);
        //println!("Min dir to delete is {}", min_dir_to_delete.unwrap().path);
        min_dir_to_delete.unwrap().size.unwrap()
    }
}

pub fn day_07() {
    println!("Day 07-1: {:?}", day_07::day_07_1());
    println!("Day 07-2: {:?}", day_07::day_07_2());
}
