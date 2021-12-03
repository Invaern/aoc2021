use std::str::FromStr;
use aoc2021::{as_vec, get_input};

#[derive(Debug, Eq, PartialEq)]
struct BinaryNumber {
    str: String,
}

impl BinaryNumber {
    fn decimal(&self) -> u32 {
        u32::from_str_radix(&self.str, 2).expect(&format!("BinaryNumber {:?} is not binary?!", self))
    }
}

impl FromStr for BinaryNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_str = s.chars().all(|c| c == '1' || c == '0');
        if valid_str {
            Ok(BinaryNumber { str: s.to_owned() })
        } else {
            Err(format!("Invalid binary string: {}", s))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BitFrequencies {
    frequencies: Vec<i32>,
}

impl BitFrequencies {
    fn gamma(&self) -> BinaryNumber {
        let str = self.frequencies.iter()
            .map(|fq| if *fq > 0 { '1' } else { '0' })
            .collect();
        BinaryNumber { str }
    }

    fn epsilon(&self) -> BinaryNumber {
        let str = self.frequencies.iter()
            .map(|fq| if *fq > 0 { '0' } else { '1' })
            .collect();
        BinaryNumber { str }
    }
}

impl From<BinaryNumber> for BitFrequencies {
    fn from(binary: BinaryNumber) -> Self {
        let frequencies = binary.str.chars()
            .map(|bit| if bit == '1' { 1 } else { -1 })
            .collect();
        BitFrequencies { frequencies }
    }
}


fn merge(first: BitFrequencies, second: BitFrequencies) -> BitFrequencies {
    if first.frequencies.len() != second.frequencies.len() {
        panic!("BitFrequencies {:?} and {:?} can't be merged, something went horribly wrong!", first, second);
    } else {
        let frequencies = first.frequencies.iter().zip(second.frequencies.iter())
            .map(|(first, second)| first + second)
            .collect();
        BitFrequencies { frequencies }
    }
}

fn main() {
    let input = get_input("inputs/03.txt");
    let solution = solve(&input);
    println!("{}", solution)
}

fn solve(input: &str) -> u32 {
    let binaries = as_vec(input);
    let frequencies = get_frequencies(binaries);
    let gamma = frequencies.gamma().decimal();
    let epsilon = frequencies.epsilon().decimal();

    gamma * epsilon
}

fn get_frequencies(binaries: Vec<BinaryNumber>) -> BitFrequencies {
    let frequencies = binaries.into_iter()
        .map(|binary| binary.into())
        .reduce(|first, second| merge(first, second))
        .expect("Input can't be empty");
    frequencies
}


#[cfg(test)]
mod tests {
    use crate::{BinaryNumber, get_frequencies, solve};

    const SAMPLE_INPUT: &str = r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "#;

    #[test]
    fn solve_sample() {
        assert_eq!(198, solve(SAMPLE_INPUT))
    }

    #[test]
    fn parse_binary() {
        let input = "00100";
        let expected = BinaryNumber { str: input.to_owned() };
        let result = input.parse();
        assert_eq!(Ok(expected), result)
    }

    #[test]
    fn to_decimal() {
        let binary = BinaryNumber { str: "00100".to_owned() };
        assert_eq!(4, binary.decimal())
    }

    #[test]
    fn epsilon() {
        let binaries = aoc2021::as_vec(SAMPLE_INPUT);
        let frequencies = get_frequencies(binaries);
        let expected = BinaryNumber { str: "01001".to_owned() };
        assert_eq!(expected, frequencies.epsilon())
    }

    #[test]
    fn gamma() {
        let binaries = aoc2021::as_vec(SAMPLE_INPUT);
        let frequencies = get_frequencies(binaries);
        let expected = BinaryNumber { str: "10110".to_owned() };
        assert_eq!(expected, frequencies.gamma())
    }
}