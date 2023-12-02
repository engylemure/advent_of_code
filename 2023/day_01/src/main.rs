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
            .map(|line| {
                let mut numbers = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10))
                    .flatten();
                let first = numbers.next().unwrap_or_default();
                let mut last = first;
                while let Some(last_number) = numbers.next() {
                    last = last_number;
                }
                first * 10 + last
            })
            .sum()
    }

    fn map_numbers_line<'a>(line: &'a str) -> impl Iterator<Item = u32> + 'a {
        line.char_indices().filter_map(|(pos, c)| {
            if c.is_numeric() {
                c.to_digit(10)
            } else if line[pos..].starts_with("one") {
                Some(1)
            } else if line[pos..].starts_with("two") {
                Some(2)
            } else if line[pos..].starts_with("three") {
                Some(3)
            } else if line[pos..].starts_with("four") {
                Some(4)
            } else if line[pos..].starts_with("five") {
                Some(5)
            } else if line[pos..].starts_with("six") {
                Some(6)
            } else if line[pos..].starts_with("seven") {
                Some(7)
            } else if line[pos..].starts_with("eight") {
                Some(8)
            } else if line[pos..].starts_with("nine") {
                Some(9)
            } else {
                None
            }
        })
    }

    pub fn solution_02(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let mut numbers = map_numbers_line(line);
                let first = numbers.next().unwrap_or_default();
                let mut last = first;
                while let Some(last_number) = numbers.next() {
                    last = last_number;
                }
                first * 10 + last
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::trebuchet::*;

    #[test]
    fn test_01() {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        assert_eq!(solution_01(input), 142);
    }


    #[test]
    fn test_02() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        assert_eq!(solution_02(input), 281);
    }
}
