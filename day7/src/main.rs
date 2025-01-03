use std::fs::read_to_string;

fn main() {
    let filename = "input";
    println!("Processing {filename}");

    let contents: String = read_to_string(filename).expect("Should have been able to read the file");

    let mut test_values: Vec<u128> = vec![];
    let mut numbers: Vec<Vec<u128>> = vec![];

    for line in contents.lines() {
        let (first, parts) = line.split_once(":").unwrap();

        test_values.push(first.parse::<u128>().unwrap());

        let as_ints: Vec<u128> = parts.split(' ').filter(|&x| !x.is_empty() ).map(|x| x.parse::<u128>().unwrap()).collect();
        numbers.push(as_ints);
    }

    let num_cases: usize = test_values.len();
    let mut score = 0;

    for index in 0..num_cases {
        if try_all(test_values[index], &numbers[index][1..], numbers[index][0]) {
            score += test_values[index];
        }
    }
    println!("Result: {}", score);
}

fn try_all(test_value: u128, values: &[u128], sum: u128) -> bool {
    if values.len() == 1 {
        if test_value == values[0] + sum || test_value == values[0] * sum {
            return true;
        }
    }  else {
        if try_all(test_value, &values[1..], values[0] + sum)
            || try_all(test_value, &values[1..], values[0] * sum) {
            return true;
        }
    }
    return false;
}