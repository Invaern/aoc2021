use std::collections::HashMap;

use crate::model::{Line, LineType, Point};


pub fn line_filter(line: &Line) -> bool {
    use LineType::*;
    line.line_type() == Horizontal || line.line_type() == Vertical
}

pub fn solve(lines: &[Line], filter: impl Fn(&Line) -> bool) -> u32 {
    let mut points_map: HashMap<Point, u16> = HashMap::new();

    let filtered_lines = lines.iter()
        .filter(|line| filter(line));
    for line in filtered_lines {
        let points = line.line_points();
        for point in points {
            if let Some(v) = points_map.get_mut(&point) {
                *v += 1;
            } else {
                points_map.insert(point, 1);
            }
        }
    }

    let intersections = points_map.into_values()
        .filter(|v| *v > 1)
        .count();
    intersections as u32
}

#[cfg(test)]
mod tests {
    use aoc2021::BoxError;

    use crate::parser;
    use crate::part1::{line_filter, solve};

    #[test]
    fn solve_sample() -> Result<(), BoxError> {
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
        assert_eq!(solve(&lines, line_filter), 5);
        Ok(())
    }
}