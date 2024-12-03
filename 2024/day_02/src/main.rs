use red_nosed_reports::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod red_nosed_reports {
    fn unsafe_number_idx<'a>(numbers: impl Iterator<Item = &'a i32>) -> Option<usize> {
        let mut numbers = numbers.enumerate();
        let mut prev_idx_and_number = numbers.next()?;
        let mut is_increasing = None;
        while let Some((idx, num)) = numbers.next() {
            let diff = prev_idx_and_number.1 - num;
            if diff.abs() < 1 || diff.abs() > 3 {
                return Some(idx);
            }
            match is_increasing {
                Some(is_increasing) => {
                    if is_increasing != (diff < 0) {
                        return Some(idx);
                    }
                }
                None => is_increasing = Some(diff < 0),
            }
            prev_idx_and_number = (idx, num);
        }
        None
    }
    pub fn solution_01(input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                unsafe_number_idx(
                    line.trim()
                        .split_whitespace()
                        .map(|num| num.parse::<i32>())
                        .flatten()
                        .collect::<Vec<i32>>()
                        .iter(),
                )
                .is_none()
                .then_some(1)
            })
            .sum()
    }

    pub fn solution_02(input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                let numbers = line
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse::<i32>())
                    .flatten()
                    .collect::<Vec<i32>>();
                if let Some(unsafe_idx) = unsafe_number_idx(numbers.iter()) {
                    let is_safe = unsafe_number_idx(
                        numbers
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, num)| (idx != unsafe_idx).then_some(num)),
                    )
                    .is_none()
                        || unsafe_number_idx(
                            numbers
                                .iter()
                                .enumerate()
                                .filter_map(|(idx, num)| (idx != (unsafe_idx - 1)).then_some(num)),
                        )
                        .is_none()
                        || unsafe_number_idx(numbers.iter().skip(1)).is_none();
                    is_safe.then_some(1)
                } else {
                    Some(1)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::red_nosed_reports::*;

    #[test]
    fn test_01() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(solution_01(input), 2);
    }

    #[test]
    fn test_02() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(solution_02(input), 4);
    }
}
