use std::fmt::Debug;
use std::str::FromStr;

pub fn as_numbers<T>(input: &str) -> Vec<T>
where T: std::str::FromStr, <T as FromStr>::Err: Debug {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<T>().unwrap())
        .collect()
}