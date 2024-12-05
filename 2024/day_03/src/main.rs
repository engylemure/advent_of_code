use mul_it_over::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod mul_it_over {
    use std::num::ParseIntError;

    pub fn solution_01(input: &str) -> u32 {
        let mut mul_stack_str = String::new();
        let mut numbers_str = String::new();
        let mut numbers = Vec::new();
        for c in input.chars() {
            match (mul_stack_str.as_ref(), c) {
                ("mul(", ')') => {
                    match numbers_str
                        .split(',')
                        .map(|n| n.parse::<u32>())
                        .collect::<Result<Vec<_>, ParseIntError>>()
                    {
                        Ok(numbers_to_multiply) if numbers_to_multiply.len() > 1 => {
                            numbers.push(numbers_to_multiply);
                        }
                        _ => {}
                    }
                    mul_stack_str.clear();
                    numbers_str.clear();
                }
                ("", 'm') | ("m", 'u') | ("mu", 'l') | ("mul", '(') => {
                    mul_stack_str.push(c);
                }
                ("mul(", '0'..='9') | ("mul(", ',') => {
                    numbers_str.push(c);
                }
                _ => {
                    mul_stack_str.clear();
                    numbers_str.clear();
                }
            }
        }
        numbers
            .into_iter()
            .map(|numbers| numbers.into_iter().product::<u32>())
            .sum()
    }

    pub fn solution_02(input: &str) -> u32 {
        let mut mul_enabled = true;
        let mut stack_str = String::new();
        let mut numbers_str = String::new();
        let mut numbers = Vec::new();
        for c in input.chars() {
            match (stack_str.as_ref(), c) {
                ("", 'd')
                | ("d", 'o')
                | ("do", 'n')
                | ("don", '\'')
                | ("don'", 't')
                | ("do", '(')
                | ("don't", '(') => {
                    stack_str.push(c);
                }
                ("don't(", ')') => {
                    mul_enabled = false;
                    stack_str.clear();
                    numbers_str.clear();
                }
                ("do(", ')') => {
                    mul_enabled = true;
                    stack_str.clear();
                    numbers_str.clear();
                }
                ("", 'm') | ("m", 'u') | ("mu", 'l') | ("mul", '(') if mul_enabled => {
                    stack_str.push(c);
                }
                ("mul(", '0'..='9') | ("mul(", ',') if mul_enabled => {
                    numbers_str.push(c);
                }
                ("mul(", ')') if mul_enabled => {
                    match numbers_str
                        .split(',')
                        .map(|n| n.parse::<u32>())
                        .collect::<Result<Vec<_>, ParseIntError>>()
                    {
                        Ok(numbers_to_multiply) if numbers_to_multiply.len() > 1 => {
                            numbers.push(numbers_to_multiply);
                        }
                        _ => {}
                    }
                    stack_str.clear();
                    numbers_str.clear();
                }
                _ => {
                    stack_str.clear();
                    numbers_str.clear();
                }
            }
        }
        numbers
            .into_iter()
            .map(|numbers| numbers.into_iter().product::<u32>())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::mul_it_over::*;

    #[test]
    fn test_01() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(solution_01(input), 161);
    }

    #[test]
    fn test_02() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(solution_02(input), 48);
    }
}
