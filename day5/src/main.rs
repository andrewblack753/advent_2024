use std::fs::read_to_string;

// this is so inefficient

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let contents: String = read_to_string(filename).expect("Should have been able to read the file");

    // One blank line between rules and checks
    // rules, then updates
    let parts: Vec<&str> = contents.split("\n\n").collect::<Vec<_>>();

    //Now break rules and updates into Vecs of Vecs for easier parsing

    let mut all_rules: Vec<Vec<&str>> =  Vec::<Vec::<&str>>::new();
    for line in parts[0].lines() {
        // should be two parts for each rule
        let within_line: Vec<&str> = line.split("|").collect::<Vec<_>>();
        all_rules.push(within_line);        
    }

    let mut all_updates: Vec<Vec<&str>> =  Vec::<Vec::<&str>>::new();
    for line in parts[1].lines() {
        let within_line: Vec<&str> = line.split(",").collect::<Vec<_>>();
        all_updates.push(within_line);
    }

    let mut correct_updates: Vec<Vec<&str>> =  Vec::<Vec::<&str>>::new();

    'updates: for update in all_updates {
        for rule in all_rules.clone() {
            let elem_one = rule[0];
            let elem_two = rule[1];

            if update.contains(&elem_one) && update.contains(&elem_two) {
                if update.iter().position(|&x| x == elem_one) > update.iter().position(|&y| y == elem_two) {
                    continue 'updates;
                }
            }
        }
        correct_updates.push(update);
    }

    println!("Found {} good updates", correct_updates.len());

    let mut added_middles: i32 = 0;
    for update in correct_updates {
        let the_val = update[update.len()/2];
        added_middles += the_val.parse::<i32>().unwrap();
    }

    println!("Result: {}", added_middles);

}
