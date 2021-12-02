use std::fmt::Debug;
use std::str::FromStr;
use std::{env, fs};

pub fn as_vec<T>(input: &str) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<T>().unwrap())
        .collect()
}

pub fn get_input(default_path: &str) -> String {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => default_path.to_owned()
    };
    fs::read_to_string(path).expect("Failed to load input")
}
