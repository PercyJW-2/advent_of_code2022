use std::env;
use std::fs;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let riddle_number = &args[2].parse::<i32>().unwrap();
    let file_contents = fs::read_to_string(filename).unwrap();

    match riddle_number {
        1 => day_one::day_one(&file_contents),
        2 => day_two::day_two(&file_contents),
        3 => day_three::day_three(&file_contents),
        4 => day_four::day_four(&file_contents),
        5 => day_five::day_five(&file_contents),
        6 => day_six::day_six(&file_contents),
        7 => day_seven::day_seven(&file_contents),
        _ => println!("Riddle not Implemented")
    }
}
