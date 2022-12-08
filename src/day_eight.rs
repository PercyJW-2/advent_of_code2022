pub fn day_eight(riddle_input: &String) {
    let mut grid: Vec<Vec<i32>> = vec![];
    for row in riddle_input.split("\n") {
        let mut row_vec: Vec<i32> = vec![];
        for tree in row.chars() {
            row_vec.push(tree as i32);
        }
        if row_vec.len() > 0 {
            grid.push(row_vec);
        }
    }
    let mut visible_count = grid.len() * 2 + (grid[0].len() - 2) * 2;
    let mut max_view_dist = 0;
    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[row].len() - 1) {
            if  check_above(&grid, (col, row)) ||
                check_below(&grid, (col, row)) ||
                check_left(&grid, (col, row)) ||
                check_right(&grid, (col, row)) {
                visible_count += 1;
            }
            let above = get_distance_above(&grid, (col, row));
            let below = get_distance_below(&grid, (col, row));
            let left = get_distance_left(&grid, (col, row));
            let right = get_distance_right(&grid, (col, row));
            let view_dist = above * below * left * right;
            if view_dist > max_view_dist {
                //println!("{above} {left} {below} {right}");
                max_view_dist = view_dist;
            }
        }
    }
    println!("{visible_count} Trees are visible.");
    println!("{max_view_dist} Is the maximum scenic score");
}

fn check_above(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> bool {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    for row in (0..position.1).rev() {
        if tree_height_to_check <= tree_grid[row][position.0] {
            return false;
        }
    }
    true
}

fn check_below(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> bool {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    for row in (position.1+1)..tree_grid.len() {
        if tree_height_to_check <= tree_grid[row][position.0] {
            return false;
        }
    }
    true
}

fn check_left(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> bool {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    for col in (0..position.0).rev() {
        if tree_height_to_check <= tree_grid[position.1][col] {
            return false;
        }
    }
    true
}

fn check_right(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> bool {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    for col in (position.0 + 1)..tree_grid[position.1].len() {
        if tree_height_to_check <= tree_grid[position.1][col] {
            return false;
        }
    }
    true
}

fn get_distance_above(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> i32 {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    let mut view_distance = 0;
    for row in (0..position.1).rev() {
        let current_height = tree_grid[row][position.0];
        if tree_height_to_check > current_height {
            view_distance += 1;
        } else if tree_height_to_check == current_height {
            return view_distance + 1;
        } else {
            return view_distance + 1;
        }
    }
    view_distance
}

fn get_distance_below(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> i32 {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    let mut view_distance = 0;
    for row in (position.1 + 1)..tree_grid.len() {
        let current_height = tree_grid[row][position.0];
        if tree_height_to_check > current_height {
            view_distance += 1;
        } else if tree_height_to_check == current_height {
            return view_distance + 1;
        } else {
            return view_distance + 1;
        }
    }
    view_distance
}

fn get_distance_left(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> i32 {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    let mut view_distance = 0;
    for col in (0..position.0).rev() {
        let current_height = tree_grid[position.1][col];
        if tree_height_to_check > current_height {
            view_distance += 1;
        } else if tree_height_to_check == current_height {
            return view_distance + 1;
        } else {
            return view_distance + 1;
        }
    }
    view_distance
}

fn get_distance_right(tree_grid: &Vec<Vec<i32>>, position: (usize, usize)) -> i32 {
    let tree_height_to_check: i32 = tree_grid[position.1][position.0];
    let mut view_distance = 0;
    for col in (position.0 + 1)..tree_grid.len() {
        let current_height = tree_grid[position.1][col];
        if tree_height_to_check > current_height {
            view_distance += 1;
        } else if tree_height_to_check == current_height {
            return view_distance + 1;
        } else {
            return view_distance + 1;
        }
    }
    view_distance
}
