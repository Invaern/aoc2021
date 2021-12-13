use aoc2021::{BoxError, get_input};
use crate::parser::parse;

mod model;
mod parser;
mod part1;
mod part2;

fn main() -> Result<(), BoxError> {
    let input = get_input("inputs/05.txt");
    let lines = parse(&input)?;

    let solution1 = part1::solve(&lines, part1::line_filter);
    println!("Part 1: {}", solution1);
    let solution2 = part1::solve(&lines, part2::line_filter);
    println!("Part 2: {}", solution2);

    Ok(())
}