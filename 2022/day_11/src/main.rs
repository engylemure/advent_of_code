use monkey_in_the_middle::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod monkey_in_the_middle {
    use std::str::FromStr;
    type Input = Vec<Monkey>;
    type SolutionOne = usize;
    type SolutionTwo = u128;

    #[derive(Debug)]
    pub struct Monkey {
        id: usize,
        items: Vec<usize>,
        operation: Vec<OperationToken>,
        test_dividend: usize,
        when_true_throw_to: usize,
        when_false_throw_to: usize
    }

    #[derive(Debug)]
    enum OperationToken {
        Old,
        Number(usize),
        Multiply,
        Add,
        Subtract,
        Divide
    }

    impl FromStr for OperationToken {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "*" => Self::Multiply,
                "+" => Self::Add,
                "-" => Self::Subtract,
                "/" => Self::Divide,
                "old" => Self::Old,
                maybe_number => Self::Number(maybe_number.parse().map_err(|_| ())?)
            })
        }
    }


    pub fn solution_01(input: Input) -> SolutionOne {
        let mut monkeys = input;
        let mut monkey_activites: Vec<(usize, usize)> = monkeys.iter().map(|m| (m.id, 0)).collect();
        for _ in 0..20 {
            for monkey_id in 0..monkeys.len() {
                // Starting the inspection
                monkey_activites[monkey_id].1 += monkeys[monkey_id].items.len();
                let items: Vec<_> = monkeys[monkey_id].items.drain(0..).collect();
                for item in items.into_iter() {
                    let first_arg = match monkeys[monkey_id].operation[0] {
                        OperationToken::Old => item,
                        OperationToken::Number(n) => n,
                        _ => panic!("This is not a valid arg!")
                    };
                    let second_arg = match monkeys[monkey_id].operation[2] {
                        OperationToken::Old => item,
                        OperationToken::Number(n) => n,
                        _ => panic!("This is not a valid arg!")
                    };
                    let worry_level = match monkeys[monkey_id].operation[1] {
                        OperationToken::Multiply => first_arg * second_arg,
                        OperationToken::Add => first_arg + second_arg,
                        OperationToken::Subtract => first_arg - second_arg,
                        OperationToken::Divide => first_arg / second_arg,
                        _ => panic!("This is not a valid operation!"),
                    } / 3;
                    let throw_to = if worry_level % monkeys[monkey_id].test_dividend == 0 {
                        monkeys[monkey_id].when_true_throw_to
                    } else {
                        monkeys[monkey_id].when_false_throw_to
                    };
                    monkeys[throw_to].items.push(worry_level);
                }
            }
        }
        monkey_activites.sort_by(|(_, activity_a), (_, activity_b)| activity_b.partial_cmp(activity_a).unwrap());
        monkey_activites[0].1 * monkey_activites[1].1
    }

    // We will have to study and use the Chinese Reminder Theorem to handle this solution
    pub fn solution_02(input: Input) -> SolutionTwo {
        let mut monkeys = input;
        let mut monkey_activites: Vec<(usize, u128)> = monkeys.iter().map(|m| (m.id, 0)).collect();
        for _ in 0..20 {
            for monkey_id in 0..monkeys.len() {
                // Starting the inspection
                monkey_activites[monkey_id].1 += monkeys[monkey_id].items.len() as u128;
                let items: Vec<_> = monkeys[monkey_id].items.drain(0..).collect();
                println!("{:?}", (monkey_id, &items));
                for item in items.into_iter() {
                    let first_arg = match monkeys[monkey_id].operation[0] {
                        OperationToken::Old => item,
                        OperationToken::Number(n) => n,
                        _ => panic!("This is not a valid arg!")
                    };
                    let second_arg = match monkeys[monkey_id].operation[2] {
                        OperationToken::Old => item,
                        OperationToken::Number(n) => n,
                        _ => panic!("This is not a valid arg!")
                    };
                    let test_dividend = monkeys[monkey_id].test_dividend;
                    let worry_level = match monkeys[monkey_id].operation[1] {
                        OperationToken::Multiply => first_arg * second_arg,
                        OperationToken::Add => first_arg + second_arg,
                        OperationToken::Subtract => first_arg - second_arg,
                        OperationToken::Divide => first_arg / second_arg,
                        _ => panic!("This is not a valid operation!"),
                    };
                    let test = worry_level % test_dividend;
                    let throw_to = if test == 0 {
                        monkeys[monkey_id].when_true_throw_to
                    } else {
                        monkeys[monkey_id].when_false_throw_to
                    };
                    monkeys[throw_to].items.push(worry_level);
                }
            }
        }
        monkey_activites.sort_by(|(_, activity_a), (_, activity_b)| activity_b.partial_cmp(activity_a).unwrap());
        println!("{:#?}", monkey_activites);
        monkey_activites[0].1 * monkey_activites[1].1
    }

    fn parse(lines: &[&str]) -> Option<Monkey> {
        let id: usize = lines[0][6..].replace(":", "").trim().parse().ok()?;
        let starting_items: Vec<usize> = lines[1][18..].split(", ").filter_map(|item| item.trim().parse().ok()).collect();
        let operation: Vec<OperationToken> = lines[2][18..].split_whitespace().filter_map(|item| item.trim().parse().ok()).collect();
        let test_dividend: usize = lines[3][21..].trim().parse().ok()?;
        let when_true_throw_to: usize = lines[4][29..].trim().parse().ok()?;
        let when_false_throw_to: usize = lines[5][30..].trim().parse().ok()?;
        Some(Monkey { id, items: starting_items, operation, test_dividend, when_true_throw_to, when_false_throw_to })
    }

    pub fn transform_input(input: &str) -> Input {
        input.lines().collect::<Vec<_>>().chunks(7).filter_map(parse).collect()
    }
}

#[cfg(test)]
mod test {
    use super::monkey_in_the_middle::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 10605);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 0);
    }
}
