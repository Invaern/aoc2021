use std::cmp::{max, min};
use std::iter::Rev;
use std::ops::RangeInclusive;

use crate::model::PointsRange::{Forward, Reversed};

pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    #[allow(dead_code)]
    pub fn new(from: Point, to: Point) -> Self {
        Line { from, to }
    }

    pub fn line_type(&self) -> LineType {
        use LineType::*;

        if self.from.x == self.to.x {
            Vertical
        } else if self.from.y == self.to.y {
            Horizontal
        } else {
            Diagonal
        }
    }

    pub fn line_points(&self) -> Vec<Point> {
        use LineType::*;
        match self.line_type() {
            Horizontal => {
                let min_x = min(self.from.x, self.to.x);
                let max_x = max(self.from.x, self.to.x);
                (min_x..=max_x).into_iter()
                    .map(|x| Point::new(x, self.from.y))
                    .collect()
            }
            Vertical => {
                let min_y = min(self.from.y, self.to.y);
                let max_y = max(self.from.y, self.to.y);
                (min_y..=max_y).into_iter()
                    .map(|y| Point::new(self.from.x, y))
                    .collect()
            }
            Diagonal => {
                let xs: PointsRange = if self.from.x < self.to.x {
                    Forward(self.from.x..=self.to.x)
                } else {
                    Reversed((self.to.x..=self.from.x).rev())
                };
                let ys = if self.from.y < self.to.y {
                    Forward(self.from.y..=self.to.y)
                } else {
                    Reversed((self.to.y..=self.from.y).rev())
                };
                xs.zip(ys)
                    .map(|(x, y)| Point::new(x, y))
                    .collect()
            }
        }
    }
}

enum PointsRange {
    Forward(RangeInclusive<u16>),
    Reversed(Rev<RangeInclusive<u16>>),
}

impl Iterator for PointsRange {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Forward(range) => range.next(),
            Reversed(range) => range.next()
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

#[cfg(test)]
mod tests {
    use crate::model::{Line, LineType, Point};

    #[test]
    fn line_types() {
        let horizontal = Line::new(Point::new(0, 0), Point::new(10, 0));
        let vertical = Line::new(Point::new(0, 0), Point::new(0, 10));
        let diagonal = Line::new(Point::new(0, 0), Point::new(10, 10));
        assert_eq!(horizontal.line_type(), LineType::Horizontal);
        assert_eq!(vertical.line_type(), LineType::Vertical);
        assert_eq!(diagonal.line_type(), LineType::Diagonal);
    }

    #[test]
    fn test_line_points() {
        let line1 = Line::new(Point::new(0, 0), Point::new(2, 0));
        let line2 = Line::new(Point::new(2, 0), Point::new(0, 0));
        let points = vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)];
        assert_eq!(line1.line_points(), points);
        assert_eq!(line2.line_points(), points);
    }


    #[test]
    fn test_diagonal_points() {
        let line1 = Line::new(Point::new(0, 0), Point::new(2, 2));
        let line2 = Line::new(Point::new(2, 2), Point::new(0, 0));

        assert_eq!(line1.line_points(), vec![Point::new(0, 0),
                                             Point::new(1, 1), Point::new(2, 2)]);

        assert_eq!(line2.line_points(), vec![Point::new(2, 2),
                                             Point::new(1, 1), Point::new(0, 0)]);
    }
}