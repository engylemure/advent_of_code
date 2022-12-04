use rucksack_reorganization::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod rucksack_reorganization {
    use std::collections::HashSet;

    type Input = Vec<RuckSack>;

    pub fn solution_01(input: Input) -> u32 {
        input
            .iter()
            .map(RuckSack::common_char_on_compartments)
            .filter_map(|c| {
                if c.is_ascii_alphabetic() {
                    Some((c as u32) - if c.is_lowercase() { 96 } else { 38 })
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn solution_02(input: Input) -> u32 {
        let mut sum = 0;
        let mut iter = input.iter();
        while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
            let a_items = a.items.iter().collect::<HashSet<_>>();
            let b_items = b.items.iter().collect::<HashSet<_>>();
            let c_items = c.items.iter().collect::<HashSet<_>>();
           if let Some(c) = a_items.intersection(&b_items).map(|c| *c).collect::<HashSet<_>>().intersection(&c_items).next() {
                sum += (**c as u32) - if c.is_lowercase() { 96 } else { 38 };
            }
        }
        sum 
    }

    pub struct RuckSack {
        items: Vec<char>,
    }

    impl From<&str> for RuckSack {
        fn from(line: &str) -> Self {
            Self {
                items: line.trim().chars().collect(),
            }
        }
    }

    impl RuckSack {
        fn common_char_on_compartments(&self) -> char {
            let first_items = self.items[0..self.items.len() / 2].iter().collect::<HashSet<_>>();
            for c in self.items[self.items.len() / 2 ..].iter() {
                if first_items.contains(&c) {
                    return *c;
                }
            }
            panic!("There is no common char on these compartments of the RuckSack!")
        }
    }

    pub fn transform_input(input: &str) -> Vec<RuckSack> {
        input.lines().map(RuckSack::from).collect()
    }
}

#[cfg(test)]
mod test {
    use super::rucksack_reorganization::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 157);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 70);
    }
}
