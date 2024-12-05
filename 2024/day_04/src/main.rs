use ceres_search::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod ceres_search {

    fn is_xmas(chars: &[char]) -> bool {
        chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X']
    }

    fn is_xmas_two(one: &[char], two: &[char]) -> bool {
        (one == ['M', 'A', 'S'] || one == ['S', 'A', 'M'])
            && (two == ['M', 'A', 'S'] || two == ['S', 'A', 'M'])
    }

    fn search_xmas(map: &[Vec<char>]) -> u32 {
        let height = map.len();
        let width = map[0].len();
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut diagonal = 0;
        for i in 0..height {
            horizontal += &map[i]
                .windows(4)
                .filter_map(|chars| is_xmas(chars).then_some(()))
                .count();
            if i <= height - 4 {
                vertical += (0..width)
                    .filter_map(|j| {
                        is_xmas(&[map[i][j], map[i + 1][j], map[i + 2][j], map[i + 3][j]])
                            .then_some(())
                    })
                    .count();
                diagonal += (0..width - 3)
                    .map(|j| {
                        [
                            map[i][j],
                            map[i + 1][j + 1],
                            map[i + 2][j + 2],
                            map[i + 3][j + 3],
                        ]
                    })
                    .chain((3..width).map(|j| {
                        [
                            map[i + 3][j - 3],
                            map[i + 2][j - 2],
                            map[i + 1][j - 1],
                            map[i][j],
                        ]
                    }))
                    .filter_map(|chars| is_xmas(&chars).then_some(()))
                    .count();
            }
        }
        (horizontal + vertical + diagonal) as u32
    }

    fn search_xmas_two(map: &[Vec<char>]) -> u32 {
        map.windows(3)
            .flat_map(|lines| {
                (0..(lines[0].len() - 2)).map(|j| {
                    (
                        [lines[0][j], lines[1][j + 1], lines[2][j + 2]],
                        [lines[0][j + 2], lines[1][j + 1], lines[2][j]],
                    )
                })
            })
            .filter_map(|(one, two)| is_xmas_two(&one, &two).then_some(1))
            .sum()
    }

    pub fn solution_01(input: &str) -> u32 {
        search_xmas(
            &input
                .lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    pub fn solution_02(input: &str) -> u32 {
        search_xmas_two(
            &input
                .lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::ceres_search::*;

    #[test]
    fn test_01() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(solution_01(input), 18);
    }

    #[test]
    fn test_02() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(solution_02(input), 9);
    }
}
