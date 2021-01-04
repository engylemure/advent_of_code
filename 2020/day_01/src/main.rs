use report_repair::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod report_repair {
    pub fn solution_01(input: Vec<u32>) -> u32 {
        for (i, a) in input.iter().enumerate() {
            for (j, b) in input.iter().enumerate() {
                if i == j {
                    continue
                }
                if (a + b) == 2020 {
                    return a*b
                }
            }
        }
        return 0
    }

    pub fn solution_02(input: Vec<u32>) -> u32 {
        for (i, a) in input.iter().enumerate() {
            for (j, b) in input.iter().enumerate() {
                for (k, c) in input.iter().enumerate() {
                    if i == j || j == k || k == i {
                        continue
                    }
                    
                    if (a + b + c) == 2020 {
                        dbg!((a,b,c));
                        return a*b*c
                    }
                }
            }
        }
        return 0
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
    use super::report_repair::*;
    
    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 514579);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 241861950);
    }
}
