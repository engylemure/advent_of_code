use treachery_whales::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod treachery_whales {
    use std::collections::{hash_map::Entry, HashMap};

    pub type InputType = Vec<u32>;
    fn calculate_fuel_spent(input: &[u32], pos: u32) -> u32 {
        input
            .iter()
            .map(|f| if *f > pos { f - pos } else { pos - f })
            .sum()
    }

    fn calculate_real_fuel_spent(
        input: &[u32],
        pos: u32,
        cached_fuel_spent_on_distance: &mut HashMap<u32, u32>,
    ) -> u32 {
        input
            .iter()
            .map(|f| {
                let distance = if *f > pos { f - pos } else { pos - f };
                match cached_fuel_spent_on_distance.entry(distance) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => *v.insert((1..=distance).sum::<u32>()),
                }
            })
            .sum()
    }

    pub fn solution_01(input: InputType) -> usize {
        let mut value: u32 = input.iter().sum::<u32>() / input.len() as u32;
        let mut fuel_spent: u32 = calculate_fuel_spent(&input, value);
        loop {
            let lower = {
                let lower_value = value.checked_sub(1);
                lower_value.map(|v| (v, calculate_fuel_spent(&input, v)))
            };
            let upper = {
                let upper_value = value.checked_add(1);
                upper_value.map(|v| (v, calculate_fuel_spent(&input, v)))
            };
            match (lower, upper) {
                (None, None) => unreachable!(),
                (None, Some(upper)) => {
                    if upper.1 <= fuel_spent {
                        fuel_spent = upper.1;
                        value = upper.0;
                    } else {
                        break fuel_spent as usize;
                    }
                }
                (Some(lower), None) => {
                    if lower.1 <= fuel_spent {
                        fuel_spent = lower.1;
                        value = lower.0;
                    } else {
                        break fuel_spent as usize;
                    }
                }
                (Some(lower), Some(upper)) => match (lower.1 > fuel_spent, upper.1 > fuel_spent) {
                    (true, false) => {
                        fuel_spent = upper.1;
                        value = upper.0;
                    }
                    (false, true) => {
                        fuel_spent = lower.1;
                        value = lower.0;
                    }
                    _ => break fuel_spent as usize,
                },
            }
        }
    }

    pub fn solution_02(input: InputType) -> usize {
        let mut value: u32 = input.iter().sum::<u32>() / input.len() as u32;
        let mut cached_fuel_spent_on_distance = HashMap::new();
        let mut fuel_spent: u32 =
            calculate_real_fuel_spent(&input, value, &mut cached_fuel_spent_on_distance);
        loop {
            let lower = {
                let lower_value = value.checked_sub(1);
                lower_value.map(|v| {
                    (
                        v,
                        calculate_real_fuel_spent(&input, v, &mut cached_fuel_spent_on_distance),
                    )
                })
            };
            let upper = {
                let upper_value = value.checked_add(1);
                upper_value.map(|v| {
                    (
                        v,
                        calculate_real_fuel_spent(&input, v, &mut cached_fuel_spent_on_distance),
                    )
                })
            };
            match (lower, upper) {
                (None, None) => unreachable!(),
                (None, Some(upper)) => {
                    if upper.1 <= fuel_spent {
                        fuel_spent = upper.1;
                        value = upper.0;
                    } else {
                        break fuel_spent as usize;
                    }
                }
                (Some(lower), None) => {
                    if lower.1 <= fuel_spent {
                        fuel_spent = lower.1;
                        value = lower.0;
                    } else {
                        break fuel_spent as usize;
                    }
                }
                (Some(lower), Some(upper)) => match (lower.1 > fuel_spent, upper.1 > fuel_spent) {
                    (true, false) => {
                        fuel_spent = upper.1;
                        value = upper.0;
                    }
                    (false, true) => {
                        fuel_spent = lower.1;
                        value = lower.0;
                    }
                    _ => break fuel_spent as usize,
                },
            }
        }
    }

    pub fn transform_input(input: &str) -> InputType {
        input.split(',').map(|line| line.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::treachery_whales::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 37);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 168);
    }
}
