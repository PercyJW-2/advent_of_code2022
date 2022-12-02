use std::env;
use std::fs;

mod day_one;
mod day_two;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let riddle_number = &args[2].parse::<i32>().unwrap();
    let file_contents = fs::read_to_string(filename).unwrap();

    match riddle_number {
        1 => day_one::day_one(&file_contents),
        2 => day_two::day_two(&file_contents),
        _ => println!("Riddle not Implemented")
    }
}
