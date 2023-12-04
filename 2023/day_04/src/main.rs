use scratchcards::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod scratchcards {
    use std::{
        collections::{HashMap, HashSet},
        str::FromStr,
    };

    pub struct Card {
        id: usize,
        winning: HashSet<u32>,
        owned: HashSet<u32>,
    }

    impl Card {
        fn matches(&self) -> usize {
            self.winning
                .intersection(&self.owned)
                .count()
        }

        fn solution_01_points(&self) -> u64 {
            let matches = self.matches();
            if matches == 0 {
                0
            } else {
                2u64.pow((matches as u32) - 1)
            }
        }
    }

    impl FromStr for Card {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.trim().split(':');
            let id = s
                .next()
                .ok_or(())?
                .strip_prefix("Card ")
                .ok_or(())?
                .trim()
                .parse::<usize>()
                .map_err(|_| ())?;
            let mut s = s.next().ok_or(())?.trim().split("|");
            Ok(Self {
                id,
                winning: s
                    .next()
                    .ok_or(())?
                    .split_whitespace()
                    .filter_map(|s| s.trim().parse().ok())
                    .collect(),
                owned: s
                    .next()
                    .ok_or(())?
                    .split_whitespace()
                    .filter_map(|s| s.trim().parse().ok())
                    .collect(),
            })
        }
    }

    pub fn solution_01(input: &str) -> u64 {
        input
            .trim()
            .lines()
            .map(|line| Card::from_str(line).ok().unwrap().solution_01_points())
            .sum()
    }

    pub fn solution_02(input: &str) -> u64 {
        let mut card_copies: HashMap<usize, usize> = HashMap::new();
        input
            .trim()
            .lines()
            .map(|line| {
                let card = Card::from_str(line).unwrap();
                let matches = card.matches();
                let amount_of_card_copies =
                    card_copies.get(&(card.id - 1)).cloned().unwrap_or_default();
                for i in card.id..(card.id + matches) {
                    if let Some(card_copies) = card_copies.get_mut(&i) {
                        *card_copies += 1 + amount_of_card_copies;
                    } else {
                        card_copies.insert(i, 1 + amount_of_card_copies);
                    }
                }
                1 + (amount_of_card_copies as u64)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::scratchcards::*;

    #[test]
    fn test_01() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(solution_01(input), 13);
    }

    #[test]
    fn test_02() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(solution_02(input), 30);
    }
}
