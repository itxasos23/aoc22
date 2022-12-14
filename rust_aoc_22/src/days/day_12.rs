mod day_12 {
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::fs::read_to_string;
    fn get_file_contents(filename: &str) -> String {
        read_to_string(filename).unwrap()
    }

    fn build_heightmap(contents: &str) -> Vec<Vec<usize>> {
        let mut height_map: Vec<Vec<usize>> = Vec::new();
        for (_row_idx, row) in contents.lines().into_iter().enumerate() {
            let mut height_row: Vec<usize> = Vec::new();
            for (_col_idx, _char) in row.bytes().into_iter().enumerate() {
                if _char == 'S' as u8 {
                    height_row.push('a' as usize - 97);
                    continue;
                }
                if _char == 'E' as u8 {
                    height_row.push('z' as usize - 97);
                    continue;
                }

                height_row.push(_char as usize - 97);
            }
            height_map.push(height_row);
        }

        height_map
    }

    fn get_start_and_end_nodes(contents: &str) -> ((usize, usize), (usize, usize)) {
        let mut start_node = (0, 0);
        let mut end_node = (0, 0);

        for (row_idx, row) in contents.lines().into_iter().enumerate() {
            for (col_idx, _char) in row.bytes().into_iter().enumerate() {
                if _char == 'S' as u8 {
                    start_node = (row_idx, col_idx);
                }
                if _char == 'E' as u8 {
                    end_node = (row_idx, col_idx);
                }
            }
        }

        (start_node, end_node)
    }

    fn apply_djikstra(
        start_node: (usize, usize),
        height_map: &Vec<Vec<usize>>,
        reverse: bool,
    ) -> HashMap<(usize, usize), usize> {
        let width = height_map[0].len();
        let tall = height_map.len();

        let mut paths: HashMap<(usize, usize), usize> = HashMap::new();
        paths.insert(start_node, 0);

        let mut nodes_to_search: VecDeque<((usize, usize), usize)> = VecDeque::new();
        nodes_to_search.push_back((start_node, 0));

        while nodes_to_search.len() > 0 {
            let ((row, col), steps) = nodes_to_search[0];
            nodes_to_search.pop_front();
            let height = height_map[row][col];

            let row_idx = row as isize;
            let col_idx = col as isize;

            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                //dbg!((dr, dc));
                if (row == 0 && *dr == -1) || (col == 0 && *dc == -1) {
                    //println!("DEBUG - going to negative index.");
                    continue;
                }

                let new_row = row_idx + dr;
                let new_col = col_idx + dc;

                if new_row as usize == tall || new_col as usize == width {
                    //println!("DEBUG - Going out of bounds.");
                    continue;
                }

                let new_height = height_map[new_row as usize][new_col as usize];

                if !reverse && new_height as isize - height as isize > 1 {
                    continue;
                }
                if reverse && (new_height as isize - height as isize) < -1 {
                    continue;
                }


                let current_cost =
                    match paths.get(&(new_row.try_into().unwrap(), new_col.try_into().unwrap())) {
                        None => usize::MAX,
                        Some(x) => *x,
                    };

                if current_cost > steps + 1 {
                    paths.insert(
                        (new_row.try_into().unwrap(), new_col.try_into().unwrap()),
                        steps + 1,
                    );
                    nodes_to_search.push_front((
                        (new_row.try_into().unwrap(), new_col.try_into().unwrap()),
                        steps + 1,
                    ));
                }
            }
        }

        paths
    }

    pub fn day_12_1() -> usize {
        let contents = get_file_contents("input/12.txt");
        let height_map: Vec<Vec<usize>> = build_heightmap(&contents);
        let (start_node, end_node) = get_start_and_end_nodes(&contents);
        let paths = apply_djikstra(start_node, &height_map, false);
        //println!("{:?}", paths.values());

        *paths.get(&end_node).unwrap()
    }

    pub fn day_12_2() -> usize {
        let contents = get_file_contents("input/12.txt");
        let height_map: Vec<Vec<usize>> = build_heightmap(&contents);
        let (_, end_node) = get_start_and_end_nodes(&contents);

        let mut starting_node_list: Vec<(usize, usize)> = Vec::new();
        for (row_idx, row) in height_map.iter().enumerate() {
            for (col_idx, height) in row.iter().enumerate() {
                if *height != 0 {
                    continue;
                } 
                starting_node_list.push((row_idx, col_idx));
            }
        }

        let path_costs = apply_djikstra(end_node, &height_map, true);

        let mut path_lenghts: Vec<usize> = Vec::new();
        for (idx, node) in starting_node_list.iter().enumerate() {
            let cost = match path_costs.get(node) {
                None => usize::MAX,
                Some(x) => *x
            };
            path_lenghts.push(cost);
        }
        *path_lenghts.iter().min().unwrap()
    }
}

pub fn day_12() {
    println!("Day 12-1: {:?}", day_12::day_12_1());
    println!("Day 12-2: {:?}", day_12::day_12_2());
}
