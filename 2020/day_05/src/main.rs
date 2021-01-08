use binary_boarding::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod binary_boarding {
    use std::{collections::HashMap, str::FromStr};

    #[derive(Copy, Clone)]
    pub enum Location {
        Front,
        Back,
        Left,
        Right,
    }

    #[derive(PartialEq, Debug)]
    pub struct SpacePartitioning {
        pub row: u8,
        pub column: u8,
        pub seat_id: u16,
    }

    impl Location {
        pub fn from_char(c: char) -> Result<Self, ()> {
            match c.to_ascii_uppercase() {
                'F' => Ok(Location::Front),
                'B' => Ok(Location::Back),
                'L' => Ok(Location::Left),
                'R' => Ok(Location::Right),
                _ => Err(()),
            }
        }
    }

    impl FromStr for SpacePartitioning {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let chars = s.chars();
            let mut data = Vec::with_capacity(10);
            for c in chars.take(10) {
                match Location::from_char(c) {
                    Ok(location) => data.push(location),
                    Err(_) => return Err(()),
                }
            }
            Self::new(data)
        }
    }

    impl SpacePartitioning {
        pub fn new(info: Vec<Location>) -> Result<Self, ()> {
            if info.len() == 10 {
                let row = info
                    .iter()
                    .enumerate()
                    .take(7)
                    .fold(0, |mut row, (i, loc)| {
                        let row_part = 128 / 2u8.pow((i as u32) + 1);
                        match loc {
                            Location::Back => row += row_part,
                            _ => (),
                        };
                        row
                    });
                let column = info
                    .iter()
                    .enumerate()
                    .skip(7)
                    .fold(0, |mut column, (i, loc)| {
                        let column_part = 8 / 2u8.pow((i as u32 - 7) + 1);
                        match loc {
                            Location::Right => column += column_part,
                            _ => (),
                        };
                        column
                    });
                Ok(Self {
                    row,
                    column,
                    seat_id: ((row as u16) * 8 + (column as u16)),
                })
            } else {
                Err(())
            }
        }
    }

    pub fn solution_01(input: Vec<Result<SpacePartitioning, ()>>) -> u16 {
        input
            .iter()
            .flatten()
            .max_by(|a, b| a.seat_id.cmp(&b.seat_id))
            .unwrap()
            .seat_id
    }

    pub fn solution_02(input: Vec<Result<SpacePartitioning, ()>>) -> Option<u16> {
        let map: HashMap::<u16, SpacePartitioning> = input
        .into_iter()
        .flatten()
        .map(|part| (part.seat_id, part))
        .collect();
        for (k, _) in map.iter() {
            match (map.get(k), map.get(&(k+1)), map.get(&(k+2))) {
                (Some(_), None, Some(_)) => {
                    return Some(k+1)
                },
                _ => ()
            }
        }
        None
    }

    pub fn transform_input(input: &str) -> Vec<Result<SpacePartitioning, ()>> {
        input.lines().map(SpacePartitioning::from_str).collect()
    }
}

#[cfg(test)]
mod test {
    use super::binary_boarding::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 820);
    }
}
