use lanternfish::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod lanternfish {
    fn print_list(input: &Vec<u32>) {
        println!(
            "{}",
            input
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    }

    pub fn improved_dumb_solution(mut input: Vec<u32>, n_days: u32) -> Vec<u32> {
        let loop_amount = n_days / 7;
        let loop_amount = if loop_amount * 7 <= n_days {
            loop_amount + 1
        } else {
            loop_amount
        };
        for i in 1..=loop_amount {
            let mut children = Vec::new();
            let days_leap = if i * 7 > n_days { n_days % 7 } else { 7 };
            for n in input.iter_mut() {
                if let Some(new_n) = n.checked_sub(days_leap) {
                    *n = new_n;
                } else {
                    let days_to_skip = (days_leap - *n);
                    let new_n = 7 - days_to_skip;
                    // dbg!(8 - new_n, new_n, *n, days_leap);
                    *n = new_n;
                    children.push(9 - days_to_skip);
                }
            }
            input.extend(children);
        }
        input
    }

    pub fn dumb_solution(mut input: Vec<u32>, n_days: u32) -> Vec<u32> {
        // println!("i_len: {}, n_days: {}", input.len(), 0);
        for i in 0..n_days {
            let mut children = Vec::new();
            for n in input.iter_mut() {
                if *n == 0 {
                    *n = 6;
                    children.push(8);
                } else {
                    *n -= 1;
                }
            }
            input.extend(children);
            // println!("i_len: {}, n_days: {}", input.len(), i + 1);
        }
        input
    }

    pub fn stupid_solution(input: Vec<u32>, n_days: u32) -> usize {
        let mut cache = vec![1, 2, 2, 2, 2, 2, 2, 2, 3];
        if n_days > 9 {
            for i in 9..=n_days as usize {
                cache.push(cache[i - 9] + cache[i - 7]);
            }
        }
        input
            .iter()
            .map(|i| {
                if let Some(days_left) = n_days.checked_sub(*i) {
                    cache[days_left as usize]
                } else {
                    1
                }
            })
            .sum()
    }

    pub fn solution_01(input: Vec<u32>) -> usize {
        stupid_solution(input, 80)
    }

    pub fn solution_02(input: Vec<u32>) -> usize {
        stupid_solution(input, 256)
    }

    pub fn transform_input(input: &str) -> Vec<u32> {
        input
            .trim()
            .split(',')
            .map(|line| line.parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::lanternfish::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 5934);
    }

    #[test]
    fn test_01a() {
        let input = transform_input(include_str!("example_input"));
        let days = 80;
        let mut expected = dumb_solution(input.clone(), days);
        let mut solution = improved_dumb_solution(input.clone(), days);
        expected.sort_unstable();
        solution.sort_unstable();
        assert_eq!(solution, expected);
        assert_eq!(stupid_solution(input, days), expected.len());
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 26984457539);
    }

    #[test]
    fn test_03() {
        let input = "0";
        assert_eq!(dumb_solution(transform_input(input), 18).len(), 3);
    }

    #[test]
    fn simulate() {
        let input = "0";
        assert_eq!(dumb_solution(transform_input(input), 29).len(), 17);
    }
}
