use core::str::FromStr;
use std::fmt;

#[derive(Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    FORWARD,
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    value: usize
}

impl FromStr for Instruction {

    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
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


fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse::<Instruction>)
        .collect::<Result<Vec<_>, _>>()?;

    dbg!(input);

    Ok(())
}
