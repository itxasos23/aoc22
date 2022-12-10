mod day_09 {
    use std::collections::HashSet;
    use std::fs;

    fn get_file_contents(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    fn update_chain_position_once(
        chain: &mut Vec<Vec<isize>>,
        direction: &str,
        tail_visited_locations: &mut HashSet<(isize, isize)>,
    ) {
        let mut last_link: Vec<isize> = Vec::new();
        for (idx, link) in chain.iter_mut().enumerate() {
            if idx == 0 {
                //println!("DEBUG - Updating Head Link from {:?}", (link[0], link[1]));
                link[0] += if direction == "R" {
                    1
                } else if direction == "L" {
                    -1
                } else {
                    0
                };
                link[1] += if direction == "U" {
                    1
                } else if direction == "D" {
                    -1
                } else {
                    0
                };
                last_link = link.to_vec();
                //println!("DEBUG - to {:?}", (link[0], link[1]));
            } else {
                let previous_link = last_link.clone();

                //println!(
                //    "DEBUG - Checking link {} for movement against previous one",
                //    idx
                //);
                //println!(
                //    "DEBUG -- Previous link {:?}; current link is at {:?}",
                //    previous_link, link
                //);

                if (previous_link[0] - link[0]).abs() > 1 || (previous_link[1] - link[1]).abs() > 1
                {
                    //println!("DEBUG -- Link has to move.");
                    if previous_link[0] != link[0] {
                        link[0] += if previous_link[0] > link[0] { 1 } else { -1 };
                    }

                    if previous_link[1] != link[1] {
                        link[1] += if previous_link[1] > link[1] { 1 } else { -1 };
                    }

                    //println!("DEBUG -- New link position is {:?}", link);
                }
                last_link = link.to_vec();
            } 
        }
        let tail_link = &chain[chain.len() - 1];
        tail_visited_locations.insert((tail_link[0].clone(), tail_link[1].clone()));
    }

    pub fn day_09_1() -> usize {
        let contents = get_file_contents("input/09.txt");
        let mut chain: Vec<Vec<isize>> = Vec::new();

        for _ in 0..2 {
            chain.push(vec![0, 0]);
        }

        let mut tail_visited_locations: HashSet<(isize, isize)> = HashSet::new();

        for instruction_str in contents.lines() {
            let instruction = instruction_str.trim();

            let mut instruction_iter = instruction.split(" ");
            let direction = instruction_iter.next().unwrap();
            let amount = instruction_iter.next().unwrap().parse::<isize>().unwrap();

            for _ in 0..amount {
                update_chain_position_once(&mut chain, direction, &mut tail_visited_locations);
            }
        }
        tail_visited_locations.len()
    }
    pub fn day_09_2() -> usize {
        let contents = get_file_contents("input/09.txt");
        let mut chain: Vec<Vec<isize>> = Vec::new();

        for _ in 0..10 {
            chain.push(vec![0, 0]);
        }

        let mut tail_visited_locations: HashSet<(isize, isize)> = HashSet::new();

        for instruction_str in contents.lines() {
            let instruction = instruction_str.trim();

            let mut instruction_iter = instruction.split(" ");
            let direction = instruction_iter.next().unwrap();
            let amount = instruction_iter.next().unwrap().parse::<isize>().unwrap();

            for _ in 0..amount {
                update_chain_position_once(&mut chain, direction, &mut tail_visited_locations);
            }
        }
        tail_visited_locations.len()
    }
}

pub fn day_09() {
    println!("Day 09-1: {:?}", day_09::day_09_1());
    println!("Day 09-2: {:?}", day_09::day_09_2());
}
