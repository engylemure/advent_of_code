use handheld_halting::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod handheld_halting {
    use std::str::FromStr;

    #[derive(Debug, Copy, Clone)]
    pub enum Operation {
        Nop(i32),
        Acc(i32),
        Jmp(i32),
    }

    impl FromStr for Operation {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut values = s.split_whitespace();
            if let (Some(op), Some(Ok(arg))) = (values.next(), values.next().map(str::parse)) {
                Ok(match op.trim() {
                    "nop" => Operation::Nop(arg),
                    "acc" => Operation::Acc(arg),
                    "jmp" => Operation::Jmp(arg),
                    _ => return Err(()),
                })
            } else {
                Err(())
            }
        }
    }

    pub struct Ast {
        operations: Vec<Operation>,
    }

    impl Ast {
        pub fn parse(input: &str) -> Ast {
            Self {
                operations: input
                    .lines()
                    .into_iter()
                    .map(|l| Operation::from_str(l).ok())
                    .flatten()
                    .collect(),
            }
        }

        pub fn execute(
            &self,
            change_op: Option<&dyn Fn(usize, &Operation) -> Operation>,
        ) -> Result<i32, i32> {
            let mut n_exec_of_op = vec![0; self.operations.len()];
            let mut acc = 0;
            let mut i = 0;
            while i < self.operations.len() {
                if n_exec_of_op[i] > 0 {
                    return Err(acc);
                }
                n_exec_of_op[i] += 1;
                let op = change_op
                    .map(|handler| handler(i, &self.operations[i]))
                    .unwrap_or(self.operations[i]);
                match op {
                    Operation::Nop(_) => {
                        i += 1;
                    }
                    Operation::Acc(arg) => {
                        i += 1;
                        acc += arg;
                    }
                    Operation::Jmp(arg) => {
                        i = ((i as i32) + arg) as usize;
                    }
                }
            }
            Ok(acc)
        }
    }

    pub fn solution_01(input: Ast) -> Option<i32> {
        Some(match input.execute(None) {
            Ok(result) => result,
            Err(acc_val) => acc_val,
        })
    }

    pub fn solution_02(input: Ast) -> Option<i32> {
        if let Ok(result) = input.execute(None) {
            Some(result)
        } else {
            for (idx, op) in input.operations.iter().enumerate() {
                match op {
                    Operation::Acc(_) => continue,
                    _ => {
                        let res = input.execute(Some(&|i, op| {
                            if i == idx {
                                match op {
                                    Operation::Nop(arg) => Operation::Jmp(*arg),
                                    Operation::Jmp(arg) => Operation::Nop(*arg),
                                    op => *op,
                                }
                            } else {
                                *op
                            }
                        }));
                        if let Ok(result) = res {
                            return Some(result);
                        }
                    }
                }
            }
            None
        }
    }

    pub fn transform_input(input: &str) -> Ast {
        Ast::parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::handheld_halting::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), Some(5));
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), Some(8));
    }
}
