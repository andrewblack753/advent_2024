use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let contents: String = read_to_string(filename).expect("Should have been able to read the file");

    // https://www.youtube.com/watch?v=4-J4duzP8Ng
    let the_grid: Vec<Vec<char>> = contents.lines().map(|line: &str| line.chars().collect()).collect();
    // ty nathandchin; tried using ndarray but didn't get very far

    // 140x140
    let row_count = the_grid.len();
    let col_count = the_grid[0].len();

    let target: String = "XMAS".to_string();
    // instead of tracking on 2s and 3s for indexing below, could make dependent on len of target string

    // SAMX
    let target_rev: String  = target.chars().rev().collect::<String>();

    let mut jolly_count: i32 = 0;

    for i in 0..row_count {
        for j in 0..col_count {
            if j < col_count - 3 {
                let horizontal: String = [the_grid[i][j], the_grid[i][j+1], the_grid[i][j+2], the_grid[i][j+3]].into_iter().collect();
                if horizontal == target || horizontal == target_rev {
                    jolly_count += 1
                }
            }
            if i > 2 && j < col_count - 3{
                let diag_up: String = [the_grid[i][j], the_grid[i-1][j+1], the_grid[i-2][j+2], the_grid[i-3][j+3]].into_iter().collect();
                if diag_up == target || diag_up == target_rev {
                    jolly_count += 1;
                }                
            }
            if i < col_count - 3 {
                let vertical: String = [the_grid[i][j], the_grid[i+1][j], the_grid[i+2][j], the_grid[i+3][j]].into_iter().collect();
                if vertical == target || vertical == target_rev {
                    jolly_count += 1;
                }              
            }
            if i < col_count - 3 && j < row_count -3 {            
                let diag_down: String = [the_grid[i][j], the_grid[i+1][j+1], the_grid[i+2][j+2], the_grid[i+3][j+3]].into_iter().collect();
                if diag_down == target || diag_down == target_rev {
                    jolly_count += 1;
                }
            }
        }
    }
    println!("XMAS count: {} ", jolly_count);

}