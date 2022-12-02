pub fn day_one(riddle_input: &String) {
    let inventories: Vec<_> = riddle_input.split("\n\n").collect();
    let mut inventory_sums: Vec<_> = inventories.iter()
        .map(|&inventory| {
            inventory
                .split("\n")
                .map(|calorie_value:&str| calorie_value.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        }).collect();
    inventory_sums.sort_by(|a,b| b.cmp(a));
    println!("The maximum amount of calories for a single Elf is: {}", inventory_sums[0]);
    println!("The maximum amount of calories for three Elfs is: {}", inventory_sums[0] + inventory_sums[1] + inventory_sums[2])
}
