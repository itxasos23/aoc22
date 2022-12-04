mod day_04 {
    use std::collections::HashSet;
    use std::fs;

    fn get_file_contents() -> String {
        let input_file_path = "input/04.txt";
        return fs::read_to_string(input_file_path).unwrap();
    }

    fn parse_row(row: &str) -> (usize, usize, usize, usize) {
        let elves: Vec<&str> = row.split(',').collect();
        let (elf_0, elf_1) = (elves[0], elves[1]);

        let elf_0_range: Vec<&str> = elf_0.split('-').collect();
        let (elf_0_from, elf_0_to) = (
            elf_0_range[0].parse::<usize>().unwrap(),
            elf_0_range[1].parse::<usize>().unwrap(),
        );

        let elf_1_range: Vec<&str> = elf_1.split('-').collect();
        let (elf_1_from, elf_1_to) = (
            elf_1_range[0].parse::<usize>().unwrap(),
            elf_1_range[1].parse::<usize>().unwrap(),
        );
        (elf_0_from, elf_0_to, elf_1_from, elf_1_to)
    }

    fn is_one_fully_contained(
        elf_0_from: usize,
        elf_0_to: usize,
        elf_1_from: usize,
        elf_1_to: usize,
    ) -> bool {
        (elf_0_from >= elf_1_from && elf_0_to <= elf_1_to)
            || (elf_1_from >= elf_0_from && elf_1_to <= elf_0_to)
    }

    fn do_pairs_overlap(
        elf_0_from: usize,
        elf_0_to: usize,
        elf_1_from: usize,
        elf_1_to: usize,
    ) -> bool {
        (elf_1_from <= elf_0_from) && (elf_0_from <= elf_1_to)
            || (elf_1_from <= elf_0_to) && (elf_0_to <= elf_1_to)
            || (elf_0_from <= elf_1_from) && (elf_1_from <= elf_0_to)
            || (elf_0_from <= elf_1_to) && (elf_1_to <= elf_0_to)
    }

    pub fn day_04_1() -> usize {
        let contents = get_file_contents();
        let rows = contents.lines();

        let mut sum_overlaps: usize = 0;

        for row in rows {
            let (elf_0_from, elf_0_to, elf_1_from, elf_1_to) = parse_row(&row);
            if is_one_fully_contained(elf_0_from, elf_0_to, elf_1_from, elf_1_to) {
                sum_overlaps += 1;
            }
        }
        return sum_overlaps;
    }

    pub fn day_04_2() -> usize {
        let contents = get_file_contents();
        let rows = contents.lines();

        let mut sum_overlaps: usize = 0;

        for row in rows {
            let (elf_0_from, elf_0_to, elf_1_from, elf_1_to) = parse_row(&row);
            if do_pairs_overlap(elf_0_from, elf_0_to, elf_1_from, elf_1_to) {
                sum_overlaps += 1;
            }
        }
        return sum_overlaps;
    }
}

pub fn day_04() {
    println!("Day 04-1: {}", day_04::day_04_1());
    println!("Day 04-2: {}", day_04::day_04_2());
}
