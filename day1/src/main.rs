
fn main() -> anyhow::Result<()> {

    // include input as part of binary, parse to ints, return as result
    let input = include_str!("./input.txt")
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    const ELEMENTS: usize = 3;
    const WINDOW_SIZE: usize = ELEMENTS + 1;

    let result = input
        .windows(WINDOW_SIZE)
        .filter(|iter| {
            let lhs: i64 = iter[..ELEMENTS].iter().sum();
            let rhs: i64 = iter[(WINDOW_SIZE - ELEMENTS)..].iter().sum();
            rhs > lhs
        })
        .collect::<Vec<_>>();

    println!("{:?}", result.len());
    
    Ok(())
}
