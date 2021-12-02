use aoc2021::{as_vec, get_input};

fn main() {
    let input = get_input("inputs/01.txt");
    let solution = solve(&input);
    println!("{}", solution)
}

fn solve(input: &str) -> u32 {
    let numbers = as_vec::<u32>(input);
    increases(numbers)
}

fn increases(numbers: Vec<u32>) -> u32 {
    let (incs, _) = numbers
        .into_iter()
        .fold((0u32, u32::MAX), |(inc, last), current| {
            let incs = if current > last { inc + 1 } else { inc };
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
        assert_eq!(solve(sample_input), 7);
    }
}
