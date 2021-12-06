use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct BoardNumber {
    value: u8,
    checked: bool,
}

pub struct Board {
    numbers_set: HashSet<u8>,
    numbers: Vec<BoardNumber>,
}

impl Board {
    pub fn new(numbers: Vec<u8>) -> Self {
        let board_numbers = numbers.iter()
            .map(|n| BoardNumber { value: *n, checked: false })
            .collect();
        let set = HashSet::from_iter(numbers.into_iter());
        Board {
            numbers: board_numbers,
            numbers_set: set,
        }
    }

    pub fn rows(&self) -> RowIterator {
        RowIterator { row: 0, numbers: &self.numbers[..] }
    }

    pub fn columns(&self) -> ColumnIterator {
        ColumnIterator { column: 0, numbers: &self.numbers[..] }
    }
}


#[derive(Debug)]
struct Line {
    numbers: Vec<BoardNumber>,
}

impl<'a> IntoIterator for Line {
    type Item = BoardNumber;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}

struct ColumnIterator<'a> {
    column: usize,
    numbers: &'a [BoardNumber],
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column < 5 {
            let columns = (self.column..25).step_by(5).into_iter()
                .map(|n| self.numbers[n])
                .collect();
            self.column += 1;

            Some(Line { numbers: columns })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct RowIterator<'a> {
    row: usize,
    numbers: &'a [BoardNumber],
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < 5 {
            let row_start = self.row * 5;
            let row_end = row_start + 5;
            self.row += 1;
            let line = Line { numbers: self.numbers[row_start..row_end].to_vec() };
            Some(line)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::board::{Board, BoardNumber};

    #[test]
    fn test_rows() {
        let numbers = (0..25).into_iter().collect();
        let board = Board::new(numbers);
        let board_rows: Vec<Vec<BoardNumber>> = board.rows()
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();

        let expected_rows: Vec<Vec<BoardNumber>> = vec![
            (0..5).into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            (5..10).into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            (10..15).into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            (15..20).into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            (20..25).into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
        ];
        for (row, expected_row) in board_rows.into_iter().zip(expected_rows.into_iter()) {
            for (row_value, expected_value) in row.into_iter().zip(expected_row.into_iter()) {
                assert_eq!(row_value, expected_value)
            }
        }
    }

    #[test]
    fn test_columns() {
        let numbers = (0..25).into_iter().collect();
        let board = Board::new(numbers);
        let board_columns: Vec<Vec<BoardNumber>> = board.columns()
            .into_iter()
            .map(|col| col.into_iter().collect())
            .collect();

        let expected_columns: Vec<Vec<BoardNumber>> = vec![
            vec![0, 5, 10, 15, 20].into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            vec![1, 6, 11, 16, 21].into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            vec![2, 7, 12, 17, 22].into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            vec![3, 8, 13, 18, 23].into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
            vec![4, 9, 14, 19, 24].into_iter().map(|n| BoardNumber { value: n, checked: false }).collect(),
        ];
        for (col, expected_col) in board_columns.into_iter().zip(expected_columns.into_iter()) {
            for (col_value, expected_value) in col.into_iter().zip(expected_col.into_iter()) {
                assert_eq!(col_value, expected_value)
            }
        }
    }
}