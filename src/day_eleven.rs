use std::cell::RefCell;
use std::rc::Rc;

pub fn day_eleven(riddle_input: &String) {
    let mut monkey_vec: Vec<Rc<RefCell<Monkey>>> = vec![];
    parse_input(riddle_input, &mut monkey_vec);
    execute_monkey_business(&mut monkey_vec, 20, 3);
    monkey_vec = vec![];
    parse_input(riddle_input, &mut monkey_vec);
    execute_monkey_business(&mut monkey_vec, 10000, 1);
}

fn parse_input(riddle_input: &String, monkey_vec: &mut Vec<Rc<RefCell<Monkey>>>) {
    for monkey_string in riddle_input.split("\n\n") {
        let mut items: Vec<u64> = vec![];
        let mut operator = ' ';
        let mut operation_value = 0;
        let mut test_number = 0;
        let mut monkey_true = 0;
        let mut monkey_false = 0;
        for line in monkey_string.split("\n") {
            if line.starts_with("  Starting items: ") {
                let item_string = line.split_once(": ").expect("Invalid Format").1;
                items = item_string
                    .split(", ")
                    .map(|item| item.parse::<u64>().expect("Invalid Number"))
                    .collect();
            } else if line.starts_with("  Operation: ") {
                let operation_str = line.split_once(" old ").expect("Invalid Format").1;
                let operation = operation_str.split_once(" ").expect("Invalid Format");
                operator = operation.0.chars().nth(0).expect("Invalid Format");
                let operation_value_result = operation.1.parse();
                if operation_value_result.is_err() {
                    operator = '^';
                    operation_value = 1;
                } else {
                    operation_value = operation_value_result.unwrap();
                }
            } else if line.starts_with("  Test: ") {
                let test_tup = line.split_once(" by ").expect("Invalid Format");
                test_number = test_tup.1.parse().expect("Invalid Number");
            } else if line.starts_with("    If true: ") {
                let true_tup = line.split_once(" monkey ").expect("Invalid Format");
                monkey_true = true_tup.1.parse().expect("Invalid Number");
            } else if line.starts_with("    If false: ") {
                let false_tup = line.split_once(" monkey ").expect("Invalid Format");
                monkey_false = false_tup.1.parse().expect("Invalid Number");
            }
        }
        monkey_vec.push(Rc::new(RefCell::new(Monkey {
            items,
            operator,
            operation_value,
            test_number,
            monkey_true,
            monkey_false,
        })));
    }
}

fn execute_monkey_business(monkey_vec: &mut Vec<Rc<RefCell<Monkey>>>, rounds: i32, worry_management: u64) {
    let mut item_interactions = vec![0u64; monkey_vec.len()];
    let operator_mult: u64 = monkey_vec.iter().map(|monkey| monkey.borrow().test_number).product();
    for _round in 0..rounds {
        let mut round_interactions: Vec<u64> = vec![];
        for monkey in monkey_vec.iter() {
            round_interactions.push(monkey.borrow().items.len() as u64);
            for item in &monkey.borrow().items {
                let mut new_value: u64 = match monkey.borrow().operator {
                    '*' => *item * monkey.borrow().operation_value,
                    '+' => *item + monkey.borrow().operation_value,
                    '^' => *item * *item,
                    _ => *item,
                } / worry_management;
                new_value %= operator_mult;
                if new_value % monkey.borrow().test_number == 0 {
                    monkey_vec[monkey.borrow().monkey_true]
                        .borrow_mut()
                        .items
                        .push(new_value);
                } else {
                    monkey_vec[monkey.borrow().monkey_false]
                        .borrow_mut()
                        .items
                        .push(new_value);
                }
            }
            monkey.borrow_mut().items = vec![];
        }
        item_interactions = item_interactions.iter().zip(round_interactions.iter()).map(|tup| tup.0 + tup.1).collect();
    }
    item_interactions.sort_by(|a, b| b.cmp(a));
    println!("Level of monkey business after {rounds} rounds: {}", item_interactions[0] * item_interactions[1]);
}

struct Monkey {
    items: Vec<u64>,
    operator: char,
    operation_value: u64,
    test_number: u64,
    monkey_true: usize,
    monkey_false: usize,
}
