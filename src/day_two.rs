use std::process::exit;

pub fn day_two(riddle_input: &String) {
    let duel_strings = riddle_input
        .split("\n")
        .map(|round| round.split(" ").collect::<Vec<_>>())
        .filter(|round| round.len() == 2)
        .map(|round| (round[0], round[1]))
        .collect::<Vec<_>>();
    println!("Number of Duels: {}", duel_strings.len());
    let mut points: i32 = 0;
    for duel in &duel_strings {
        points += get_points_from_duel(duel);
    }
    println!("The total score is {points} points.");

    points = 0;
    for duel in &duel_strings {
        let mut new_duel: (&str, &str) = (duel.0, "");
        new_duel.1 = match duel.1 {
            "X" => match duel.0 {
                "A" => "Z",
                "B" => "X",
                "C" => "Y",
                _ => exit(-1)
            },
            "Y" => match duel.0 {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => exit(-1)
            },
            "Z" => match duel.0 {
                "A" => "Y",
                "B" => "Z",
                "C" => "X",
                _ => exit(-1)
            }
            _ => exit(-1)
        };
        points += get_points_from_duel(&new_duel);
    }
    println!("The total score is {points} points.");
}

fn get_points_from_duel(duel: &(&str, &str)) -> i32 {
    let mut points: i32 = 0;
    points += match duel.1 {
        "X"=> 1,
        "Y"=> 2,
        "Z"=> 3,
        _ => 0
    };
    points += match duel {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "Z") => 3,
        ("C", "X") => 6,
        _ => 0
    };
    return points;
}
