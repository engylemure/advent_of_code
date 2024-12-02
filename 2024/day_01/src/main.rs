use historian_hysteria::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod historian_hysteria {
    use std::collections::HashMap;

    fn get_left_and_right_numbers(input: &str) -> (Vec<i32>, Vec<i32>) {
        input
            .lines()
            .map(|line| {
                let mut numbers = line.split_whitespace().flat_map(|num| num.parse::<i32>());
                (
                    numbers.next().unwrap_or_default(),
                    numbers.next().unwrap_or_default(),
                )
            })
            .fold((Vec::new(), Vec::new()), |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc
            })
    }

    pub fn solution_01(input: &str) -> u32 {
        let (mut left, mut right) = get_left_and_right_numbers(input);
        left.sort();
        right.sort();
        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum()
    }

    pub fn solution_02(input: &str) -> u32 {
        let (left, right) = get_left_and_right_numbers(input);
        let appearence_map = right.into_iter().fold(HashMap::new(), |mut acc, val| {
            match acc.entry(val) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    *(occupied_entry.get_mut()) += 1;
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            };
            acc
        });
        left.into_iter()
            .map(|val| match appearence_map.get(&val) {
                Some(appearence) => *appearence * (val as u32),
                None => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::historian_hysteria::*;

    #[test]
    fn test_01() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(solution_01(input), 11);
    }

    #[test]
    fn test_02() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(solution_02(input), 31);
    }
}
