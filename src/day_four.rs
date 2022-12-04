pub fn day_four(riddle_input: &String) {
    let section_assignment_pairs = riddle_input
        .split("\n")
        .filter(|assignment_pair| assignment_pair.len() > 0)
        .map(|assignment_pair:&str| assignment_pair.split_once(",").expect("Invalid Format"))
        .map(|(assignment_0, assignment_1):(&str,&str)| (assignment_0.split_once("-").expect("Invalid Format"), assignment_1.split_once("-").expect("Invalid Format")))
        .map(|((no1, no2), (no3, no4)):((&str,&str),(&str,&str))| ((no1.parse::<i32>().expect("invalid int"), no2.parse::<i32>().expect("invalid int")), (no3.parse::<i32>().expect("invalid int"), no4.parse::<i32>().expect("invalid int"))))
        .collect::<Vec<((i32,i32),(i32,i32))>>();
    let mut complete_overlapping_count = 0;
    let mut overlapping_count = 0;
    for assignment_pair in section_assignment_pairs {
        if assignment_pair.0.0 >= assignment_pair.1.0 && assignment_pair.0.0 <= assignment_pair.1.1 && assignment_pair.0.1 >= assignment_pair.1.0 && assignment_pair.0.1 <= assignment_pair.1.1 ||
           assignment_pair.1.0 >= assignment_pair.0.0 && assignment_pair.1.0 <= assignment_pair.0.1 && assignment_pair.1.1 >= assignment_pair.0.0 && assignment_pair.1.1 <= assignment_pair.0.1 {
            complete_overlapping_count += 1;
        }
        if assignment_pair.0.0 >= assignment_pair.1.0 && assignment_pair.0.0 <= assignment_pair.1.1 || assignment_pair.0.1 >= assignment_pair.1.0 && assignment_pair.0.1 <= assignment_pair.1.1 ||
            assignment_pair.1.0 >= assignment_pair.0.0 && assignment_pair.1.0 <= assignment_pair.0.1 || assignment_pair.1.1 >= assignment_pair.0.0 && assignment_pair.1.1 <= assignment_pair.0.1 {
            overlapping_count += 1;
        }
    }
    println!("Amount of completely overlapping associations: {complete_overlapping_count}");
    println!("Amount of overlapping associations: {overlapping_count}");
}
