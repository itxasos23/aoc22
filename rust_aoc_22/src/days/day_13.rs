mod day_13 {
    use std::fs::read_to_string;
    use serde_json::{Result, Value, Number};
    use std::cmp::Ordering;


    fn get_file_contents(filename: &str) -> String {
        read_to_string(filename).unwrap()
    }

    fn are_values_sorted(r0: &Value, r1: &Value) -> Option<bool> {
        if r0.is_u64() && r1.is_u64() {
            //println!("Comparing: {} and {}", r0.as_u64().unwrap(), r1.as_u64().unwrap());
            if r0.as_u64().unwrap() < r1.as_u64().unwrap() {return Some(true)};
            if r0.as_u64().unwrap() > r1.as_u64().unwrap() {return Some(false)};
        }
        
        let local_r0 = if !r0.is_array() {vec![r0.clone()]} else {r0.as_array().unwrap().to_vec()};
        let local_r1 = if !r1.is_array() {vec![r1.clone()]} else {r1.as_array().unwrap().to_vec()};

        for idx in 0..*[local_r0.len(), local_r1.len()].iter().min().unwrap() {
            if local_r0[idx] == local_r1[idx] {continue};

            //println!("Comparing {:?} and {:?}", &local_r0[idx], &local_r1[idx]);
            let sub_comparison_result = 
                
            match are_values_sorted(&local_r0[idx], &local_r1[idx]) {
                None => continue,
                Some(x) => return Some(x),
            };
        }; 

        if local_r1.len() > local_r0.len() {return Some(true)};
        if local_r1.len() < local_r0.len() {return Some(false)};
        None

    }

    pub fn day_13_1() -> usize {
        let contents = get_file_contents("input/13.txt");
        let mut right_count: usize = 0;
        for (pair_idx, pair) in contents.split("\n\n").enumerate() {
            let mut lines_iter = pair.lines();
            let row_0 = lines_iter.next().unwrap();
            let row_1 = lines_iter.next().unwrap();
            
            let row_0_parsed: Value = serde_json::from_str(&row_0).unwrap();
            let row_1_parsed: Value = serde_json::from_str(&row_1).unwrap();

            let comparison_result = are_values_sorted(&row_0_parsed, &row_1_parsed).unwrap();
            
            //println!("DEBUG - Pair {} is{} sorted.", pair_idx, if comparison_result {""} else {" not"});

            right_count += if comparison_result {pair_idx + 1} else {0};
        };
        right_count
    }
    pub fn day_13_2() -> usize {
        let mut contents = get_file_contents("input/13.txt");
        contents = contents.replace("\n\n", "\n");
        contents = contents.trim().to_string();
        let mut items = contents.split('\n').map(|s| serde_json::from_str(s).unwrap()).collect::<Vec<Value>>();

        items.push(Value::Array(vec![Value::Array(vec![Value::Number(Number::from(2))])]));
        items.push(Value::Array(vec![Value::Array(vec![Value::Number(Number::from(6))])]));
        
        items.sort_by(
            |a, b| match are_values_sorted(a, b) {
                None => Ordering::Equal,
                Some(x) => match x {
                    true => Ordering::Less,
                    false => Ordering::Greater
                }
            }
        );
        
        let mut val_1: usize = 0;
        let mut val_2: usize = 0;
    
        for (idx, item) in items.iter().enumerate() {
            if item == &Value::Array(vec![Value::Array(vec![Value::Number(Number::from(2))])]) {
                val_1 = idx + 1;
            }
            if item == &Value::Array(vec![Value::Array(vec![Value::Number(Number::from(6))])]) {
                val_2 = idx + 1;
            }
        };

        val_1 * val_2

    }
}

pub fn day_13() {
    println!("Day 13-1: {:?}", day_13::day_13_1());
    println!("Day 13-2: {:?}", day_13::day_13_2());
}
