use rock_paper_scissors::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}


mod rock_paper_scissors {

    fn fold_round_res(acc: (u32, u32), val: (u32, u32)) -> (u32, u32) {
        (acc.0 + val.0, acc.1 + val.1)
    }

    pub fn solution_01(input: Vec<Round>) -> u32 {
        input
            .iter()
            .map(Round::result)
            .fold((0, 0), fold_round_res)
            .1
    }

    pub fn solution_02(input: Vec<Round>) -> u32 {
        input.iter()
            .map(Round::result_two)
            .fold((0, 0), fold_round_res)
            .1
    }

    #[derive(PartialEq)]
    pub enum Shape {
        Rock,
        Paper,
        Scissor,
    }

    pub struct Round(Shape, Shape);

    impl From<char> for Shape {
        fn from(c: char) -> Self {
            match c.to_ascii_lowercase() {
                'a' | 'x' => Self::Rock,
                'b' | 'y' => Self::Paper,
                'c' | 'z' => Self::Scissor,
                _ => panic!("'{}' is not a valid Shape", c),
            }
        }
    }

    impl From<&str> for Round {
        fn from(round: &str) -> Self {
            let mut options = round
                .trim()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(Shape::from);
            Self(options.next().unwrap(), options.next().unwrap())
        }
    }

    impl Shape {
        pub fn score(&self) -> u32 {
            match self {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissor => 3,
            }
        }
    }

    impl Round {
        pub fn result(&self) -> (u32, u32) {
            let mut round_outcome = match (&self.0, &self.1) {
                (Shape::Rock, Shape::Scissor) => (6, 0),
                (Shape::Paper, Shape::Rock) => (6, 0),
                (Shape::Scissor, Shape::Paper) => (6, 0),
                (Shape::Scissor, Shape::Rock) => (0, 6),
                (Shape::Rock, Shape::Paper) => (0, 6),
                (Shape::Paper, Shape::Scissor) => (0, 6),
                _ => (3, 3),
            };
            round_outcome.0 += self.0.score();
            round_outcome.1 += self.1.score();
            round_outcome
        }

        pub fn result_two(&self) -> (u32, u32) {
            match &self.1 {
                Shape::Rock => (
                    6 + self.0.score(),
                    match self.0 {
                        Shape::Rock => Shape::Scissor,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissor => Shape::Paper,
                    }
                    .score(),
                ),
                Shape::Paper => (3 + self.0.score(), 3 + self.0.score()),
                Shape::Scissor => (
                    self.0.score(),
                    6 + match self.0 {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissor,
                        Shape::Scissor => Shape::Rock,
                    }
                    .score(),
                ),
            }
        }
    }

    pub fn transform_input(input: &str) -> Vec<Round> {
        input.lines().map(|line| Round::from(line)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::rock_paper_scissors::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 15);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 12);
    }
}
