use std::fs::read_to_string;



fn main() {
    let filename = "input.data";
    println!("Processing {filename}");

    let mut all_reports = Vec::<Vec::<i32>>::new();
    let mut failing_reports = Vec::<Vec::<i32>>::new();

    let contents = read_to_string(filename).expect("Should have been able to read the file");
    for line in contents.lines() {
        all_reports.push(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect());
        //println!("{}", int_line[0])
    }

    let total_reports = all_reports.len();
    let mut safe_reports = 0;

    for report in all_reports {
        let is_valid: bool;

        // println!("{:?}", report);

        if report[0] > report[1] {
            // w is a 2-element window into the levels of report
            // check monotonic / not a diff greater than 4

            // could use is_sorted() but that does <= rather than <

            is_valid = (report.windows(2).all(|w| w[0] > w[1])) && (report.windows(2).all(|w| (w[0] - w[1]).abs() < 4 ));
        } else {
            is_valid = (report.windows(2).all(|w| w[0] < w[1])) && (report.windows(2).all(|w| (w[0] - w[1]).abs() < 4 ));
        }

        if is_valid {
            safe_reports += 1;
        } else {
            failing_reports.push(report)
        }
    }

    println!("{} safe reports", safe_reports);  
    println!("{} total reports", total_reports);

    // Pt. 2

    for report in failing_reports {
        // do something 
    }
    
}
