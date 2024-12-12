use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");
    let contents = read_to_string(filename).expect("Should have been able to read the file");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    //let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
    //let matches = re.captures_iter(&contents).map(|m| m.extract());

    let mut sum: i32 = 0;

    // discard the offset of the match itself
    for (_, [first, second]) in re.captures_iter(&contents).map(|m| m.extract()) {
        let product =  first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        sum += product;
    }

    println!("{}", sum)

    // each match is a string representing the pair
    // for a_match in matches {
    //     print!("{}\n", a_match);
    // }

    // println!("{}", matches[1])

    // let re2 = Regex::new(r"mul\((.+),(.+)\)").unwrap();
}
