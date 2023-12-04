use gear_radios::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod gear_radios {
    use std::collections::{HashMap, HashSet};

    #[derive(Default)]
    struct NumBuilder {
        id_counter: usize,
        num_string: String,
        positions: Vec<(usize, usize)>,
    }

    impl NumBuilder {
        fn push_char(&mut self, c: char, pos: (usize, usize)) {
            self.num_string.push(c);
            self.positions.push(pos);
        }
        fn build(&mut self) -> Option<(usize, u64, Vec<(usize, usize)>)> {
            if !self.num_string.is_empty() {
                let num = self.num_string.parse::<u64>().ok()?;
                self.num_string.clear();
                let positions = self.positions.drain(..).collect();
                let id = self.id_counter;
                self.id_counter += 1;
                Some((id, num, positions))
            } else {
                None
            }
        }
    }

    #[derive(Default, Debug)]
    struct Database {
        num_map: HashMap<usize, u64>,
        num_pos_map: HashMap<(usize, usize), usize>,
        special_positions: Vec<(usize, usize)>,
    }

    impl Database {
        fn try_build_number(&mut self, num_builder: &mut NumBuilder) {
            if let Some((id, num, positions)) = num_builder.build() {
                self.num_map.insert(id, num);
                for position in positions {
                    self.num_pos_map.insert(position, id);
                }
            }
        }
    }

    pub fn solution_01(input: &str) -> u64 {
        let (db, _) = input.trim().lines().enumerate().fold(
            (Database::default(), NumBuilder::default()),
            |mut acc, (i, line)| {
                let line = line.trim();
                for (j, c) in line.char_indices() {
                    match c {
                        '0'..='9' => {
                            acc.1.push_char(c, (i, j));
                        }
                        '.' => {
                            acc.0.try_build_number(&mut acc.1);
                        }
                        _ => {
                            acc.0.special_positions.push((i, j));
                            acc.0.try_build_number(&mut acc.1);
                        }
                    }
                }
                acc.0.try_build_number(&mut acc.1);
                acc
            },
        );
        let Database {
            special_positions,
            mut num_map,
            mut num_pos_map,
        } = db;
        let mut numbers_to_add = HashSet::new();
        for (i, j) in special_positions {
            let i_less_one = i.checked_sub(1);
            let j_less_one = j.checked_sub(1);
            for pos in [
                i_less_one.and_then(|i| j_less_one.map(|j| (i, j))),
                i_less_one.and_then(|i| Some((i, j))),
                i_less_one.and_then(|i| Some((i, j + 1))),
                j_less_one.and_then(|j| Some((i, j))),
                Some((i, j)),
                Some((i, j + 1)),
                j_less_one.and_then(|j| Some((i + 1, j))),
                Some((i + 1, j)),
                Some((i + 1, j + 1)),
            ]
            .into_iter()
            .filter_map(|pos| pos)
            {
                if let Some(id) = num_pos_map.remove(&pos) {
                    numbers_to_add.insert(id);
                }
            }
        }
        numbers_to_add
            .into_iter()
            .filter_map(|id| num_map.remove(&id))
            .sum()
    }

    pub fn solution_02(input: &str) -> u64 {
        let (db, _) = input.trim().lines().enumerate().fold(
            (Database::default(), NumBuilder::default()),
            |mut acc, (i, line)| {
                let line = line.trim();
                for (j, c) in line.char_indices() {
                    match c {
                        '0'..='9' => {
                            acc.1.push_char(c, (i, j));
                        }
                        '.' => {
                            acc.0.try_build_number(&mut acc.1);
                        }
                        c => {
                            if c == '*' {
                                acc.0.special_positions.push((i, j));
                            }
                            acc.0.try_build_number(&mut acc.1);
                        }
                    }
                }
                acc.0.try_build_number(&mut acc.1);
                acc
            },
        );
        let Database {
            special_positions,
            num_map,
            num_pos_map,
        } = db;
        special_positions
            .into_iter()
            .filter_map(|(i, j)| {
                let i_less_one = i.checked_sub(1);
                let j_less_one = j.checked_sub(1);
                let numbers = [
                    i_less_one.and_then(|i| j_less_one.map(|j| (i, j))),
                    i_less_one.and_then(|i| Some((i, j))),
                    i_less_one.and_then(|i| Some((i, j + 1))),
                    j_less_one.and_then(|j| Some((i, j))),
                    Some((i, j)),
                    Some((i, j + 1)),
                    j_less_one.and_then(|j| Some((i + 1, j))),
                    Some((i + 1, j)),
                    Some((i + 1, j + 1)),
                ]
                .into_iter()
                .filter_map(|pos| pos.and_then(|pos| num_pos_map.get(&pos).cloned()))
                .collect::<HashSet<_>>();
                if numbers.len() == 2 {
                    Some(numbers.into_iter().map(|id| num_map[&id]).product::<u64>())
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::gear_radios::*;

    #[test]
    fn test_01() {
        let input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;
        assert_eq!(solution_01(input), 4361);
    }

    #[test]
    fn test_02() {
        let input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;
        assert_eq!(solution_02(input), 467835);
    }
}
