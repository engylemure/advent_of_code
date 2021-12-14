use smoke_basin::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod smoke_basin {
    pub type InputType = Vec<Vec<u32>>;

    fn is_the_lowest(
        number: u32,
        top: Option<u32>,
        right: Option<u32>,
        bottom: Option<u32>,
        left: Option<u32>,
    ) -> bool {
        top.map(|top| top > number).unwrap_or(true)
            && bottom.map(|bottom| bottom > number).unwrap_or(true)
            && right.map(|bottom| bottom > number).unwrap_or(true)
            && left.map(|bottom| bottom > number).unwrap_or(true)
    }

    pub fn solution_01(input: InputType) -> u32 {
        let mut risk_level = 0;
        let r_len = input.len();
        let c_len = input[0].len();
        for i in 0..r_len {
            for j in 0..c_len {
                let top = if i == 0 { None } else { Some(input[i - 1][j]) };
                let right = if j == c_len - 1 {
                    None
                } else {
                    Some(input[i][j + 1])
                };
                let bottom = if i == r_len - 1 {
                    None
                } else {
                    Some(input[i + 1][j])
                };
                let left = if j == 0 { None } else { Some(input[i][j - 1]) };
                if is_the_lowest(input[i][j], top, right, bottom, left) {
                    risk_level += input[i][j] + 1;
                }
            }
        }
        risk_level
    }

    pub fn solution_02(input: InputType) -> u32 {
        0
    }

    pub fn transform_input(input: &str) -> InputType {
        input
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::smoke_basin::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 15);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 61229);
    }
}
