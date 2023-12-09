use std::fs::File;
use std::io::{self, BufRead};

fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over lines and collect them into a vector of strings
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

fn find_digits(line: &str) -> Option<u32> {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.is_empty() {
        None
    } else {
        Some(digits[0] * 10 + *digits.last().unwrap())
    }
}

fn main() {
    let file_path = "input.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let mut sum = 0;
            for line in lines {
                if let Some(digit) = find_digits(&line) {
                    sum += digit;
                }
            }
            println!("Sum: {}", sum);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}