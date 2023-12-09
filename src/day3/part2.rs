use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

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

fn find_number(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut digits: VecDeque<u32> = VecDeque::new();
    digits.push_front(grid[y][x].to_digit(10).unwrap());

    
    // search right side
    let mut x_current = x + 1;
    while x_current < grid[0].len() && grid[y][x_current].is_digit(10) {
        digits.push_back(grid[y][x_current].to_digit(10).unwrap());
        x_current += 1;
    }
    // search left side
    let mut x_current = x - 1;
    while grid[y][x_current].is_digit(10) {
        digits.push_front(grid[y][x_current].to_digit(10).unwrap());
        if x_current == 0 {
        break;
      }
      x_current -= 1;
    }
    let mut result = 0;
    // println!("{:?}", digits);
    for &digit in &digits {
        result = result * 10 + digit;
    }
    // println!("{}", result);
    result
}

fn find_digit_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    // Get the dimensions of the grid
    let rows = grid.len();
    let cols = grid[0].len();

    // Define the neighboring coordinates relative to the current position
    let neighbors = [
                (-1, 0),
        (0, -1),           (0, 1),
                (1, 0),  
    ];

    let mut numbers: Vec<u32> = Vec::new();

    for &(dx, dy) in neighbors.iter() {
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;

        // Check if the new coordinates are within bounds
        if new_y < rows && new_x < cols {
            let neighbor_char = grid[new_y][new_x];
            if dy == 0 {
                if neighbor_char.is_digit(10) {
                    numbers.push(find_number(&grid, new_x, new_y));
                }
            } else {
                if neighbor_char.is_digit(10) {
                    numbers.push(find_number(&grid, new_x, new_y));
                } else {
                    if new_x < cols - 1 && grid[new_y][new_x+1].is_digit(10) {
                        numbers.push(find_number(&grid, new_x+1, new_y));
                    }
                    if new_x > 0 && grid[new_y][new_x-1].is_digit(10) {
                        numbers.push(find_number(&grid, new_x-1, new_y));
                    }
                }
            }
        }
    }

    numbers
}

fn find_stars(grid: &Vec<Vec<char>>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for (y_index, row) in grid.iter().enumerate() {
        // println!("y {}", y_index);
        for (x_index, &ch) in row.iter().enumerate() {
            if ch == '*' {
                let neighbors = find_digit_neighbors(&grid, x_index, y_index);
                // println!("{:?}", neighbors);
                if neighbors.len() == 2 {
                  numbers.push(neighbors[0]*neighbors[1]);
                }
            }
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
    let numbers = find_stars(&grid);
    let sum: u32 = numbers.iter().sum();
    println!("{}", sum);
}