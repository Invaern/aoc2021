use std::collections::HashMap;
use std::ops::{Div, Rem};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BoardNumber {
    pub value: u8,
    pub checked: bool,
    row: u8,
    col: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    numbers: HashMap<u8, BoardNumber>,
}

struct Bingo {
    rows: [u8; 5],
    cols: [u8; 5],
}

impl Bingo {
    fn new() -> Self {
        Bingo { rows: [0; 5], cols: [0; 5] }
    }

    fn with_number(&mut self, number: &BoardNumber) {
        if number.checked {
            self.rows[number.row as usize] += 1;
            self.cols[number.col as usize] += 1;
        }
    }

    fn bingo(&self) -> bool {
        self.rows.iter().chain(self.cols.iter())
            .any(|v| *v == 5)
    }
}


impl Board {
    pub fn new(numbers: Vec<u8>) -> Self {
        let numbers_map: HashMap<u8, BoardNumber> = numbers.into_iter().enumerate()
            .map(|(idx, val)| (val, create_board_number(idx, val)))
            .collect();
        Board { numbers: numbers_map }
    }

    pub fn select_number(&mut self, value: u8) {
        if let Some(board_number) = self.numbers.get_mut(&value) {
            board_number.checked = true;
        }
    }

    pub fn bingo(&self) -> bool {
        let mut bingo = Bingo::new();
        self.numbers.values()
            .filter(|bn| bn.checked)
            .for_each(|bn| bingo.with_number(bn));
        bingo.bingo()
    }

    pub fn all_numbers(&self) -> impl Iterator<Item=&BoardNumber> {
        self.numbers.values()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("=== BOARD ===");
        for row in 0..5 {
            let numbers: Vec<&BoardNumber> = self.numbers.values()
                .filter(|n| n.row == row)
                .collect();
            for number in numbers {
                let checked = if number.checked { "*" } else { "" };
                print!("{}{} ", checked, number.value)
            }
            print!("\n")
        }
    }
}

fn create_board_number(index: usize, value: u8) -> BoardNumber {
    let (row, col) = idx_to_position(index);
    BoardNumber {
        value,
        checked: false,
        row,
        col,
    }
}

fn idx_to_position(index: usize) -> (u8, u8) {
    let row = index.rem(5) as u8;
    let col = index.div(5) as u8;
    (row, col)
}


#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn test_bingo() {
        let mut board = Board::new((0..25).collect());
        assert_eq!(board.bingo(), false);
        for i in 0..5 {
            board.select_number(i);
        }
        assert_eq!(board.bingo(), true)
    }
}