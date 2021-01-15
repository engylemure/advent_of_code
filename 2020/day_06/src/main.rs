use custom_customs::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod custom_customs {
    use std::collections::HashMap;

    pub struct CustomGroup(HashMap<char, u32>, u32);

    impl CustomGroup {
        pub fn from_str(input: &str) -> Self {
            Self::new(input.lines().map(|line| line.chars()).fold(
                (HashMap::new(), 0),
                |mut acc, chars| {
                    acc.1 += 1;
                    for c in chars {
                        match acc.0.entry(c) {
                            std::collections::hash_map::Entry::Occupied(oc) => {
                                *oc.into_mut() += 1;
                            }
                            std::collections::hash_map::Entry::Vacant(vc) => {
                                vc.insert(1);
                            }
                        };
                    }
                    acc
                },
            ))
        }

        pub fn new(input: (HashMap<char, u32>, u32)) -> Self {
            Self(input.0, input.1)
        }

        pub fn number_of_answered_questions(&self) -> u32 {
            self.0.len() as u32
        }

        pub fn number_of_answered_questions_by_everyone(&self) -> u32 {
            self.0.values().filter(|v| **v == self.1).count() as u32
        }
    }

    pub fn solution_01(input: Vec<CustomGroup>) -> u32 {
        input
            .iter()
            .map(CustomGroup::number_of_answered_questions)
            .sum()
    }

    pub fn solution_02(input: Vec<CustomGroup>) -> u32 {
        input
            .iter()
            .map(CustomGroup::number_of_answered_questions_by_everyone)
            .sum()
    }

    pub fn transform_input(input: &str) -> Vec<CustomGroup> {
        input.split("\n\n").map(CustomGroup::from_str).collect()
    }
}

#[cfg(test)]
mod test {
    use super::custom_customs::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 11);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 6);
    }
}
