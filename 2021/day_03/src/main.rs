use binary_diagnostic::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod binary_diagnostic {
    use std::thread::*;
    fn generate_basic_diagnostic(input: &[Vec<char>]) -> Vec<(usize, usize)> {
        let mut diagnostic: Vec<(usize, usize)> = Vec::new();
        input[0].iter().for_each(|c| match c {
            '0' => {
                diagnostic.push((1, 0));
            }
            '1' => {
                diagnostic.push((0, 1));
            }
            _ => {}
        });
        for bin in input.iter().skip(1) {
            bin.iter().enumerate().for_each(|(idx, c)| match c {
                '0' => {
                    diagnostic[idx].0 += 1;
                }
                '1' => {
                    diagnostic[idx].1 += 1;
                }
                _ => {}
            });
        }
        diagnostic
    }

    pub fn solution_01(input: Vec<Vec<char>>) -> usize {
        let (least_common, most_common) = generate_basic_diagnostic(&input)
            .iter()
            .rev()
            .enumerate()
            .fold((0, 0), |mut acc, (idx, val)| {
                if val.0 > val.1 {
                    acc.0 += 2usize.pow(idx as u32);
                } else {
                    acc.1 += 2usize.pow(idx as u32);
                }
                acc
            });
        least_common * most_common
    }

    fn bin_vec_char_into_usize(vec_char: Vec<char>) -> usize {
        vec_char
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, c)| match c {
                '0' => 0,
                '1' => 2usize.pow(idx as u32),
                _ => unreachable!(),
            })
            .sum()
    }

    pub fn solution_02(input: Vec<Vec<char>>) -> usize {
        let bin_len = input[0].len();
        let mut oxy_vec = input.clone();
        let mut co2_vec = input;
        let oxy_handle = spawn(move || {
            for bin_idx in 0..bin_len {
                if oxy_vec.len() == 1 {
                    break;
                }
                let mut diagnostic = (0, 0);
                for bin in &oxy_vec {
                    match bin[bin_idx] {
                        '0' => {
                            diagnostic.0 += 1;
                        }
                        '1' => {
                            diagnostic.1 += 1;
                        }
                        _ => unreachable!(),
                    }
                }
                if diagnostic.0 > diagnostic.1 {
                    oxy_vec.retain(|bin| bin[bin_idx] == '0');
                } else {
                    oxy_vec.retain(|bin| bin[bin_idx] == '1');
                }
            }
            oxy_vec
        });
        let co2_handle = spawn(move || {
            for bin_idx in 0..bin_len {
                if co2_vec.len() == 1 {
                    break;
                }
                let mut diagnostic = (0, 0);
                for bin in &co2_vec {
                    match bin[bin_idx] {
                        '0' => {
                            diagnostic.0 += 1;
                        }
                        '1' => {
                            diagnostic.1 += 1;
                        }
                        _ => unreachable!(),
                    }
                }
                if diagnostic.0 > diagnostic.1 {
                    co2_vec.retain(|bin| bin[bin_idx] == '1');
                } else {
                    co2_vec.retain(|bin| bin[bin_idx] == '0');
                }
            }
            co2_vec
        });
        let mut co2_vec = co2_handle.join().unwrap();
        let mut oxy_vec = oxy_handle.join().unwrap();
        let oxy = bin_vec_char_into_usize(oxy_vec.pop().unwrap());
        let co2 = bin_vec_char_into_usize(co2_vec.pop().unwrap());
        oxy * co2
    }

    pub fn transform_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::binary_diagnostic::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 198);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 230);
    }
}
