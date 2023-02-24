use cathode_ray_tube::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod cathode_ray_tube {
    type Input = Vec<Command>;
    type SolutionOne = i32;
    type SolutionTwo = String;

    pub type Command = Option<i32>;

    const LAST_CYCLE_ON_SOLUTION_ONE: i32 = 220;

    pub fn solution_01(input: Input) -> SolutionOne {
        let mut register = 1;
        let mut cycle = 1;
        let mut signal_strength = 0;
        let mut relevant_cycle = 20;
        for cmd in input {
            let cycles_to_add = cmd.is_some().then_some(2).unwrap_or(1);
            cycle += cycles_to_add;
            let add_amount = cmd.unwrap_or(0);
            if cycle > relevant_cycle {
                signal_strength += relevant_cycle * register;
                relevant_cycle += 40;
            } else if cycle == relevant_cycle {
                signal_strength += relevant_cycle * (register + add_amount);
                relevant_cycle += 40;
            }
            register += add_amount;
            if cycle >= LAST_CYCLE_ON_SOLUTION_ONE {
                break;
            }
        }
        signal_strength
    }

    pub fn solution_02(input: Input) -> SolutionTwo {
        let mut register: i32 = 1;
        let mut cycle: i32 = 1;
        let mut crt = String::new();
        for cmd in input {
            let add_amount = cmd.unwrap_or(0);
            for _ in 0..cmd.is_some().then_some(2).unwrap_or(1) {
                if (cycle - 1).abs_diff(register) <= 1 {
                    crt.push('#');
                } else {
                    crt.push('.');
                }
                cycle += 1;
                if cycle % 41 == 0 {
                    crt.push('\n');
                    cycle = 1;
                }
            }
            register += add_amount;
        }
        crt
    }

    fn parse_line(line: &str) -> Option<Command> {
        let mut line_iter = line.split_whitespace();
        match line_iter.next()?.trim() {
            "noop" => Some(None),
            "addx" => Some(Some(line_iter.next()?.parse().ok()?)),
            _ => panic!("failed parsing input!")
        }
    }

    pub fn transform_input(input: &str) -> Input {
        input.lines().filter_map(parse_line).collect()
    }
}

#[cfg(test)]
mod test {
    use super::cathode_ray_tube::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 13140);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        let expected = include_str!("solution_two_expected_output");
        assert_eq!(solution_02(transform_input(input)), expected);
    }
}
