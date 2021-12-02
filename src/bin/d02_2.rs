use std::fs;
use std::str::FromStr;
use crate::Command::{Down, Forward, Up};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}



impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd_name, value)) = s.split_once(" ") {
            let parsed_val = value.parse::<u32>().map_err(|err| format!("Failed to convert to u32: {}", err))?;
            match cmd_name {
                "forward" => Ok(Forward(parsed_val)),
                "down" => Ok(Down(parsed_val)),
                "up" => Ok(Up(parsed_val)),
                other => Err(format!("Unknown command: {}", other))
            }
        } else {
            Err(format!("Invalid input: {}", s))
        }
    }
}

struct Submarine {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl Submarine {
    fn new(depth: u32, horizontal: u32, aim: u32) -> Self {
        Submarine{ depth, horizontal, aim }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/02.txt").expect("Failed to load input");
    let solution = solve(&input);
    println!("{}", solution)

}

fn solve(input: &str) -> u32 {
    let commands = aoc2021::as_vec(input);
    let submarine = run_commands(commands);
    submarine.depth * submarine.horizontal
}

fn run_commands(commands: Vec<Command>) -> Submarine {
    let mut submarine = Submarine::new(0, 0, 0);
    for cmd in commands {
        match cmd {
            Forward(val) => {
                submarine.horizontal += val;
                submarine.depth += submarine.aim * val;
            },
            Up(val) => submarine.aim -= val,
            Down(val) => submarine.aim += val
        };
    }

    submarine
}
#[cfg(test)]
mod tests {
    use crate::{Command, solve};

    #[test]
    fn parse_forward() {
        let line = "forward 10";
        let expected = Command::Forward(10);
        let result = line.parse::<Command>();
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn parse_down() {
        let line = "down 15";
        let expected = Command::Down(15);
        let result = line.parse::<Command>();
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn parse_up() {
        let line = "up 20";
        let expected = Command::Up(20);
        let result = line.parse::<Command>();
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn solve_sample() {
        let input = r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "#;
        assert_eq!(900, solve(input))
    }

}