use handy_haversacks::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod handy_haversacks {
    use std::collections::{HashMap};
    #[derive(Debug)]
    pub struct BagRules(HashMap<String, HashMap<String, usize>>);

    impl BagRules {
        pub fn from_str(input: &str) -> Self {
            Self(input.lines().map(Self::rule_from_str).flatten().collect())
        }

        fn rule_from_str(input: &str) -> Option<(String, HashMap<String, usize>)> {
            let mut id_and_rules = input.split(" bags contain");
            let id = id_and_rules.next()?.to_string();
            let rules_from_id = id_and_rules
                .next()?
                .trim()
                .strip_suffix(".")?
                .split(',')
                .map(|rule| match rule {
                    "contain no other bags" => None,
                    rule => {
                        let rule = rule.trim();
                        let space_first_occurence = rule.find(' ')?;
                        let (count, id) = rule.split_at(space_first_occurence);
                        let id = id.trim();
                        Some((
                            id.strip_suffix(" bags")
                                .or_else(|| id.strip_suffix(" bag"))?
                                .to_string(),
                            count.parse::<usize>().ok()?,
                        ))
                    }
                })
                .flatten()
                .collect();
            Some((id, rules_from_id))
        }
        fn update_cached_amout<'a>(
            &'a self,
            bag_id: &'a str,
            bag_id_to_verify: &'a str,
            cached_amount: &mut HashMap<&'a str, usize>,
        ) -> usize {
            if bag_id_to_verify == bag_id {
                return 1
            }
            let bag_rule = self.0.get(bag_id_to_verify).unwrap();
            let mut amount = 0;
            for (k, v) in bag_rule {
                let amount_from_bag = if k == bag_id {
                    *v
                } else {
                    match cached_amount.get(k.as_str()) {
                        Some(value) => v*value,
                        None => v * self.update_cached_amout(bag_id, k, cached_amount),
                    }
                };
                amount += amount_from_bag;
            }
            cached_amount.insert(bag_id_to_verify, amount);
            amount
        }
        fn amount_of_bag_id_by_bag<'a>(
            &'a self,
            bag_id: &'a str,
        ) -> HashMap<&'a str, usize> {
            let mut cached_amount = HashMap::new();
            cached_amount.insert(bag_id, 1);
            for (k, _) in self.0.iter() {
                self.update_cached_amout(bag_id, k, &mut cached_amount);
            }
            cached_amount
        }

        fn amount_of_bags_from_bag_id(&self, bag_id: &str) -> usize {
            let bag_rule = self.0.get(bag_id).unwrap();
            let mut amount = 0;
            for (k, v) in bag_rule {
                amount+= v + v*self.amount_of_bags_from_bag_id(k)
            }
            amount
        }
    }

    pub fn solution_01(input: BagRules) -> Option<u32> {
        Some(
            input
                .amount_of_bag_id_by_bag("shiny gold")
                .iter()
                .filter(|(k, _)| **k != "shiny gold")
                .fold(0, |acc, (_, val)| acc + if *val >= 1 { 1 } else { 0 }),
        )
    }

    pub fn solution_02(input: BagRules) -> u32 {
        input.amount_of_bags_from_bag_id("shiny gold") as u32
    }

    pub fn transform_input(input: &str) -> BagRules {
        BagRules::from_str(input)
    }
}

#[cfg(test)]
mod test {
    use super::handy_haversacks::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), Some(4));
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 32);
    }
}
