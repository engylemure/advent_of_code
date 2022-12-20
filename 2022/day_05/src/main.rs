use supply_stacks::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod supply_stacks {
    type Input = (StackCrates, Vec<Command>);
    type SolutionOne = String;
    type SolutionTwo = String;

    #[derive(Default, Debug)]
    pub struct StackCrates(Vec<Vec<char>>);

    #[derive(Debug)]
    pub struct Command {
        amount: usize,
        from: usize,
        to: usize,
    }

    impl Command {
        fn from_line(line: &str) -> Option<Self> {
            let line = line
                .replace("move ", "")
                .replace("from ", "\n")
                .replace("to ", "\n");
            let mut line_iter = line
                .lines()
                .map(|l| l.trim().parse());
            Some(
                Self {
                    amount: line_iter.next()?.ok()?,
                    from: line_iter.next()?.ok()?,
                    to: line_iter.next()?.ok()?
                }
            )
        }
    }

    impl StackCrates {
        pub fn move_crates(&mut self, amount: usize, from: usize, to: usize) {
            for _ in 0..amount {
                self.move_one_crate(from, to);
            }
        }

        pub fn move_crates_crate_mover_9001(&mut self, amount: usize, from: usize, to: usize) {
            let drain_idx = self.0[from - 1].len() - amount;
            let mut crates_to_move: Vec<_> = self.0[from - 1].drain(drain_idx..).collect();
            self.0[to - 1].append(&mut crates_to_move);
        }

        pub fn move_one_crate(&mut self, from: usize, to: usize) {
            if let Some(val) = self.0[from - 1].pop() {
                self.0[to - 1].push(val);
            }
        }
    }

    pub fn solution_01(mut input: Input) -> SolutionOne {
        for cmd in input.1 {
            let Command { amount, from, to } = cmd;
            input.0.move_crates(amount, from, to)
        }
        input.0.0.iter_mut().filter_map(|v| v.pop()).collect()
    }

    pub fn solution_02(mut input: Input) -> SolutionTwo {
        for cmd in input.1 {
            let Command { amount, from, to } = cmd;
            input.0.move_crates_crate_mover_9001(amount, from, to)
        }
        input.0.0.iter_mut().filter_map(|v| v.pop()).collect()
    }

    fn is_index_line(line: &str) -> bool {
        line.trim().starts_with(|c: char| c.is_ascii_digit())
    }

    fn convert_crate_line(line: Vec<char>) -> Vec<Option<char>> {
        let mut crate_line = Vec::new();
        for i in 0..=(line.len() / 4) {
            let c = line[i * 4 + 1];
            crate_line.push(c.is_alphabetic().then_some(c));
        }
        crate_line
    }

    pub fn transform_input(input: &str) -> Input {
        let mut lines = input.lines();
        let mut crates_input = Vec::new();
        while let Some(line) = lines.next() {
            if is_index_line(line) {
                break;
            }
            crates_input.push(convert_crate_line(line.chars().collect()));
        }
        let amount_of_stack_crates = crates_input[0].len();
        let stack_max_size = crates_input.len();
        let mut stack_crates = StackCrates(vec![vec![]; amount_of_stack_crates]);
        for i in 0..amount_of_stack_crates {
            for j in 1..=stack_max_size {
                if let Some(c) = crates_input[stack_max_size - j][i] {
                    stack_crates.0[i].push(c);
                }
            }
        }
        lines.next();
        (stack_crates, lines.filter_map(Command::from_line).collect())
    }
}

#[cfg(test)]
mod test {
    use super::supply_stacks::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), "CMZ");
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), "MCD");
    }
}
