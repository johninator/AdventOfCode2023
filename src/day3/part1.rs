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

fn strings_to_2d_array(strings: &Vec<String>) -> Vec<Vec<char>> {
    let height = strings.len();

    let mut result: Vec<Vec<char>> = Vec::with_capacity(height);

    for i in 0..height {
        let row: Vec<char> = strings[i].chars().collect();
        result.push(row);
    }

    result
}

fn has_non_digit_neighbor(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // Get the dimensions of the grid
    let rows = grid.len();
    let cols = grid[0].len();

    // Define the neighboring coordinates relative to the current position
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0),  (1, 1),
    ];

    for &(dx, dy) in neighbors.iter() {
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;

        // Check if the new coordinates are within bounds
        if new_y < rows && new_x < cols {
            let neighbor_char = grid[new_y][new_x];
            if !neighbor_char.is_digit(10) && neighbor_char != '.' {
                return true; // Found a non-digit, non-dot neighbor
            }
        }
    }

    false // No non-digit, non-dot neighbors found
}

fn find_numbers(grid: &Vec<Vec<char>>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let mut current_numbers: Vec<u32> = Vec::new();
    for (y_index, row) in grid.iter().enumerate() {
        // println!("y {}", y_index);
        for (x_index, &ch) in row.iter().enumerate() {
            // println!("x {}", x_index);
            if ch.is_digit(10) {
                current_numbers.push(ch.to_digit(10).unwrap());
            } else if !current_numbers.is_empty() {
                let mut valid = false;
                let mut value = 0;
                for (index, num) in current_numbers.iter().enumerate() {
                    value = value*10 + num;
                    // println!("i {}", index);
                    if has_non_digit_neighbor(grid, x_index - index - 1, y_index) {
                        valid = true;
                    }
                }
                if valid {
                  numbers.push(value);
                }
                current_numbers.clear();
            }
        }
        if !current_numbers.is_empty() {
            let mut valid = false;
            let mut value = 0;
            for (index, num) in current_numbers.iter().enumerate() {
                value = value*10 + num;
                // println!("i {}", index);
                if has_non_digit_neighbor(grid, row.len() - index - 1, y_index) {
                    valid = true;
                }
            }
            if valid {
              numbers.push(value);
            }
            current_numbers.clear();
        }
    }
    numbers
}

fn main() {
    let file_path = "input.txt";

    let lines = read_file_lines(file_path).unwrap();
    let grid = strings_to_2d_array(&lines);
    for line in &grid {
    println!("{:?}", line);
    }
    let numbers = find_numbers(&grid);
    let sum: u32 = numbers.iter().sum();
    println!("{}", sum);
}