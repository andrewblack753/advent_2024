use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");
    let contents: String = read_to_string(filename).expect("Should have been able to read the file");

    let re: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    //let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
    //let matches = re.captures_iter(&contents).map(|m| m.extract());

    let mut sum: i32 = 0;

    // discard the offset of the match itself
    for (_, [first, second]) in re.captures_iter(&contents).map(|m: regex::Captures<'_>| m.extract()) {
        let product: i32 =  first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        sum += product;
    }

    println!("{}", sum);
    
    let mut sum2: i32 = 0;
    let sub_strings: Vec<&str> = contents.split("don't()").collect::<Vec<_>>();

    let mut count_index: usize = 1;
    let sub_string: &str = sub_strings[0];
    // mul()s before first don't() are all valid
    for (_, [first, second]) in re.captures_iter(&sub_string).map(|m: regex::Captures<'_>| m.extract()) {
        let product: i32 =  first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        sum2 += product;
    }

    while count_index < sub_strings.len() {
        let sub_string: &str = sub_strings[count_index];
        let index: Option<usize> = sub_string.find("do()");
        println!("{}", sub_string);
        if index.is_some() {
            let the_index: usize = index.unwrap();
            let sub_string: &str  = &sub_string[the_index..];
            for (_, [first, second]) in re.captures_iter(&sub_string).map(|m: regex::Captures<'_>| m.extract()) {
                let product: i32 =  first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
                sum2 += product;
            }          

        }
        count_index += 1;
    }
    println!("{}", sum2);

}
