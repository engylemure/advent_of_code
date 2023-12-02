use trebuchet::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod trebuchet {
    pub fn solution_01(input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let mut splitted_line = line.split(":");
                let game: u32 = splitted_line
                    .next()
                    .unwrap()
                    .strip_prefix("Game ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let is_valid = splitted_line
                    .next()
                    .unwrap()
                    .split(";")
                    .map(|set| {
                        set.split(",").all(|cube_set| {
                            let mut splitted_cube_set = cube_set.trim().split_whitespace();
                            let number_of_cubes =
                                splitted_cube_set.next().unwrap().parse::<u32>().unwrap();
                            match splitted_cube_set.next().unwrap() {
                                "red" => number_of_cubes <= 12,
                                "green" => number_of_cubes <= 13,
                                "blue" => number_of_cubes <= 14,
                                _ => false,
                            }
                        })
                    })
                    .all(|a| a);
                is_valid.then_some(game)
            })
            .sum()
    }

    pub fn solution_02(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let line = line.trim();
                let mut splitted_line = line.split(":");
                splitted_line.next();
                let min_amount_of_cubes = splitted_line
                    .next()
                    .unwrap()
                    .split(";")
                    .map(|set| {
                        set.split(",").fold((0, 0, 0), |mut acc, cube_set| {
                            let mut splitted_cube_set = cube_set.trim().split_whitespace();
                            let number_of_cubes =
                                splitted_cube_set.next().unwrap().parse::<u32>().unwrap();
                            match splitted_cube_set.next().unwrap() {
                                "red" if acc.0 < number_of_cubes => {
                                    acc.0 = number_of_cubes;
                                }
                                "green" if acc.1 < number_of_cubes => {
                                    acc.1 = number_of_cubes;
                                }
                                "blue" if acc.2 < number_of_cubes => {
                                    acc.2 = number_of_cubes;
                                }
                                _ => {}
                            };
                            acc
                        })
                    })
                    .fold((0, 0, 0), |mut acc, val| {
                        if val.0 > acc.0 {
                            acc.0 = val.0;
                        }
                        if val.1 > acc.1 {
                            acc.1 = val.1;
                        }
                        if val.2 > acc.2 {
                            acc.2 = val.2;
                        }
                        acc
                    });
                min_amount_of_cubes.0 * min_amount_of_cubes.1 * min_amount_of_cubes.2
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::trebuchet::*;

    #[test]
    fn test_01() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solution_01(input), 8);
    }

    #[test]
    fn test_02() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(solution_02(input), 2286);
    }
}
