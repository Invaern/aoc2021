use std::collections::VecDeque;
use crate::board::Board;

pub fn solve(numbers: Vec<u8>, boards: Vec<Board>) -> u32 {
    if let Some(bingo_board) = find_board(numbers, boards) {
        sum_unchecked(&bingo_board.board) * (bingo_board.last_number as u32)
    } else {
        0
    }
}

fn sum_unchecked(board: &Board) -> u32 {
    board.all_numbers()
        .filter(|bn| !bn.checked)
        .map(|bn| bn.value as u32)
        .sum()
}

fn find_board(numbers: Vec<u8>, boards: Vec<Board>) -> Option<BingoBoard> {
    let mut result = None;
    let mut remaining_numbers: VecDeque<u8> = VecDeque::from(numbers);
    let mut remaining_boards = boards;

    while let Some(next_number) = remaining_numbers.pop_front() {
        let (bingo_board, other) = last_bingo_board(next_number, remaining_boards);
        remaining_boards = other;
        result = choose_last_bingo(result, bingo_board);
    }


    result
}

fn choose_last_bingo(b1: Option<BingoBoard>, b2: Option<BingoBoard>) -> Option<BingoBoard> {
    match (b1, b2) {
        (_, Some(b2)) => Some(b2),
        (b1, None) => b1
    }
}

fn last_bingo_board(number: u8, mut boards: Vec<Board>) -> (Option<BingoBoard>, Vec<Board>) {
    for board in boards.iter_mut() {
        board.select_number(number);
    }
    let (mut bingo_boards, other): (Vec<Board>, Vec<Board>) = boards.into_iter().partition(|b| b.bingo());

    let last_bingo_board = bingo_boards.pop()
        .map(|b| { BingoBoard { last_number: number, board: b } });
    (last_bingo_board, other)
}

#[derive(Debug)]
struct BingoBoard {
    last_number: u8,
    board: Board,
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::part2::{find_board, solve};

    #[test]
    fn solve_sample() -> Result<(), String> {
        let sample = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        let (numbers, boards) = parse(sample)?;
        assert_eq!(solve(numbers, boards), 1924);
        Ok(())
    }
}
