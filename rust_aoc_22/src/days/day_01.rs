use std::fs;

fn get_file_contents() -> String {
    let input_file_path = "input/01.txt";
    return fs::read_to_string(input_file_path).unwrap();
}

fn day_01_1() -> i32{
    let contents = get_file_contents();
    let blocks = contents.strip_suffix("\n").unwrap().split("\n\n");

    let mut calories_sum: Vec<i32> = blocks.map(|s| s.split("\n").map(|x| x.parse::<i32>().unwrap()).sum()).collect();
    calories_sum.sort_by_key(|x| -x);

    return calories_sum[0]
}

fn day_01_2() -> i32{
    let contents = get_file_contents();
    let blocks = contents.strip_suffix("\n").unwrap().split("\n\n");

    let mut calories_sum: Vec<i32> = blocks.map(|s| s.split("\n").map(|x| x.parse::<i32>().unwrap()).sum()).collect();
    calories_sum.sort_by_key(|x| -x);

    return calories_sum[0 .. 3].iter().sum::<i32>();
}

pub fn day_01() {
    println!("Part1: {}", day_01_1());
    println!("Part2: {}", day_01_2());
}

