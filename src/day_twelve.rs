use crate::day_twelve::Directions::{DOWN, LEFT, RIGHT, UP};

pub fn day_twelve(riddle_input: &String) {
    let elevations = riddle_input
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|height_char| {
            return match height_char {
                'S' => 'a' as i32,
                'E' => 'z' as i32,
                _ => height_char as i32
            }
        }).collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();
    let start_string_index = riddle_input.find("S").expect("Invalid Format");
    let start_y = start_string_index / (elevations[0].len() + 1);
    let start_x = start_string_index % (elevations[0].len() + 1);
    let end_string_index = riddle_input.find("E").expect("Invalid Format");
    let end_y = end_string_index / (elevations[0].len() + 1);
    let end_x = end_string_index % (elevations[0].len() + 1);
    let path_grid = wavefront_propagation((start_x, start_y), &elevations);
    println!("Steps needed to reach top: {}", path_grid[end_y][end_x]);

    let start_positions = riddle_input
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == 'a')
        .map(|(i, _)| (i % (elevations[0].len() + 1), i / (elevations[0].len() + 1)))
        .collect::<Vec<(usize, usize)>>();
    let mut fewest_steps = path_grid[end_y][end_x];
    for pos in start_positions {
        let paths = wavefront_propagation(pos, &elevations);
        let length = paths[end_y][end_x];
        if length < fewest_steps {
            fewest_steps = length;
        }
    }
    println!("Fewest Steps from bottom to top is: {fewest_steps}");
}

fn wavefront_propagation(start_pos: (usize, usize), elevations: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let x_dim = elevations[0].len();
    let y_dim = elevations.len();
    let mut propagation_grid = vec![vec![i32::MAX; x_dim]; y_dim];
    let mut next_positions = vec![start_pos];
    propagation_grid[start_pos.1][start_pos.0] = 0;
    while !next_positions.is_empty() {
        let current_pos = next_positions.pop().expect("Vector empty");
        let current_distance = propagation_grid[current_pos.1][current_pos.0];
        let current_elevation = elevations[current_pos.1][current_pos.0];
        for direction in [UP, LEFT, DOWN, RIGHT] {
            let new_pos_opt = get_position(current_pos, direction, x_dim, y_dim);
            if new_pos_opt.is_some() {
                let new_pos = new_pos_opt.unwrap();
                let new_distance = propagation_grid[new_pos.1][new_pos.0];
                let new_elevation = elevations[new_pos.1][new_pos.0];
                if new_distance > current_distance + 1
                    && new_elevation <= current_elevation + 1 {
                    propagation_grid[new_pos.1][new_pos.0] = current_distance + 1;
                    next_positions.push(new_pos);
                }
            }
        }
    }
    propagation_grid
}

fn get_position(
    pos: (usize, usize),
    direction: Directions,
    x_dim: usize,
    y_dim: usize,
) -> Option<(usize, usize)> {
    match direction {
        UP => {
            let new_y = pos.1 + 1;
            if new_y < y_dim {
                return Some((pos.0, new_y));
            }
        }
        LEFT => {
            if pos.0 > 0 {
                return Some((pos.0 - 1, pos.1));
            }
        }
        DOWN => {
            if pos.1 > 0 {
                return Some((pos.0, pos.1 - 1));
            }
        }
        RIGHT => {
            let new_x = pos.0 + 1;
            if new_x < x_dim {
                return Some((new_x, pos.1));
            }
        }
    };
    None
}

enum Directions {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}
