use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1, u16 as p_u16};
use nom::combinator::eof;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{separated_pair, terminated};

use crate::model::{Line, Point};

fn point(s: &str) -> IResult<&str, Point> {
    let (s, (x, y)) = separated_pair(p_u16, tag(","), p_u16)(s)?;
    Ok((s, Point { x, y }))
}

fn line(s: &str) -> IResult<&str, Line> {
    let (s, (from, to)) = separated_pair(point, tag(" -> "), point)(s)?;
    Ok((s, Line { from, to }))
}

fn puzzle_input(s: &str) -> IResult<&str, Vec<Line>> {
    let line_end = alt((multispace1, eof));

    let (s, _) = multispace0(s)?;
    let (s, lines) = many0(terminated(line, line_end))(s)?;
    Ok((s, lines))
}

pub fn parse(s: &str) -> Result<Vec<Line>, String> {
    match puzzle_input(s) {
        Ok(("", lines)) => Ok(lines),
        Ok((leftover, _)) => Err(format!("Leftover input: {}", leftover)),
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::puzzle_input;

    #[test]
    fn test_puzzle_input() -> Result<(), aoc2021::BoxError> {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
        let (s, lines) = puzzle_input(input)?;
        assert_eq!(s, "");
        assert_eq!(lines.len(), 10);
        Ok(())
    }
}