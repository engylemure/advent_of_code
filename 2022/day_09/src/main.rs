use rope_bridge::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod rope_bridge {
    use std::collections::HashSet;
    use std::str::FromStr;
    type Input = Vec<Command>;
    type SolutionOne = usize;
    type SolutionTwo = usize;

    #[derive(Debug)]
    pub struct Command {
        direction: Direction,
        amount: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Direction {
        Right,
        Left,
        Up,
        Down,
    }

    #[derive(Debug)]
    pub struct Rope<const KNOT_SIZE: usize> {
        pub knots: [(i32, i32); KNOT_SIZE],
    }

    impl FromStr for Command {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s_iter = s.split_whitespace();
            Ok(Command {
                direction: match s_iter.next().ok_or(())?.trim() {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => return Err(()),
                },
                amount: s_iter.next().ok_or(())?.trim().parse().map_err(|_| ())?,
            })
        }
    }

    impl<const KNOT_SIZE: usize> Rope<KNOT_SIZE> {
        pub fn new() -> Self {
            Self {
                knots: [(0, 0); KNOT_SIZE],
            }
        }
        pub fn tail(&self) -> (i32, i32) {
            self.knots[KNOT_SIZE - 1]
        }
        pub fn move_to_direction(&mut self, direction: Direction) {
            match direction {
                Direction::Right => {
                    self.knots[0].0 += 1;
                }
                Direction::Left => {
                    self.knots[0].0 -= 1;
                }
                Direction::Up => {
                    self.knots[0].1 += 1;
                }
                Direction::Down => {
                    self.knots[0].1 -= 1;
                }
            };
            for head_idx in 0..(KNOT_SIZE - 1) {
                let x_diff = self.knots[head_idx].0 - self.knots[head_idx + 1].0;
                let y_diff = self.knots[head_idx].1 - self.knots[head_idx + 1].1;
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    self.knots[head_idx + 1].0 += x_diff.signum();
                    self.knots[head_idx + 1].1 += y_diff.signum();
                }
            }
        }
    }

    pub fn solution_01(input: Input) -> SolutionOne {
        let mut rope = Rope::<2>::new();
        let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
        for cmd in input {
            for _ in 0..(cmd.amount) {
                rope.move_to_direction(cmd.direction);
                visited_positions.insert(rope.tail());
            }
        }
        let max_x = visited_positions.iter().map(|(x, _)| *x).max().unwrap();
        let min_x = visited_positions.iter().map(|(x, _)| *x).min().unwrap();
        let max_y = visited_positions.iter().map(|(_, y)| *y).max().unwrap();
        let min_y = visited_positions.iter().map(|(_, y)| *y).min().unwrap();
        for y in max_y..min_y {
            for x in min_x..max_x {
                if visited_positions.contains(&(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }
        visited_positions.len()
    }

    pub fn solution_02(input: Input) -> SolutionTwo {
        let mut rope = Rope::<10>::new();
        let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
        for cmd in input {
            for _ in 0..(cmd.amount) {
                rope.move_to_direction(cmd.direction);
                visited_positions.insert(rope.tail());
            }
        }
        visited_positions.len()
    }

    pub fn transform_input(input: &str) -> Input {
        input.lines().filter_map(|l| l.parse().ok()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::rope_bridge::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 13);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 1);
    }

    #[test]
    fn test_03() {
        let input = include_str!("example_input2");
        assert_eq!(solution_02(transform_input(input)), 36);
    }

    #[test]
    fn test_04() {
        let mut rope = Rope::<3>::new();
        rope.knots[0] = (0, 2);
        rope.knots[1] = (0, 1);
        rope.move_to_direction(Direction::Up);
        assert_eq!(rope.knots[1], (0, 2));
        assert_eq!(rope.knots[2], (0, 1));
        rope.move_to_direction(Direction::Right);
        assert_eq!(rope.knots[1], (0, 2));
        assert_eq!(rope.knots[2], (0, 1));
        rope.move_to_direction(Direction::Right);
        assert_eq!(rope.knots[1], (1, 3));
        assert_eq!(rope.knots[2], (1, 2));
    }

    #[test]
    fn test_05() {
        let mut rope = Rope::<3>::new();
        rope.knots[0] = (0, 2);
        rope.knots[1] = (0, 1);
        rope.move_to_direction(Direction::Up);
        assert_eq!(rope.knots[1], (0, 2));
        assert_eq!(rope.knots[2], (0, 1));
        rope.move_to_direction(Direction::Right);
        assert_eq!(rope.knots[1], (0, 2));
        assert_eq!(rope.knots[2], (0, 1));
        rope.move_to_direction(Direction::Right);
        assert_eq!(rope.knots[1], (1, 3));
        assert_eq!(rope.knots[2], (1, 2));
    }
}
