use treetop_tree_house::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod treetop_tree_house {
    type Input = Vec<Vec<u32>>;
    type SolutionOne = usize;
    type SolutionTwo = String;

    fn is_visible(i: usize, j: usize, input: &Input) -> bool {
        let val = input[i][j];
        input[i][0..j].iter().find(|v| **v >= val).is_none()
            || input[i][j+1..].iter().find(|v| **v >= val).is_none()
            || input[0..i].iter().find(|v| v[j] >= val).is_none()
            || input[i+1..].iter().find(|v| v[j] >= val).is_none()
    }

    pub fn solution_01(input: Input) -> SolutionOne {
        let height = input.len();
        let width = input[0].len();
        let on_edge = (height + width) * 2 - 4;
        on_edge
            + (0..((height * width) - on_edge))
                .filter_map(|pos| {
                    let i = pos / (width - 2) + 1;
                    let j = pos % (width - 2) + 1;
                    is_visible(i, j, &input).then_some(())
                })
                .count()
    }

    pub fn solution_02(mut input: Input) -> SolutionTwo {
        String::new()
    }

    pub fn transform_input(input: &str) -> Input {
        input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::treetop_tree_house::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 21);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), "MCD");
    }
}
