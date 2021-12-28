use core::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, Copy)]
enum Direction {
    UP,
    DOWN,
    FORWARD,
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    value: i64
}

impl FromStr for Instruction {

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        dbg!(line);
        // 1. get tokens by splitting on whitespace
        //  - token 1 must be equal to a direction
        //  - token 2 must be an int
        //  - there must not be more tokens
        // 2. construct our Instruction
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens.len() {
            // <direction: &str> <value: usize>
            2 => {
                let direction = match tokens[0] {
                    "up" => Direction::UP,
                    "down" => Direction::DOWN,
                    "forward" => Direction::FORWARD,
                    error => panic!("unsupported direction: {:?}", error)
                };

                let value = match str::parse::<i64>(tokens[1]) {
                    Result::Ok(value) => value,
                    Result::Err(err) => panic!("unsupported value: {:?}", err),
                };

                dbg!(direction);
                Ok(Instruction {
                    direction,
                    value,
                })
            },
            error => { panic!("encountered too many tokens: {:?}", error) },
        }
    }

    type Err = ParseInstructionError;
}

#[derive(Debug)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for ParseInstructionError {}

#[derive(Debug, Clone, Copy)]
struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64
}

impl Position {
    fn follow(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction { direction: Direction::DOWN, value } => self.aim += value,
            Instruction { direction: Direction::UP, value } => self.aim -= value,
            Instruction { direction: Direction::FORWARD, value } => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
        }
    }
}

impl Default for Position {
    fn default() -> Self { 
        Position { 
            horizontal: 0, 
            depth: 0,
            aim: 0
        }
    }
}


fn main() -> anyhow::Result<()> {
    let mut position = Position::default();
    
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse::<Instruction>)
        .collect::<Result<Vec<_>, _>>()?;

    input.iter().for_each(|instr| position.follow(instr));



    println!("{:?}", position.horizontal * position.depth);
    Ok(())
}
