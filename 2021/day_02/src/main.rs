use dive::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod dive {
    use std::str::FromStr;

    pub enum SubmarineAction {
        Forward,
        Up,
        Down,
    }

    impl FromStr for SubmarineAction {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "forward" => Ok(Self::Forward),
                "up" => Ok(Self::Up),
                "down" => Ok(Self::Down),
                _ => Err(()),
            }
        }
    }
    pub struct SubmarineCmd {
        action: SubmarineAction,
        units: usize,
    }

    impl FromStr for SubmarineCmd {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut input = s.split_whitespace();
            let action = input.next().ok_or(())?.parse()?;
            let units = input.next().ok_or(())?.parse().map_err(|_| ())?;
            Ok(SubmarineCmd { action, units })
        }
    }

    pub fn solution_01(input: Vec<SubmarineCmd>) -> usize {
        let mut position = (0, 0);
        for cmd in input.iter() {
            match cmd.action {
                SubmarineAction::Forward => position.0 += cmd.units,
                SubmarineAction::Up => position.1 -= cmd.units,
                SubmarineAction::Down => position.1 += cmd.units,
            }
        }
        return position.0 * position.1;
    }

    pub fn solution_02(input: Vec<SubmarineCmd>) -> usize {
        let mut position = (0, 0);
        let mut aim = 0;
        for cmd in input.iter() {
            match cmd.action {
                SubmarineAction::Forward => {
                    position.0 += cmd.units;
                    position.1 += aim * cmd.units;
                }
                SubmarineAction::Up => {
                    aim -= cmd.units;
                }
                SubmarineAction::Down => {
                    aim += cmd.units;
                }
            }
        }
        return position.0 * position.1;
    }

    pub fn transform_input(input: &str) -> Vec<SubmarineCmd> {
        input
            .lines()
            .map(|line| line.parse::<SubmarineCmd>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::dive::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 150);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 900);
    }
}
