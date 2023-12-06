use wait_for_it::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod wait_for_it {
    use std::str::Lines;

    pub fn distance(race_lasts: usize, time_with_button_pressed: usize) -> usize {
        (race_lasts - time_with_button_pressed) * time_with_button_pressed
    }

    fn bhaskara(a: f64, b: f64, c: f64, positive_delta: bool) -> f64 {
        (-b + if positive_delta { 1.0 } else { -1.0 } * (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)
    }

    pub fn necessary_pressed_time_for_distance(distance: usize, race_lasts: usize) -> (f64, f64) {
        (
            bhaskara(-1.0, race_lasts as f64, -(distance as f64), true).ceil(),
            bhaskara(-1.0, race_lasts as f64, -(distance as f64), false).ceil(),
        )
    }

    fn parse_s_01_numbers_on_line<'a>(
        lines: &mut Lines<'a>,
        prefix_to_strip: &'static str,
    ) -> impl Iterator<Item = usize> + 'a {
        lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix(prefix_to_strip)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|time| time.trim().parse::<usize>().unwrap())
    }

    fn amount_of_times_with_distances_greater_than_best_distance(
        (time, best_distance): (usize, usize),
    ) -> usize {
        let (mut min, mut max) = necessary_pressed_time_for_distance(best_distance, time);
        if distance(time, min as u64 as usize) <= best_distance {
            min += 1.0;
        }
        if distance(time, max as u64 as usize) > best_distance {
            max += 1.0;
        }
        (min.max(max) - min.min(max)) as u64 as usize
    }

    fn parse_s_02_number_on_line<'a>(
        lines: &mut Lines<'a>,
        prefix_to_strip: &'static str,
    ) -> usize {
        lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix(prefix_to_strip)
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
    }

    pub fn solution_01(input: &str) -> usize {
        let mut lines = input.lines();
        parse_s_01_numbers_on_line(&mut lines, "Time:")
            .zip(parse_s_01_numbers_on_line(&mut lines, "Distance:"))
            .map(amount_of_times_with_distances_greater_than_best_distance)
            .product()
    }

    pub fn solution_02(input: &str) -> usize {
        let mut lines = input.lines();
        let time = parse_s_02_number_on_line(&mut lines, "Time:");
        let best_distance = parse_s_02_number_on_line(&mut lines, "Distance:");
        amount_of_times_with_distances_greater_than_best_distance((time, best_distance))
    }
}

#[cfg(test)]
mod test {
    use crate::wait_for_it::*;
    const INPUT: &str = r#"Time:      7  15   30
    Distance:  9  40  200"#;

    mod distance {
        use crate::wait_for_it::distance;

        #[test]
        fn test_01() {
            assert_eq!(distance(7, 1), 6);
        }

        #[test]
        fn test_02() {
            assert_eq!(distance(7, 1), 6);
        }
    }

    #[test]
    fn test_01() {
        assert_eq!(solution_01(INPUT), 288);
    }

    #[test]
    fn test_02() {
        assert_eq!(solution_02(INPUT), 71503);
    }
}
