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

fn extract_numbers(input: &str) -> (Vec<u32>, Vec<u32>) {
    let substring = &input[input.find(':').unwrap() + 1..];
    let mut parts = substring.splitn(2, '|');
    
    if let (Some(second_part), Some(third_part)) = (parts.next(), parts.next()) {
        let second_numbers: Vec<u32> = second_part.trim().split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let third_numbers: Vec<u32> = third_part.trim().split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        (second_numbers, third_numbers)
    } else {
        (Vec::new(), Vec::new())
    }
}

fn count_matching_numbers(vec1: &Vec<u32>, vec2: &Vec<u32>) -> usize {
    vec2.iter().filter(|&num| vec1.contains(num)).count()
}


fn main() {
    let file_path = "input.txt";

    let lines = read_file_lines(file_path).unwrap();
    let mut sum = 0;
    for line in &lines {
        let numbers = extract_numbers(&line);
        let count = count_matching_numbers(&numbers.1, &numbers.0);
        if count > 0 {
          sum += (2 as u32).pow((count - 1) as u32);
        }
    }
    println!("{}", sum);
}