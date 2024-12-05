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
        let horizontal_xmas = map
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.windows(4)
                    .enumerate()
                    .map(move |(j, chars)| ((std::iter::repeat(i)).zip(j..j + 4), chars))
            })
            .filter_map(|(indices, chars)| is_xmas(chars).then_some(indices));
        let vertical_xmas = map
            .windows(4)
            .enumerate()
            .flat_map(|(i, lines)| {
                (0..lines[0].len()).map(move |j| {
                    (
                        (i..i + 4).zip(std::iter::repeat(j)),
                        [lines[0][j], lines[1][j], lines[2][j], lines[3][j]],
                    )
                })
            })
            .filter_map(|(indices, chars)| is_xmas(&chars).then_some(indices));
        let diagonal_xmas = map
            .windows(4)
            .enumerate()
            .flat_map(|(i, lines)| {
                (0..(lines[0].len() - 3))
                    .map(move |j| {
                        (
                            Box::new((i..(i + 4)).zip(j..=(j + 3)))
                                as Box<dyn Iterator<Item = (usize, usize)>>,
                            [
                                lines[0][j],
                                lines[1][j + 1],
                                lines[2][j + 2],
                                lines[3][j + 3],
                            ],
                        )
                    })
                    .chain((3..lines[0].len()).map(move |j| {
                        (
                            Box::new((i..(i + 4)).rev().zip(j - 3..=j))
                                as Box<dyn Iterator<Item = (usize, usize)>>,
                            [
                                lines[3][j - 3],
                                lines[2][j - 2],
                                lines[1][j - 1],
                                lines[0][j],
                            ],
                        )
                    }))
            })
            .filter_map(|(indices, chars)| is_xmas(&chars).then_some(indices));
        (dbg!(horizontal_xmas.count()) + dbg!(vertical_xmas.count()) + dbg!(diagonal_xmas.count()))
            as u32
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
