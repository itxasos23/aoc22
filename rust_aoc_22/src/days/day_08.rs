mod day_08 {

    use std::fs;

    fn get_file_contents(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    fn build_tree_array(contents: &str) -> Vec<Vec<usize>> {
        
        let mut trees: Vec<Vec<usize>> = Vec::new();
        for line in contents.lines() {
            let mut tree_row: Vec<usize> = Vec::new();
            for value in line.chars() {
                let height = value.to_string().parse::<usize>().unwrap();
                tree_row.push(height);
            }
            trees.push(tree_row);
        }
        trees
    }

    fn visible_tree_count(trees: &Vec<Vec<usize>>) -> usize {
        let mut visible_count = 0;

        for (id_row, row) in trees.iter().enumerate() {
            for (id_col, tree) in row.iter().enumerate() {
                if id_row == 0 || id_col == 0 || id_row == trees.len()-1 || id_col == row.len()-1 {visible_count += 1; continue;}

                // We're guaranteed for this not to be an edge

                let left_of_tree = &row[..id_col];
                let right_of_tree = &row[id_col+1..];
                
                let top_of_tree = &trees[..id_row].iter().map(|row| row[id_col]).collect::<Vec<usize>>();
                let bottom_of_tree = &trees[id_row+1..].iter().map(|row| row[id_col]).collect::<Vec<usize>>();

                if 
                    left_of_tree.iter().reduce(|acc, x| acc.max(x)).unwrap() < tree || 
                    right_of_tree.iter().reduce(|acc, x| acc.max(x)).unwrap() < tree || 
                    top_of_tree.iter().reduce(|acc, x| acc.max(x)).unwrap() < tree ||
                    bottom_of_tree.iter().reduce(|acc, x| acc.max(x)).unwrap() < tree 
                    {
                    visible_count += 1;
                }
            } 
        }
        visible_count
    }

    fn get_scenic_score(height: &usize, ordered_treeline: &Vec<usize>) -> usize {
        if ordered_treeline.len() == 0 {return 0;}

        for (idx, new_tree_height) in ordered_treeline.iter().enumerate() {
            if new_tree_height >= height {return idx+1;}
        }

        return ordered_treeline.len();

    }

    fn get_best_scenic_score(trees: &Vec<Vec<usize>>) -> usize {
        let mut scenic_scores: Vec<Vec<usize>> = Vec::new();
        

        let mut debug_scores: Vec<Vec<usize>> = Vec::new();



        for (id_row, row) in trees.iter().enumerate() {
            let mut row_scenic_scores: Vec<usize> = Vec::new();
            let mut debug_row: Vec<usize> = Vec::new();
            for (id_col, tree) in row.iter().enumerate() {
                if id_row == 0 || id_col == 0 || id_row == trees.len()-1 || id_col == row.len()-1 {row_scenic_scores.push(0); debug_row.push(0); continue;}

                // We're guaranteed for this not to be an edge
                let left_of_tree = &row[..id_col];

                let mut rev_left_of_tree: Vec<usize> = vec![];
                for element in left_of_tree.iter().rev() {
                    rev_left_of_tree.push(*element);
                }
                let left_scenic_score = get_scenic_score(tree, &rev_left_of_tree);
                debug_row.push(left_scenic_score);

                let right_of_tree = &row[id_col+1..];
                let right_scenic_score = get_scenic_score(tree, &right_of_tree.to_vec());


                let top_of_tree = &trees[..id_row].iter().map(|row| row[id_col]).collect::<Vec<usize>>();
                let mut rev_top_of_tree: Vec<usize> = vec![];
                for element in top_of_tree.iter().rev() {
                    rev_top_of_tree.push(*element);
                }                
                let top_scenic_score = get_scenic_score(tree, &rev_top_of_tree);


                let bottom_of_tree = &trees[id_row+1..].iter().map(|row| row[id_col]).collect::<Vec<usize>>();
                let bottom_scenic_score = get_scenic_score(tree, &bottom_of_tree.to_vec());
                    
                row_scenic_scores.push(left_scenic_score * right_scenic_score * top_scenic_score * bottom_scenic_score)
            }
            scenic_scores.push(row_scenic_scores);
            debug_scores.push(debug_row);
        }
        
        //for row in &debug_scores{
        //    println!("{:?}", row);
        //}
        
        let mut max_scenic_score = 0;

        for row in &scenic_scores {
            for tree in row {
                if tree > &max_scenic_score {max_scenic_score = *tree;}
            }
        }

        scenic_scores.iter().fold(
            0, |acc, row| row.iter().fold(0, |acc_r, tree| acc_r.max(*tree)).max(acc))
    }
    pub fn day_08_1() -> usize {
        let contents = get_file_contents("input/08.txt");
        let trees = build_tree_array(&contents);
        let count = visible_tree_count(&trees);
        count
    }

    pub fn day_08_2() -> usize {
        let contents = get_file_contents("input/08.txt");
        let trees = build_tree_array(&contents);
        let best_scenic_score = get_best_scenic_score(&trees);
        best_scenic_score
    }
}

pub fn day_08() {
    println!("Day 08-1: {:?}", day_08::day_08_1());
    println!("Day 08-2: {:?}", day_08::day_08_2());
}
