use std::collections::HashSet;

pub fn day_nine(riddle_input: &String) {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut chain_poses = [(0, 0); 9];
    let mut tail_pos_set: HashSet<(i32, i32)> = HashSet::new();
    let mut chain_poses_tail_set: HashSet<(i32, i32)> = HashSet::new();
    let mut tail_move_count = 0;
    for move_command_str in riddle_input.split("\n") {
        if move_command_str == "" {
            break;
        }
        let move_command_tuple_str = move_command_str
            .split_once(" ")
            .expect("Invalid Format");
        let move_direction = move_command_tuple_str.0;
        let move_amount = move_command_tuple_str.1.parse::<i32>().expect("Not a number");
        let relative_move = match move_direction {
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            "R" => (1, 0),
            _ => (0, 0)
        };
        for _ in 0..move_amount {
            head_pos.0 += relative_move.0;
            head_pos.1 += relative_move.1;
            let relative_tail_move = get_tail_move(tail_pos, head_pos);
            tail_pos.0 += relative_tail_move.0;
            tail_pos.1 += relative_tail_move.1;
            tail_pos_set.insert(tail_pos);
            let mut prev_pos = head_pos;
            for pos in &mut chain_poses {
                let rel_move = get_tail_move(*pos, prev_pos);
                pos.0 += rel_move.0;
                pos.1 += rel_move.1;
                prev_pos = *pos;
            }
            chain_poses_tail_set.insert(chain_poses[chain_poses.len() - 1]);
            tail_move_count += 1;
        }
        println!();
        print_chain_state(chain_poses);
        println!();
        println!("------");
    }
    let tail_pos_count = tail_pos_set.len();
    let chain_pos_count = chain_poses_tail_set.len();
    println!("The Tail moved {tail_move_count} times");
    println!("The Tail visited {tail_pos_count} positions");
    println!("The Chain visited {chain_pos_count} positions");
}

fn is_neighbour (tail_pos: (i32, i32), head_pos: (i32, i32)) -> bool {
    if tail_pos.0 < head_pos.0 - 1 || tail_pos.0 > head_pos.0 + 1 {
        return false;
    }
    if tail_pos.1 < head_pos.1 - 1 || tail_pos.1 > head_pos.1 +1 {
        return false;
    }
    true
}

fn get_tail_move(tail_pos: (i32, i32), new_head_pos: (i32, i32)) -> (i32, i32) {
    let mut relative_move = (0, 0);
    if is_neighbour(tail_pos, new_head_pos) {
        return relative_move;
    }
    if tail_pos.0 == new_head_pos.0 {
        relative_move.0 = 0;
    } else if tail_pos.0 < new_head_pos.0 {
        // tail is left
        relative_move.0 = 1;
    } else {
        // tail is right
        relative_move.0 = -1;
    }
    if tail_pos.1 == new_head_pos.1 {
        relative_move.1 = 0;
    } else if tail_pos.1 < new_head_pos.1 {
        // tail is below
        relative_move.1 = 1;
    } else {
        // tail is above
        relative_move.1 = -1;
    }
    relative_move
}

fn print_chain_state(chain_ref: [(i32, i32); 9]) {
    let x_poses = chain_ref.map(|pos| pos.0);
    let min_x = x_poses.iter().min().expect("No poses");
    // let max_x = chain_ref.map(|pos| pos.0).iter().max().expect("No poses");
    let y_poses = chain_ref.map(|pos| pos.1);
    let min_y = y_poses.iter().min().expect("No poses");
    // let max_y = chain_ref.map(|pos| pos.1).iter().max().expect("No poses");
    let mut field_section = [['.'; 10]; 10];
    for pos in chain_ref {
        field_section[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = '#';
    }
    for y in (0..10).rev() {
        for x in 0..10 {
            print!("{}", field_section[y][x]);
        }
        println!();
    }
}
