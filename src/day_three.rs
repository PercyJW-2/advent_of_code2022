pub fn day_three(riddle_input: &String) {
    let backpacks = riddle_input
        .split("\n")
        .map(|backpack: &str| (&backpack[0..(backpack.len()/2)], &backpack[(backpack.len()/2)..backpack.len()]))
        .collect::<Vec<_>>();
    part_one(&backpacks);
    let unprocessed_backpacks = riddle_input
        .split("\n")
        .collect::<Vec<&str>>();
    let groups = unprocessed_backpacks
        .chunks(3)
        .collect::<Vec<_>>();
    part_two(&groups)
}

fn part_one(backpacks: &Vec<(&str, &str)>) {
    let mut points = 0;
    for backpack in backpacks {
        for i in 0..backpack.0.len() {
            let current_type: char = backpack.0.as_bytes()[i] as char;
            if backpack.1.contains(current_type) {
                if current_type.is_ascii_uppercase() {
                    points += (current_type as i32 - 'A' as i32) + 27;
                } else {
                    points += current_type as i32 - 'a' as i32 + 1;
                }
                break;
            }
        }
    }
    println!("Sum of Priorities: {points}");
}

fn part_two(groups: &Vec<&[&str]>) {
    let mut points = 0;
    for group in groups {
        for i in 0..group[0].len() {
            let current_type: char = group[0].as_bytes()[i] as char;
            if group[1].contains(current_type) && group[2].contains(current_type) {
                if current_type.is_ascii_uppercase() {
                    points += (current_type as i32 - 'A' as i32) + 27;
                } else {
                    points += current_type as i32 - 'a' as i32 + 1;
                }
                break;
            }
        }
    }
    println!("Sum of Badges: {points}");
}
