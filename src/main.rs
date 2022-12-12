use std::env;
use std::fs;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;
mod day_ten;
mod day_eleven;
mod day_twelve;

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
        8 => day_eight::day_eight(&file_contents),
        9 => day_nine::day_nine(&file_contents),
        10 => day_ten::day_ten(&file_contents),
        11 => day_eleven::day_eleven(&file_contents),
        12 => day_twelve::day_twelve(&file_contents),
        _ => println!("Riddle not Implemented")
    }
}
