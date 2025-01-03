use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let contents: String = read_to_string(filename).expect("Should have been able to read the file");
    let mut the_grid: Vec<Vec<char>> = contents.lines().map(|line: &str| line.chars().collect()).collect();

    let mut the_grid_2 = the_grid.clone();

    let row_count = the_grid.len();
    let col_count = the_grid[0].len();

    // check until the guard navigates off-map
    // this means that the input is set out such that loops are not possible

    let guard_icon: char = '^';
    let mut y_coord_orig = 0;
    let mut x_coord_orig = 0;

    // find starting guard location
    'rows: while y_coord_orig < row_count {
        if the_grid[y_coord_orig].contains(&guard_icon) {
            x_coord_orig = the_grid[y_coord_orig].iter().position(|&x| x == guard_icon).unwrap();
            break 'rows;
        }
        y_coord_orig += 1;
    }

    let mut y_coord = y_coord_orig;
    let mut x_coord = x_coord_orig;

    // 0 north, 1 east, 2 south, 3 west
    let mut guard_orientation = 0;

    let mut score = 1;
    the_grid[y_coord][x_coord] = 'X';


    // walk through until bounds check
    'run_though: loop {
        let mut obstructed = false;
        match guard_orientation {
            0 => {
                if y_coord == 0 {
                    break 'run_though;
                }
                if the_grid[y_coord-1][x_coord] == '#' {
                    obstructed = true;
                } else {
                    y_coord -= 1;
                }
            }

            1 => {
                if x_coord == col_count {
                    break 'run_though;
                }
                if the_grid[y_coord][x_coord+1] == '#' {
                    obstructed = true;
                } else {
                    x_coord += 1;
                }
            }

            2 => {
                if y_coord == row_count {
                    break 'run_though;
                }
                if the_grid[y_coord+1][x_coord] == '#' {
                    obstructed = true;
                } else {
                    y_coord += 1;
                }
            }

            3 => {
                if x_coord == 0 {
                    break 'run_though;
                }
                if the_grid[y_coord][x_coord-1] == '#' {
                    obstructed = true;
                } else {
                    x_coord -= 1;
                }                
            }

            _ => ()
        }

        if !obstructed {
            if the_grid[y_coord][x_coord] == '.' {
                match guard_orientation {
                    0 | 2 => {
                        the_grid[y_coord][x_coord] = '|';
                    }
                    1 | 3 => {
                        the_grid[y_coord][x_coord] = '-';
                    }
                    _ => ()
                }
            
            // score only increments on totally new square
            score += 1;
            }

            else if the_grid[y_coord][x_coord] == '-' && (guard_orientation == 0 || guard_orientation == 2)
                || the_grid[y_coord][x_coord] == '|' && (guard_orientation == 1 || guard_orientation == 3) {
                    the_grid[y_coord][x_coord] = '+'; 
                }


        } else {
            guard_orientation = (guard_orientation + 1) % 4;
        }

    } 

    // println!("\n\n\n");
    // for row in the_grid {
    //     for cha in row {
    //         print!("{}", cha);
    //     }
    //     println!("\n");
    // }

    println!("Score: {}", score);

    // Day 6b
    // the puzzle text is a hint - you can use the '|', '-', '+' chars to track if the guard has passed that way already

    // iterate through final grid of previous puzzle
    // for each X check whether diverging there would cause a loop
}  