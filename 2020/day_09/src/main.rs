use encoding_error::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input), 25));
    println!("{:?}", solution_02(transform_input(input), 25));
}

mod encoding_error {

    #[derive(Debug)]
    pub struct Xmas(Vec<u64>);

    impl Xmas {
        fn parse(input: &str) -> Self {
            Self(input.lines().map(str::parse).flatten().collect())
        }

        fn weak_value(&self, preamble_size: usize) -> Option<(usize, u64)> {
            let mut i = preamble_size;
            let mut window = self.0.windows(preamble_size);
            while i < self.0.len() {
                let val = self.0[i];
                match window.next() {
                    Some(window) => {
                        if !Self::satisfy_rule(window, val) {
                            return Some((i, val));
                        }
                    }
                    None => break,
                };
                i += 1;
            }
            None
        }

        fn satisfy_rule(slice: &[u64], value: u64) -> bool {
            for (i, a) in slice.iter().enumerate() {
                for (j, b) in slice.iter().enumerate() {
                    if i != j && b + a == value {
                        return true;
                    }
                }
            }
            false
        }
    }

    pub fn solution_01(input: Xmas, preamble_size: usize) -> Option<u64> {
        input.weak_value(preamble_size).map(|(_, val)| val)
    }

    pub fn solution_02(input: Xmas, preamble_size: usize) -> Option<u64> {
        let (_, weak) = input.weak_value(preamble_size)?;
        for i in 0..(input.0.len() - 1) {
            let mut sum = input.0[i];
            let mut min = sum;
            let mut max = sum;
            for j in (i + 1)..input.0.len() {
                sum += input.0[j] as u64;
                if input.0[j] > max {
                    max = input.0[j];
                }
                if input.0[j] < min {
                    min = input.0[j];
                }
                if sum == weak && (j - i) > 0 {
                    return Some(min + max);
                }
            }
        }
        None
    }

    pub fn transform_input(input: &str) -> Xmas {
        Xmas::parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::encoding_error::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input), 5), Some(127));
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input), 5), Some(62));
    }
}
