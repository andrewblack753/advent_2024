use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let contents: String = read_to_string(filename).expect("Should have been able to read the file");
    let mut the_grid: Vec<Vec<char>> = contents.lines().map(|line: &str| line.chars().collect()).collect();

    let row_count = the_grid.len();
    let col_count = the_grid[0].len();

    // check until the guard navigates off-map
    // this means that the input is set out such that loops are not possible

    let guard_icon: char = '^';
    let mut y_coord = 0;
    let mut x_coord = 0;

    // find starting guard location
    'rows: while y_coord < row_count {
        if the_grid[y_coord].contains(&guard_icon) {
            x_coord = the_grid[y_coord].iter().position(|&x| x == guard_icon).unwrap();
            break 'rows;
        }
        y_coord += 1;
    }

    // 0 north, 1 east, 2 south, 3 west
    let mut guard_orientation = 0;

    let mut score = 1;
    the_grid[y_coord][x_coord] = 'X';


    // walk through until bounds check
    'run_though: loop {
        let mut obstructed = false;
        match guard_orientation {
            0 => {
                println!("north");
                if y_coord == 0 {
                    println!("done");
                    break 'run_though;
                }
                if the_grid[y_coord-1][x_coord] == '#' {
                    obstructed = true;
                } else {
                    y_coord -= 1;
                    if the_grid[y_coord][x_coord] == '.' {
                        the_grid[y_coord][x_coord] = 'X';
                        score += 1;
                    }
                }
            }

            1 => {
                println!("east");
                if x_coord == col_count {
                    println!("done");                    
                    break 'run_though;
                }
                if the_grid[y_coord][x_coord+1] == '#' {
                    obstructed = true;
                } else {
                    x_coord += 1;
                    if the_grid[y_coord][x_coord] == '.' {
                        the_grid[y_coord][x_coord] = 'X';
                        score += 1;
                    }
                }
            }

            2 => {
                println!("south");
                if y_coord == row_count {
                    println!("done");                    
                    break 'run_though;
                }
                if the_grid[y_coord+1][x_coord] == '#' {
                    obstructed = true;
                } else {
                    y_coord += 1;
                    if the_grid[y_coord][x_coord] == '.' {
                        the_grid[y_coord][x_coord] = 'X';
                        score += 1;
                    }
                }
            }

            3 => {
                println!("west");
                if x_coord == 0 {
                    println!("done");                    
                    break 'run_though;
                }
                if the_grid[y_coord][x_coord-1] == '#' {
                    obstructed = true;
                } else {
                    x_coord -= 1;
                    if the_grid[y_coord][x_coord] == '.' {
                        the_grid[y_coord][x_coord] = 'X';
                        score += 1;
                    }
                }                
            }

            _ => {
                println!("Something has gone terribly wrong.");
                break 'run_though;
            }                    
        }

        if obstructed {
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
}  