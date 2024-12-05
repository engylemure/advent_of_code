use print_queue::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod print_queue {
    use std::collections::{HashMap, HashSet};

    pub fn solution_01(input: &str) -> u32 {
        let mut lines = input.lines();
        let mut precedence_map: HashMap<u32, HashSet<u32>> = HashMap::new();
        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            let mut numbers = line.split("|").flat_map(str::parse::<u32>);
            match precedence_map.entry(numbers.next().unwrap()) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().extend(numbers)
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(numbers.collect::<HashSet<_>>());
                }
            }
        }
        lines
            .flat_map(|line| {
                let numbers = line
                    .trim()
                    .split(',')
                    .flat_map(str::parse::<u32>)
                    .collect::<Vec<_>>();
                let mut is_correct = true;
                'outer: for (i, num) in numbers.iter().enumerate() {
                    if let Some(expected_next) = precedence_map.get(num) {
                        for prev_number in numbers.iter().take(i) {
                            if expected_next.contains(prev_number) {
                                is_correct = false;
                                break 'outer;
                            }
                        }
                    }
                }
                is_correct.then(|| numbers[numbers.len() / 2])
            })
            .sum()
    }

    pub fn solution_02(input: &str) -> u32 {
        let mut lines = input.lines();
        let mut precedence_map: HashMap<u32, HashSet<u32>> = HashMap::new();
        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            let mut numbers = line.split("|").flat_map(str::parse::<u32>);
            match precedence_map.entry(numbers.next().unwrap()) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().extend(numbers)
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(numbers.collect::<HashSet<_>>());
                }
            }
        }
        lines
            .flat_map(|line| {
                let mut numbers = line
                    .trim()
                    .split(',')
                    .flat_map(str::parse::<u32>)
                    .collect::<Vec<_>>();
                let mut is_correct = true;
                'outer: for (i, num) in numbers.iter().enumerate() {
                    if let Some(expected_next) = precedence_map.get(num) {
                        for prev_number in numbers.iter().take(i) {
                            if expected_next.contains(prev_number) {
                                is_correct = false;
                                break 'outer;
                            }
                        }
                    }
                }
                if !is_correct {
                    for i in 0..numbers.len() {
                        if let Some(expected_next) = precedence_map.get(&numbers[i]) {
                            for j in 0..i {
                                if expected_next.contains(&numbers[j]) {
                                    numbers.swap(i, j);
                                }
                            }
                        }
                    }
                }
                (!is_correct).then(|| numbers[numbers.len() / 2])
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::print_queue::*;

    #[test]
    fn test_01() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(solution_01(input), 143);
    }

    #[test]
    fn test_02() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(solution_02(input), 123);
    }
}
