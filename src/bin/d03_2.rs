use std::str::FromStr;
use aoc2021::{as_vec, get_input};

#[derive(Debug, Eq, PartialEq, Clone)]
struct BinaryNumber {
    str: String,
}

impl BinaryNumber {
    fn decimal(&self) -> u32 {
        u32::from_str_radix(&self.str, 2).expect(&format!("BinaryNumber {:?} is not binary?!", self))
    }

    fn bit_match(&self, bit_idx: usize, bit: char) -> bool {
        if let Some(c) = self.str.chars().nth(bit_idx) {
            c == bit
        } else {
            false
        }
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
    fn oxygen_bit(&self, bit_idx: usize) -> Option<char> {
        if let Some(bit) = self.frequencies.iter().nth(bit_idx) {
            let bit_char = if *bit >= 0 { '1' } else { '0' };
            Some(bit_char)
        } else {
            None
        }
    }

    fn scrubber_bit(&self, bit_idx: usize) -> Option<char> {
        if let Some(bit) = self.frequencies.iter().nth(bit_idx) {
            let bit_char = if *bit >= 0 { '0' } else { '1' };
            Some(bit_char)
        } else {
            None
        }
    }
}

fn as_frequency(binary: &BinaryNumber) -> BitFrequencies {
    let frequencies = binary.str.chars()
        .map(|bit| if bit == '1' { 1 } else { -1 })
        .collect();
    BitFrequencies { frequencies }
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
    let scrubber_rating_bin = scrubber_rating(0, &binaries);
    let oxygen_rating_bin = oxygen_rating(0, &binaries);

    scrubber_rating_bin.decimal() * oxygen_rating_bin.decimal()
}

fn get_frequencies(binaries: &[BinaryNumber]) -> BitFrequencies {
    let frequencies = binaries.into_iter()
        .map(|binary| as_frequency(binary))
        .reduce(|first, second| merge(first, second))
        .expect("Input can't be empty");
    frequencies
}

fn oxygen_rating(current_bit: usize, binaries: &[BinaryNumber]) -> BinaryNumber {
    if binaries.len() == 1 {
        return binaries[0].clone();
    }
    let frequencies = get_frequencies(binaries);
    if let Some(oxygen_bit) = frequencies.oxygen_bit(current_bit) {
        let matching_binaries: Vec<BinaryNumber> = binaries.iter()
            .filter(|b| b.bit_match(current_bit, oxygen_bit))
            .map(|b| b.clone())
            .collect();
        oxygen_rating(current_bit + 1, &matching_binaries)
    } else {
        panic!("Unable to find oxygen rating. Remaining binaries: {:?}", binaries)
    }
}

fn scrubber_rating(current_bit: usize, binaries: &[BinaryNumber]) -> BinaryNumber {
    if binaries.len() == 1 {
        return binaries[0].clone();
    }
    let frequencies = get_frequencies(binaries);
    if let Some(oxygen_bit) = frequencies.scrubber_bit(current_bit) {
        let matching_binaries: Vec<BinaryNumber> = binaries.iter()
            .filter(|b| b.bit_match(current_bit, oxygen_bit))
            .map(|b| b.clone())
            .collect();
        scrubber_rating(current_bit + 1, &matching_binaries)
    } else {
        panic!("Unable to find oxygen rating. Remaining binaries: {:?}", binaries)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryNumber, oxygen_rating, scrubber_rating, solve};

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
        assert_eq!(230, solve(SAMPLE_INPUT))
    }


    #[test]
    fn test_oxygen_rating() {
        let binaries = aoc2021::as_vec(SAMPLE_INPUT);
        let expected = BinaryNumber { str: "10111".to_owned() };
        let result = oxygen_rating(0, &binaries);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_scrubber_rating() {
        let binaries = aoc2021::as_vec(SAMPLE_INPUT);
        let expected = BinaryNumber { str: "01010".to_owned() };
        let result = scrubber_rating(0, &binaries);
        assert_eq!(expected, result)
    }
}