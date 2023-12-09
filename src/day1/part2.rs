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

fn find_matching_digit(input: &str) -> Option<u32> {
    let prefixes = [
        "one" ,
        "two" ,
        "three",
        "four",
        "five",
        "six" ,
        "seven",
        "eight",
        "nine"
    ];

    for (index, &prefix) in prefixes.iter().enumerate() {
        if input.starts_with(prefix) {
            let adjusted_index: u32 = index as u32 + 1;
            return Some(adjusted_index);
        }
    }
    None
}

fn find_digits(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            digits.push(character.to_digit(10).unwrap())
        } 
        else {
            if let Some(digit) = find_matching_digit(&line[index..]) {
                digits.push(digit);
            }

        }
    }
    digits
}

fn main() {
    let file_path = "input.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let mut sum = 0;
            for line in lines {
                println!("Line: {}", line);
                let digits = find_digits(&line);
                println!("Digits: {:?}", digits);
                sum += digits[0] * 10 + *digits.last().unwrap();
            }
            println!("Sum: {}", sum);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}