use std::fs::File;
use std::io::{self, BufRead};
use std::fmt;

fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over lines and collect them into a vector of strings
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_input(input: &str) -> Vec<Game> {
    let index = input.find(':').unwrap();
    let substring = &input[index + 1..];
    substring
        .split(';')
        .filter(|&game| !game.trim().is_empty())
        .map(|game| {
            let (red, green, blue) = game.split(',')
                .fold((0, 0, 0), |(r, g, b), elem| {
                    let parts: Vec<&str> = elem.trim().split_whitespace().collect();
                    let count = parts[0].parse::<u32>().unwrap_or(0);
                    match parts[1] {
                        "red" => (r + count, g, b),
                        "green" => (r, g + count, b),
                        "blue" => (r, g, b + count),
                        _ => (r, g, b),
                    }
                });
            Game { red, green, blue }
        })
        .collect()
}

fn sum_elements(games: &[Game]) -> (u32, u32, u32) {
    let (total_red, total_green, total_blue) = games.iter().fold((0, 0, 0), |acc, game| {
        (
            acc.0 + game.red,
            acc.1 + game.green,
            acc.2 + game.blue,
        )
    });

    (total_red, total_green, total_blue)
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Red: {}, Green: {}, Blue: {}", self.red, self.green, self.blue)
    }
}


fn main() {
    let file_path = "input.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let mut sum = 0;
            for (index, line) in lines.iter().enumerate() {
                let games = parse_input(&line);
                let mut valid = true;
                for game in games {
                  if game.red > 12 || game.green > 13 || game.blue > 14 {
                    valid = false;
                    break;
                  }
                }
                if valid {
                    sum += index + 1;
                }
            }
            println!("Sum: {}", sum);
            
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}