use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on file reading error
        .lines() // split stringer into iterate or lines
        .map(String::from) // Convert into String
        .collect() // place lines into vector
    }


fn main() {
    let file_path = "input.txt";
    println!("Filename: {}", file_path);

    let input_lines: Vec<String> = read_lines(file_path);
    let mut calibration_vals: Vec<i32> = Vec::new();

    for line in input_lines.into_iter() {
        let mut first_num: Option<i32> = None;
        let mut last_num: Option<i32> = None;

        // forwards
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                first_num = Some(digit as i32);
                break;
            }
        }

        // backwards
        for c in line.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                last_num = Some(digit as i32);
                break;
            }
        }

        // combine first_num and last_num into a single number
        if let (Some(first), Some(last)) = (first_num, last_num) {
            let combined_num: i32 = first * 10 + last;
            calibration_vals.push(combined_num);
        }

    }

    let res: i32 = calibration_vals.iter().sum();

    println!("The sum of all of the calibration values is {}", res);
}
