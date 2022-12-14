mod day_11 {
    use std::fs::read_to_string;

    #[derive(Debug, Clone)]
    struct Test {
        divisor: usize,
        target_if_true: usize,
        target_if_false: usize,
    }

    #[derive(Debug, Clone)]
    enum Operator {
        Add,
        Mul,
    }

    #[derive(Debug, Clone)]
    struct Operation {
        operator: Operator,
        first_operand: Option<usize>,
        second_operand: Option<usize>,
    }

    #[derive(Debug, Clone)]
    struct Monkey {
        test: Test,
        operation: Operation,
        business: usize,
    }

    #[derive(Debug, Clone)]
    struct Item {
        value: usize,
        monkey: usize,
    }

    impl Monkey {
        fn apply_test(&self, item: &usize) -> bool {
            item % self.test.divisor == 0
        }
    }

    fn get_file_contents(filename: &str) -> String {
        read_to_string(filename).unwrap()
    }

    fn build_operation(operation_str: &str) -> Operation {
        let operation = operation_str.split("=").nth(1).unwrap().trim();
        let mut operation_elements = operation.split(" ").into_iter();

        let first_operand_str = operation_elements.next().unwrap();
        let first_operand = match first_operand_str {
            "old" => None,
            x => Some(x.parse::<usize>().unwrap()),
        };

        let operator_str = operation_elements.next().unwrap();
        let operator = match operator_str {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            x => panic!("Unexpected operator {}", x),
        };

        let second_operand_str = operation_elements.next().unwrap();
        let second_operand = match second_operand_str {
            "old" => None,
            x => Some(x.parse::<usize>().unwrap()),
        };

        Operation {
            operator,
            first_operand,
            second_operand,
        }
    }

    fn build_monkeys(monkeys: &mut Vec<Monkey>, items: &mut Vec<Item>, contents: &str) {
        for (monkey_idx, monkey_str) in contents.split("\n\n").into_iter().enumerate() {
            let lines = monkey_str.lines().collect::<Vec<&str>>();

            let items_str = lines[1].split("Starting items: ").nth(1).unwrap();
            let operation = build_operation(lines[2]);

            let monkey_items = items_str
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let parser = |s: &str, pattern: &str| {
                s.split(pattern)
                    .nth(1)
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap()
            };

            let divisor = parser(lines[3], "Test: divisible by");
            let target_if_true = parser(lines[4], "monkey");
            let target_if_false = parser(lines[5], "monkey");

            monkeys.push(Monkey {
                business: 0,
                operation,
                test: Test {
                    divisor,
                    target_if_true,
                    target_if_false,
                },
            });
            items.extend(monkey_items.iter().map(|mi| Item {
                value: *mi,
                monkey: monkey_idx,
            }));
        }
    }

    fn get_new_item_value_part_1(item: usize, operation: &Operation) -> usize {
        let first_operand = match operation.first_operand {
            None => item,
            Some(x) => x,
        };
        let second_operand = match operation.second_operand {
            None => item,
            Some(x) => x,
        };
        let result = match operation.operator {
            Operator::Add => first_operand + second_operand,
            Operator::Mul => first_operand * second_operand,
        };
        result / 3
    }

    fn get_new_item_value_part_2(
        item: usize,
        operation: &Operation,
        global_modulus: usize,
    ) -> usize {
        let first_operand = match operation.first_operand {
            None => item,
            Some(x) => x,
        };
        let second_operand = match operation.second_operand {
            None => item,
            Some(x) => x,
        };
        let result = match operation.operator {
            Operator::Add => first_operand + second_operand,
            Operator::Mul => first_operand * second_operand,
        };
        result % global_modulus
    }

    fn play_part_1(monkeys: &mut Vec<Monkey>, items: &mut Vec<Item>) {
        for _ in 0..20 {
            for (idx, monkey) in monkeys.iter_mut().enumerate() {
                for item in items.iter_mut() {
                    if item.monkey != idx {
                        continue;
                    }

                    let new_value = get_new_item_value_part_1(item.value, &monkey.operation);
                    item.monkey = if monkey.apply_test(&new_value) {
                        monkey.test.target_if_true
                    } else {
                        monkey.test.target_if_false
                    };
                    item.value = new_value;
                    monkey.business += 1;
                }
            }
        }
    }

    fn play_part_2(monkeys: &mut Vec<Monkey>, items: &mut Vec<Item>) {
        let global_modulus = monkeys.iter().fold(1, |acc, m| acc * m.test.divisor);
        for _ in 0..10000 {
            for (idx, monkey) in monkeys.iter_mut().enumerate() {
                for item in items.iter_mut() {
                    if item.monkey != idx {
                        continue;
                    }

                    let new_value =
                        get_new_item_value_part_2(item.value, &monkey.operation, global_modulus);
                    item.monkey = if monkey.apply_test(&new_value) {
                        monkey.test.target_if_true
                    } else {
                        monkey.test.target_if_false
                    };
                    item.value = new_value;
                    monkey.business += 1;
                }
            }
        }
    }

    pub fn day_11_1() -> usize {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut items: Vec<Item> = Vec::new();

        let contents = get_file_contents("input/11.txt");

        build_monkeys(&mut monkeys, &mut items, &contents);
        play_part_1(&mut monkeys, &mut items);

        monkeys.sort_by_key(|m| 0usize.overflowing_sub(m.business));
        monkeys[0].business * monkeys[1].business
    }
    pub fn day_11_2() -> usize {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut items: Vec<Item> = Vec::new();

        let contents = get_file_contents("input/11_ex.txt");

        build_monkeys(&mut monkeys, &mut items, &contents);
        play_part_2(&mut monkeys, &mut items);

        monkeys.sort_by_key(|m| 0usize.overflowing_sub(m.business));
        monkeys[0].business * monkeys[1].business
    }
}

pub fn day_11() {
    println!("Day 11-1: {:?}", day_11::day_11_1());
    println!("Day 11-2: {:?}", day_11::day_11_2());
}
