pub fn day_five(riddle_input: &String) {
    //let initial_stacks: Vec<Vec<char>> = Vec::new();
    //let move_allocations: Vec<(i32, i32, i32)> = Vec::new();

    let task_tuple = riddle_input.split_once("\n\n").expect("Invalid Format");
    let mut initial_stacks = parse_stacks(task_tuple.0);
    let move_allocation_strings = task_tuple.1
        .split("\n")
        .map(|statement| statement.replace("move ", ""))
        .map(|statement| statement.replace("from ", ""))
        .map(|statement| statement.replace("to ", ""))
        .filter(|statement| statement.len() > 0)
        .collect::<Vec<_>>();
    let move_allocations = move_allocation_strings
        .iter()
        .map(|statement| statement.split(" ").collect::<Vec<&str>>())
        .map(|statement| (statement[0].parse::<i32>().expect("NaN"), statement[1].parse::<i32>().expect("NaN"), statement[2].parse::<i32>().expect("NaN")))
        .collect::<Vec<(i32, i32, i32)>>();
    for stack_move in &move_allocations {
        for _ in 0..stack_move.0 {
            let elem = initial_stacks[(stack_move.1 - 1) as usize].pop().expect("Stack empty");
            initial_stacks[(stack_move.2 - 1) as usize].push(elem);
        }
    }
    println!("Solution when only one crate is movable at a time:");
    for stack in initial_stacks {
        println!("{:?}", stack);
    }
    initial_stacks = parse_stacks(task_tuple.0);
    for stack_move in &move_allocations {
        let mut tmp_vec: Vec<char> = Vec::new();
        for _ in 0..stack_move.0 {
            let elem = initial_stacks[(stack_move.1 - 1) as usize].pop().expect("Stack empty");
            tmp_vec.push(elem);
        }
        for _ in 0..stack_move.0 {
            initial_stacks[(stack_move.2 - 1) as usize].push(tmp_vec.pop().expect("Stack empty"))
        }
    }
    println!("Solution when multiple crates are movable at a time:");
    for stack in initial_stacks {
        println!("{:?}", stack);
    }
}

fn parse_stacks(text: &str) -> Vec<Vec<char>> {
    let mut texts = text.split("\n").collect::<Vec<&str>>();
    let stack_number: i32 = texts[texts.len() - 1]
        .rsplit_once(" ")
        .expect("Invalid Format").1
        .parse::<i32>()
        .expect("Invalid Number Format");
    texts.remove(texts.len() - 1);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..stack_number {
        stacks.push(Vec::new());
        for content_row in &texts {
            if i < content_row.len() as i32 / 2 {
                let char_option = content_row.chars().nth((i * 4 + 1) as usize).unwrap_or(' ');
                if char_option != ' ' {
                    stacks[i as usize].insert(0,char_option);
                }
            }
        }
    }
    return stacks;
}