use sonar_sweep::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod sonar_sweep {
    pub fn solution_01(input: Vec<u32>) -> u32 {
        let mut amount_increased = 0;
        let mut last_measurement = None;
        for measurement in input.iter() {
            match last_measurement {
                Some(last_measurement) if last_measurement < *measurement => {
                    amount_increased += 1;
                }
                _ => {}
            };
            last_measurement = Some(*measurement);
        }
        return amount_increased;
    }

    pub fn solution_02(input: Vec<u32>) -> u32 {
        let mut amount_increased = 0;
        let mut last_measurement_window_sum = None;
        for window in input.windows(3) {
            let measurement_window_sum: u32 = window.iter().sum();
            match last_measurement_window_sum {
                Some(last_measurement_window_sum)
                    if last_measurement_window_sum < measurement_window_sum =>
                {
                    amount_increased += 1;
                }
                _ => {}
            };
            last_measurement_window_sum = Some(measurement_window_sum);
        }
        return amount_increased;
    }

    pub fn transform_input(input: &str) -> Vec<u32> {
        input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::sonar_sweep::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 7);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 5);
    }
}
