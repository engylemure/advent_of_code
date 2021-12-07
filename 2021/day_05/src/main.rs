use hydrotermal_venture::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod hydrotermal_venture {
    use std::{
        collections::{hash_map::Entry, HashMap},
        str::FromStr,
    };
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Point {
        x: u32,
        y: u32,
    }

    #[derive(Debug)]
    pub struct Line {
        start: Point,
        end: Point,
    }
    impl FromStr for Point {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split(',');
            let x = s.next().map(|s| s.parse().map_err(|_| ())).ok_or(())??;
            let y = s.next().map(|s| s.parse().map_err(|_| ())).ok_or(())??;
            if s.next().is_some() {
                return Err(());
            }
            Ok(Point { x, y })
        }
    }

    impl FromStr for Line {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split("->");
            let start = s.next().map(|s| s.trim().parse()).ok_or(())??;
            let end = s.next().map(|s| s.trim().parse()).ok_or(())??;
            if s.next().is_some() {
                return Err(());
            }
            Ok(Line { start, end })
        }
    }

    impl Line {
        fn points<T: FromIterator<Point>>(&self) -> T {
            let end = self.end.clone();
            let mut ended = self.start == self.end;
            let mut point = self.start.clone();
            let mut x_diff = self.end.x as i32 - self.start.x as i32;
            let mut y_diff = self.end.y as i32 - self.start.y as i32;
            let should_sub_x = x_diff > 0;
            let should_sub_y = y_diff > 0;
            std::iter::from_fn(move || {
                if ended {
                    return None;
                }
                let actual_point = point.clone();
                let x = if x_diff != 0 {
                    if should_sub_x {
                        x_diff -= 1;
                        actual_point.x + 1
                    } else {
                        x_diff += 1;
                        actual_point.x - 1
                    }
                } else {
                    actual_point.x
                };
                let y = if y_diff != 0 {
                    if should_sub_y {
                        y_diff -= 1;
                        actual_point.y + 1
                    } else {
                        y_diff += 1;
                        actual_point.y - 1
                    }
                } else {
                    actual_point.y
                };
                point = Point { x, y };
                if actual_point == end {
                    ended = true;
                }
                Some(actual_point)
            })
            .collect()
        }
    }

    pub fn solution_01(input: Vec<Line>) -> usize {
        let line_points: Vec<_> = input
            .iter()
            .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
            .map(Line::points::<Vec<_>>)
            .collect();
        let mut points = HashMap::new();
        let mut amount_of_overlapped_points = 0;
        for line in line_points.iter() {
            for point in line {
                match points.entry(point) {
                    Entry::Occupied(mut o) => {
                        if *o.get() == 1 {
                            amount_of_overlapped_points += 1;
                        }
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }
            }
        }
        amount_of_overlapped_points
    }

    pub fn solution_02(input: Vec<Line>) -> usize {
        let line_points: Vec<_> = input.iter().map(Line::points::<Vec<_>>).collect();
        let mut points = HashMap::new();
        let mut amount_of_overlapped_points = 0;
        for line in line_points.iter() {
            for point in line {
                match points.entry(point) {
                    Entry::Occupied(mut o) => {
                        if *o.get() == 1 {
                            amount_of_overlapped_points += 1;
                        }
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }
            }
        }
        amount_of_overlapped_points
    }

    pub fn transform_input(input: &str) -> Vec<Line> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::hydrotermal_venture::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 5);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 12);
    }
}
