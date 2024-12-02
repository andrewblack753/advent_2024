use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let file = File::open(filename).expect("Unable to find file");
    let reader: BufReader<&File> = BufReader::new(&file);

    // You would not believe how much trouble this gave me for some reason
    let mut left_numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().split("   ").collect::<Vec<_>>()[0].parse::<i64>().unwrap())
        .collect();

    let file = File::open(filename).expect("Unable to find file");
    let reader: BufReader<&File> = BufReader::new(&file);

    let mut right_numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().split("   ").collect::<Vec<_>>()[1].parse::<i64>().unwrap())
        .collect();

    left_numbers.sort();
    right_numbers.sort();

    let mut i = left_numbers.len();
    let mut diff: i64 = 0;

    while i > 0 {
        i -= 1;
        let delta = left_numbers[i] - right_numbers[i];
        diff += delta.abs();
        // println!("Diff between {} and {} is {}", left_numbers[i], right_numbers[i], delta)
    }

    println!("Total difference is {diff}");

    // Part 2
    let mut i = left_numbers.len();
    let right_counts = right_numbers.clone().into_iter().counts();    
    let mut similarity: i64 = 0;

    while i > 0 {
        i -= 1;

        let left_val: i64 = left_numbers[i];

        // Value may not appear in right column at all
        if right_counts.contains_key(&left_val) {
            let right_count = right_counts[&left_val];
            let scoremod: i64 = left_numbers[i] * i64::try_from(right_count).unwrap();
            // println!("Value {} appears on right {} times, for an increase of {}", left_numbers[i], right_count, scoremod);
            similarity += scoremod;
        }
    }

    println!("Total Similarity is {similarity}");

}