extern crate nom;

use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, u8 as p_u8};
use nom::combinator::{eof, map};
use nom::IResult;
use nom::multi::{many0, many_m_n, separated_list1};

use crate::board::Board;

fn selected_numbers(s: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), p_u8)(s)
}

fn board(s: &str) -> IResult<&str, Board> {
    let numbers_p = many_m_n(25, 25, number);
    map(numbers_p, |numbers| Board::new(numbers))(s)
}

fn number(s: &str) -> IResult<&str, u8> {
    let (s, _) = multispace0(s)?;
    p_u8(s)
}

// fn boards(s: &str) -> IResult<&str, Vec<Board>> {
//     many0(board)(s)
// }

fn puzzle_input(s: &str) -> IResult<&str, (Vec<u8>, Vec<Board>)> {
    let (s, _) = multispace0(s)?;
    let (s, numbers) = selected_numbers(s)?;
    let (s, boards) = many0(board)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = eof(s)?;
    Ok((s, (numbers, boards)))
}

pub fn parse(s: &str) -> Result<(Vec<u8>, Vec<Board>), String> {
    match puzzle_input(s) {
        Ok(("", (numbers, boards))) => Ok((numbers, boards)),
        Err(_e) => Err("Parsing failed".to_owned()),
        Ok((leftover, _)) => Err(format!("Leftover input: {}", leftover))
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::parser::{board, puzzle_input, selected_numbers};

    #[test]
    fn test_selected_numbers() {
        let input = "1,2,3,4,5";
        let expected = vec![1u8, 2, 3, 4, 5];
        assert_eq!(selected_numbers(input), Ok(("", expected)))
    }

    #[test]
    fn test_board() {
        let input =
            r#"1  2  3  4  5
               6  7  8  9 10
              11 12 13 14 15
              16 17 18 19 20
              21 22 23 24 25 r"#;

        let expected = Board::new((1..=25).collect());
        let res = board(input);
        assert_eq!(res, Ok((" r", expected)));
    }

    // #[test]
    // fn test_boards() {
    //     let input =
    //         r#"22 13 17 11  0
    //          8  2 23  4 24
    //         21  9 14 16  7
    //          6 10  3 18  5
    //          1 12 20 15 19
    //
    //          3 15  0  2 22
    //          9 18 13 17  5
    //         19  8  7 25 23
    //         20 11 10 24  4
    //         14 21 16 12  6
    //
    //         14 21 17 24  4
    //         10 16 15  9 19
    //         18  8 23 26 20
    //         22 11 13  6  5
    //          2  0 12  3  7"#;
    //     let parsed = boards(input);
    //     match parsed {
    //         Ok(("", boards)) => assert_eq!(boards.len(), 3),
    //         Ok((input, _)) => assert_eq!(input, ""),
    //         Err(e) => {
    //             dbg!(e);
    //             assert_eq!(false, true);
    //         }
    //     }
    // }

    #[test]
    fn sample_input() {
        let input =
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
        let parsed = puzzle_input(input);
        match parsed {
            Ok(("", (numbers, boards))) => {
                assert_eq!(numbers.len(), 27);
                assert_eq!(boards.len(), 3);
            }
            Err(e) => {
                dbg!(e);
                assert_eq!(true, false)
            }
            _ => assert_eq!(true, false)
        }
    }
}
