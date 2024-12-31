use bridge_repair::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod bridge_repair {
    fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + use<'_> {
        input.lines().filter_map(|line: &str| {
            let mut parts = line.trim().split(':');
            parts
                .next()
                .map(str::parse::<u64>)?
                .ok()
                .zip(parts.next().map(|nums| {
                    nums.split_whitespace()
                        .flat_map(str::parse::<u64>)
                        .collect::<Vec<_>>()
                }))
        })
    }
    pub fn solution_01(input: &str) -> u64 {
        parse(input)
            .filter_map(|(sum, numbers)| {
                let operations_size = (numbers.len() - 1) as u32;
                (1..=usize::pow(2, operations_size))
                    .filter_map(|val| {
                        let mut numbers_iter = numbers.iter();
                        (0..operations_size).map(|n| (val >> n) & 1 == 1).fold(
                            numbers_iter.next().cloned(),
                            |total, operation| {
                                Some(if operation {
                                    total? * numbers_iter.next()?
                                } else {
                                    total? + numbers_iter.next()?
                                })
                            },
                        )
                    })
                    .any(|operation_sum| operation_sum == sum)
                    .then(|| sum)
            })
            .sum()
    }

    #[derive(Clone, PartialEq, Eq, Copy, Debug)]
    pub enum Operation {
        Sum,
        Product,
        Concat,
    }

    pub fn operations(size: usize) -> Vec<Vec<Operation>> {
        let ops = [Operation::Sum, Operation::Product, Operation::Concat];
        (0..size)
            .map(|_| ops.to_vec())
            .fold(vec![vec![]], |result, pool| {
                result
                    .into_iter()
                    .flat_map(|x| {
                        pool.iter()
                            .map(|y| {
                                let mut res = x.clone();
                                res.push(*y);
                                res
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            })
    }

    pub fn solution_02(input: &str) -> u64 {
        parse(input)
            .filter_map(|(sum, numbers)| {
                operations(numbers.len() - 1)
                    .into_iter()
                    .filter_map(|ops| {
                        let mut numbers_iter = numbers.iter();
                        ops.into_iter()
                            .fold(numbers_iter.next().cloned(), |total, operation| {
                                Some(match operation {
                                    Operation::Sum => total? + numbers_iter.next()?,
                                    Operation::Product => total? * numbers_iter.next()?,
                                    Operation::Concat => {
                                        let mut total = total?.to_string();
                                        total.push_str(&numbers_iter.next()?.to_string());
                                        total.parse().ok()?
                                    }
                                })
                            })
                    })
                    .any(|op_sum| op_sum == sum)
                    .then(|| sum)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::bridge_repair::*;

    static INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn test_01() {
        assert_eq!(solution_01(INPUT), 3749);
    }

    #[test]
    fn test_02() {
        assert_eq!(solution_02(INPUT), 11387);
    }

    #[test]
    fn test_03() {
        dbg!(operations(2));
    }
}
