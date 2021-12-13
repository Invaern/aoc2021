use crate::model::Line;

pub fn line_filter(_line: &Line) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use aoc2021::BoxError;
    use crate::parser;
    use crate::part1;

    use super::*;

    #[test]
    fn test_sample() -> Result<(), BoxError> {
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
        let lines = parser::parse(input)?;
        assert_eq!(part1::solve(&lines, line_filter), 12);
        Ok(())
    }
}