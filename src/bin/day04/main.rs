mod board;
mod parser;
mod part1;
mod part2;

fn main() -> Result<(), String> {
    let input = aoc2021::get_input("inputs/04.txt");
    let (numbers, boards) = parser::parse(&input)?;
    let solution1 = part1::solve(&numbers, boards.clone());
    println!("Part 1: {}", solution1);
    let solution2 = part2::solve(numbers, boards.clone());
    println!("Part 2: {}", solution2);
    Ok(())
}