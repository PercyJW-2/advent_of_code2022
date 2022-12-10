pub fn day_ten(riddle_input: &String) {
    let mut register_history: Vec<i32> = vec![];
    let mut register_x = 1;
    for command in riddle_input.split("\n") {
        if command == "" {
            break;
        }
        if command == "noop" {
            register_history.push(register_x);
        } else {
            register_history.push(register_x);
            register_history.push(register_x);
            let cmd_tuple = command.split_once(" ").expect("Invalid Format");
            let value = cmd_tuple.1.parse::<i32>().expect("Invalid Number");
            register_x += value;
        }
    }
    solution_one(&register_history);
    solution_two(&register_history);
}

fn solution_one(x_hist: &Vec<i32>) {
    const CYCLES_TO_CHECK: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut signal_strength_sum = 0;
    for cycle in CYCLES_TO_CHECK {
        let register_value = x_hist[(cycle - 1) as usize];
        println!("Cycle: {cycle}\t Value: {register_value}");
        signal_strength_sum += cycle * register_value;
    }
    println!("The sum of the selected signal strengths is: {signal_strength_sum}");
}

fn solution_two(x_hist: &Vec<i32>) {
    for cycle in 0..240 {
        let x_pos = cycle % 40;
        if x_pos == 0 {
            println!();
        }
        let register_val = x_hist[cycle];
        if x_pos == (register_val + 1) as usize || x_pos == register_val as usize || x_pos == (register_val - 1) as usize {
            print!("#");
        } else {
            print!(".");
        }
    }
}
