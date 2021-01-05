use tobbogan_trajectory::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(&transform_input(input), (3, 1)));
    println!("{}", solution_02(&transform_input(input)));
}

mod tobbogan_trajectory {

    pub fn solution_01(input: &Vec<Vec<char>>, slope: (usize, usize)) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut n_trees = 0;
        let (y_length, x_length) = (input.len(), input[0].len());
        loop {
            let x_plus_slope_x = x + slope.0;
            x = if x_plus_slope_x >= x_length {
                x_plus_slope_x % x_length
            } else {
                x_plus_slope_x
            };
            y += slope.1;

            if y >= y_length {
                break;
            }
            if input[y][x] == '#' {
                n_trees += 1;
            }
        }
        n_trees
    }

    pub fn solution_02(input: &Vec<Vec<char>>) -> usize {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|slope| solution_01(input, *slope))
            .fold(1, |acc, val| acc * val)
    }

    pub fn transform_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::tobbogan_trajectory::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(&transform_input(input), (3, 1)), 7);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(&transform_input(input)), 336);
    }
}
