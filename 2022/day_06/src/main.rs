use tuning_trouble::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod tuning_trouble {
    use std::{collections::HashSet, str::FromStr};

    type Input = Device;
    type SolutionOne = usize;
    type SolutionTwo = usize;

    pub struct Device {
        datastream_buffer: String,
    }

    impl FromStr for Device {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self {
                datastream_buffer: s.to_string(),
            })
        }
    }

    impl Device {
        pub fn start_of_packet_marker(&self) -> Option<(usize, String)> {
            self.marker(4)
        }

        pub fn start_of_message_marker(&self) -> Option<(usize, String)> {
            self.marker(14)
        }

        pub fn marker(&self, size: usize) -> Option<(usize, String)> {
            for (i, chars) in self
                .datastream_buffer
                .chars()
                .collect::<Vec<_>>()
                .windows(size)
                .enumerate()
            {
                if chars.iter().collect::<HashSet<_>>().len() == size {
                    return Some((i + size, chars.iter().collect::<String>()))
                }
            }
            None
        }
    }

    pub fn solution_01(mut input: Input) -> SolutionOne {
        input.start_of_packet_marker().unwrap().0
    }

    pub fn solution_02(mut input: Input) -> SolutionTwo {
        input.start_of_message_marker().unwrap().0
    }

    fn convert_crate_line(line: Vec<char>) -> Vec<Option<char>> {
        let mut crate_line = Vec::new();
        for i in 0..=(line.len() / 4) {
            let c = line[i * 4 + 1];
            crate_line.push(c.is_alphabetic().then_some(c));
        }
        crate_line
    }

    pub fn transform_input(input: &str) -> Input {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::tuning_trouble::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 7);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 19);
    }
}
