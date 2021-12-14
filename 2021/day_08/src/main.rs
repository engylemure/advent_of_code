use seven_segment_search::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod seven_segment_search {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Segment {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

    pub type SignalPattern = HashSet<Segment>;

    #[derive(Debug)]
    pub struct Display {
        signal_patterns: Vec<SignalPattern>,
        outputs: Vec<SignalPattern>,
    }

    impl Display {
        pub fn new(input: (&str, &str)) -> Self {
            let (patterns, outputs) = input;
            Self {
                signal_patterns: patterns
                    .trim()
                    .split_whitespace()
                    .map(|s| s.chars().map(Segment::from).collect())
                    .collect(),
                outputs: outputs
                    .trim()
                    .split_whitespace()
                    .map(|s| s.chars().map(Segment::from).collect())
                    .collect(),
            }
        }

        pub fn decode_output(&self) -> usize {
            let digits = self
                .signal_patterns
                .iter()
                .fold(HashMap::new(), |mut acc, val| {
                    match val.len() {
                        2 => {
                            acc.insert(1u8, val);
                        }
                        4 => {
                            acc.insert(4, val);
                        }
                        3 => {
                            acc.insert(7, val);
                        }
                        7 => {
                            acc.insert(8, val);
                        }
                        _ => (),
                    }
                    acc
                });
            let digits = self.signal_patterns.iter().fold(digits, |mut acc, val| {
                match val.len() {
                    5 => {
                        if val.is_superset(acc[&1]) {
                            acc.insert(3, val);
                        } else if acc[&4].difference(val).count() == 1 {
                            acc.insert(5, val);
                        } else {
                            acc.insert(2, val);
                        }
                    }
                    6 => {
                        if acc[&4].difference(val).count() == 0 {
                            acc.insert(9, val);
                        } else if acc[&7].difference(val).count() == 0 {
                            acc.insert(0, val);
                        } else {
                            acc.insert(6, val);
                        }
                    }
                    _ => (),
                }
                acc
            });
            1000usize
                * digits
                    .iter()
                    .find(|(_, s)| ***s == self.outputs[0])
                    .map(|(k, _)| *k as usize)
                    .unwrap()
                + 100
                    * digits
                        .iter()
                        .find(|(_, s)| ***s == self.outputs[1])
                        .map(|(k, _)| *k as usize)
                        .unwrap()
                + 10 * digits
                    .iter()
                    .find(|(_, s)| ***s == self.outputs[2])
                    .map(|(k, _)| *k as usize)
                    .unwrap()
                + digits
                    .iter()
                    .find(|(_, s)| ***s == self.outputs[3])
                    .map(|(k, _)| *k as usize)
                    .unwrap()
        }
    }

    pub type InputType = Vec<Display>;

    impl From<char> for Segment {
        fn from(c: char) -> Self {
            match c {
                'a' => Self::A,
                'b' => Self::B,
                'c' => Self::C,
                'd' => Self::D,
                'e' => Self::E,
                'f' => Self::F,
                'g' => Self::G,
                _ => unreachable!(),
            }
        }
    }

    pub fn solution_01(input: InputType) -> usize {
        input
            .iter()
            .map(|d| {
                d.outputs
                    .iter()
                    .map(|p| p.len())
                    .filter(|len| match len {
                        2 | 4 | 3 | 7 => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()
    }

    pub fn solution_02(input: InputType) -> usize {
        input.iter().map(Display::decode_output).sum()
    }

    pub fn transform_input(input: &str) -> InputType {
        input
            .lines()
            .filter_map(|s| s.split_once('|'))
            .map(Display::new)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::seven_segment_search::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 26);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 61229);
    }
}
