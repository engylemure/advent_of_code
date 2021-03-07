use seating_system::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod seating_system {
    #[derive(Debug)]
    pub struct AdapterBag {
        adapters: Vec<u64>,
    }

    impl AdapterBag {
        fn parse(input: &str) -> Self {
            let mut adapters: Vec<u64> = input.lines().map(str::parse).flatten().collect();
            adapters.sort();
            adapters.push(adapters[adapters.len() - 1] + 3);
            Self { adapters }
        }
    }

    pub fn solution_01(input: AdapterBag) -> u32 {
        let mut diffs = (0, 0, 0);
        let mut last_value = 0;
        for i in 0..input.adapters.len() {
            let value = input.adapters[i];
            match value - last_value {
                1 => diffs.0 += 1,
                2 => diffs.1 += 1,
                3 => diffs.2 += 1,
                _ => unreachable!(),
            }
            last_value = value;
        }
        diffs.0 * diffs.2
    }

    pub fn solution_02(input: AdapterBag) -> u64 {
        let mut last_value = 0;
        let diff_vec = input
            .adapters
            .iter()
            .enumerate()
            .map(|(i, adapter)| {
                let value = input.adapters[i];
                let diff = value - last_value;
                last_value = value;
                diff
            })
            .collect();
        
        // 0 1 1 2 3 1 1 1 1
        // 0 1 2 3
    }

    pub fn transform_input(input: &str) -> AdapterBag {
        AdapterBag::parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::seating_system::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 220);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 0);
    }
}
