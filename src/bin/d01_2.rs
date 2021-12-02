use std::fs;
use aoc2021;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").expect("Failed to load input");
    let solution = solve(&input);
    println!("{}", solution)

}

fn solve(input: &str) -> u32 {
    let numbers = aoc2021::as_vec(input);
    let windows = window_sums(numbers);
    increases(windows)
}

fn window_sums(numbers: Vec<u32>) -> Vec<u32> {
    numbers
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect()
}

fn increases(numbers: Vec<u32>) -> u32 {
    let (incs, _) = numbers.into_iter()
        .fold((0u32, u32::MAX),
              |(inc, last),  current| {
                  let incs = if current > last { inc + 1} else {inc};
                  (incs, current)
              });
    incs
}


#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn solve_sample() {
        let sample_input = r#"
                199
                200
                208
                210
                200
                207
                240
                269
                260
                263
        "#;
        assert_eq!(solve(sample_input), 5);
    }
}