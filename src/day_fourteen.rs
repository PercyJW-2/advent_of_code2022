pub fn day_fourteen(riddle_input: &String) {
    let mut cave_slice = vec![vec![' ']];
    for path_string in riddle_input.trim().split("\n") {
        let mut last_pos: Option<(usize, usize)> = None;
        for pos_str in path_string.split(" -> ") {
            let (x, y) = pos_str.split_once(",").expect("Invalid Format");
            let pos = (x.parse::<usize>().expect("Invalid Number"), y.parse::<usize>().expect("Invalid Number"));
            if cave_slice.len() <= pos.1 {
                for _ in cave_slice.len()..(pos.1 + 1) {
                    cave_slice.push(vec![]);
                }
            }
            for cave_slice_line in &mut cave_slice {
                if cave_slice_line.len() <= pos.0 {
                    for _ in cave_slice_line.len()..(pos.0 + 1) {
                        cave_slice_line.push(' ');
                    }
                }
            }
            if last_pos.is_some() {
                let prev_pos = last_pos.unwrap();
                if prev_pos.0 == pos.0 {
                    if prev_pos.1 < pos.1 {
                        for y in prev_pos.1..(pos.1 + 1) {
                            cave_slice[y][pos.0] = '#';
                        }
                    } else {
                        for y in pos.1..(prev_pos.1 + 1) {
                            cave_slice[y][pos.0] = '#';
                        }
                    }
                } else {
                    if prev_pos.0 < pos.0 {
                        for x in prev_pos.0..(pos.0 + 1) {
                            cave_slice[pos.1][x] = '#';
                        }
                    } else {
                        for x in pos.0..(prev_pos.0 + 1) {
                            cave_slice[pos.1][x] = '#';
                        }
                    }
                }
            }
            last_pos = Some(pos);
        }
    }
    simulate_sand((500, 0), cave_slice.clone());
    cave_slice.push(vec![' '; cave_slice[0].len()]);
    cave_slice.push(vec![' '; cave_slice[0].len()]);
    simulate_sand_until_full((500, 0), cave_slice.clone());
}

fn simulate_sand(start_pos: (usize, usize), mut cave_slice: Vec<Vec<char>>) {
    let mut sand_reached_abyss = false;
    let mut current_sand_pos = start_pos;
    let mut sand_count = 0;
    while !sand_reached_abyss {
        if cave_slice.len() == current_sand_pos.1 + 1 {
            sand_reached_abyss = true;
        } else if cave_slice[current_sand_pos.1 + 1][current_sand_pos.0] == ' ' {
            current_sand_pos.1 += 1;
        } else if current_sand_pos.0 > 0 && cave_slice[current_sand_pos.1 + 1][current_sand_pos.0 - 1] == ' ' {
            current_sand_pos.1 += 1;
            current_sand_pos.0 -= 1;
        } else if cave_slice[current_sand_pos.1 + 1].len() == current_sand_pos.0 + 1 {
            sand_reached_abyss = true;
        } else if cave_slice[current_sand_pos.1 + 1][current_sand_pos.0 + 1] == ' ' {
            current_sand_pos.1 += 1;
            current_sand_pos.0 += 1;
        } else {
            cave_slice[current_sand_pos.1][current_sand_pos.0] = 'O';
            current_sand_pos = start_pos;
            sand_count += 1;
        }
    }
    println!("Units of sand until they flow into the abyss: {sand_count}");
}

fn simulate_sand_until_full(start_pos: (usize, usize), mut cave_slice: Vec<Vec<char>>) {
    let mut cave_full = false;
    let mut current_sand_pos = start_pos;
    let mut sand_count = 0;
    while !cave_full {
        if cave_slice.len() == current_sand_pos.1 + 1 {
            cave_slice[current_sand_pos.1][current_sand_pos.0] = 'O';
            current_sand_pos = start_pos;
        } else if cave_slice[current_sand_pos.1 + 1][current_sand_pos.0] == ' ' {
            current_sand_pos.1 += 1;
        } else if current_sand_pos.0 > 0 && cave_slice[current_sand_pos.1 + 1][current_sand_pos.0 - 1] == ' ' {
            current_sand_pos.1 += 1;
            current_sand_pos.0 -= 1;
        } else if cave_slice[current_sand_pos.1 + 1].len() == current_sand_pos.0 + 1 {
            for cave_slice_line in &mut cave_slice {
                cave_slice_line.push(' ');
            }
        } else if cave_slice[current_sand_pos.1 + 1][current_sand_pos.0 + 1] == ' ' {
            current_sand_pos.1 += 1;
            current_sand_pos.0 += 1;
        } else {
            cave_slice[current_sand_pos.1][current_sand_pos.0] = 'O';
            if current_sand_pos == start_pos {
                cave_full = true
            }
            current_sand_pos = start_pos;
            sand_count += 1;
        }
    }
    println!("Units of sand until they flow into the abyss: {sand_count}");
}
