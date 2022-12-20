use camp_cleanup::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod camp_cleanup {
    use std::collections::HashSet;

    type Input = Vec<SectionAssignments>;
    pub struct SectionAssignments((u32, u32), (u32, u32));

    impl SectionAssignments {
        fn has_full_intersection(&self) -> Option<()> {
            let one = ((self.0).0..=(self.0).1).collect::<HashSet<_>>();
            let two = ((self.1).0..=(self.1).1).collect::<HashSet<_>>();
            (one.is_superset(&two) || two.is_superset(&one)).then_some(())
        }

        fn has_intersection(&self) -> Option<()> {
            let one = ((self.0).0..=(self.0).1).collect::<HashSet<_>>();
            let two = ((self.1).0..=(self.1).1).collect::<HashSet<_>>();
            one.intersection(&two).next().map(|_| ())
        } 
    }

    pub fn solution_01(input: Input) -> u32 {
        input
            .iter()
            .filter_map(SectionAssignments::has_full_intersection)
            .count() as u32
    }

    pub fn solution_02(input: Input) -> u32 {
        input.iter()
            .filter_map(SectionAssignments::has_intersection)
            .count() as u32
    }

    fn get_range_from_str(range_str: &str) -> Option<(u32, u32)> {
        let mut iter = range_str.split('-');
        Some((iter.next()?.parse::<u32>().ok()?, iter.next()?.parse::<u32>().ok()?))
    }

    impl From<&str> for SectionAssignments {
        fn from(line: &str) -> Self {
            let mut iter = line.trim().split(',').filter_map(get_range_from_str);
            SectionAssignments(iter.next().unwrap(), iter.next().unwrap())
        }
    }

    pub fn transform_input(input: &str) -> Input {
        input.lines().map(SectionAssignments::from).collect()
    }
}

#[cfg(test)]
mod test {
    use super::camp_cleanup::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 2);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 4);
    }
}
