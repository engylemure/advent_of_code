use binary_diagnostic::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod binary_diagnostic {

    fn print_bingo_card(bingo_card: &[Vec<(u32, bool)>]) {
        for line in bingo_card {
            println!("{:?} {:?} {:?} {:?} {:?}", line[0], line[1], line[2], line[3], line[4]);
        }
        println!();
    }

    fn is_winner(bingo_card: &[Vec<(u32, bool)>]) -> bool {
        for i in 0..5 {
            if bingo_card[i][0].1
                && bingo_card[i][1].1
                && bingo_card[i][2].1
                && bingo_card[i][3].1
                && bingo_card[i][4].1
            {
                return true;
            }
            if bingo_card[0][i].1
                && bingo_card[1][i].1
                && bingo_card[2][i].1
                && bingo_card[3][i].1
                && bingo_card[4][i].1
            {
                return true;
            }
        }
        false
    }
    fn get_non_checked_numbers(bingo_card: &[Vec<(u32, bool)>]) -> u32 {
        bingo_card
            .iter()
            .map(|l| l.iter().map(|v| if v.1 { 0 } else { v.0 }).sum::<u32>())
            .sum()
    }
    fn verify_number(bingo_card: &mut BingoCard, number: u32) {
        for line in bingo_card {
            for n in line {
                if n.0 == number {
                    n.1 = true;
                }
            }
        }
    }

    pub fn solution_01(
        (random_numbers, mut bingo_cards): (RandomNumbers, Vec<(BingoCard, bool)>),
    ) -> u32 {
        for number in random_numbers {
            // println!("number {}", number);
            for bingo_card in bingo_cards.iter_mut() {
                verify_number(&mut bingo_card.0, number);
                // print_bingo_card(&bingo_card.0);
                if is_winner(&bingo_card.0) {
                    return get_non_checked_numbers(&bingo_card.0) * number;
                }
            }
        }
        0
    }

    pub fn solution_02(
        (random_numbers, mut bingo_cards): (RandomNumbers, Vec<(BingoCard, bool)>),
    ) -> u32 {
        let mut bingo_cards_len = bingo_cards.len();
        for number in random_numbers {
            println!("number {}", number);
            for (idx, bingo_card) in bingo_cards.iter_mut().enumerate() {
                if bingo_card.1 {
                    continue;
                }
                verify_number(&mut bingo_card.0, number);
                println!("bingo card {}", idx);
                print_bingo_card(&bingo_card.0);
                if is_winner(&bingo_card.0) {
                    bingo_card.1 = true;
                    if bingo_cards_len == 1 {
                        return get_non_checked_numbers(&bingo_card.0) * number;
                    }
                    bingo_cards_len -= 1;
                }
            }
        }
        0
    }

    type BingoCard = Vec<Vec<(u32, bool)>>;
    type RandomNumbers = Vec<u32>;

    pub fn transform_input(input: &str) -> (RandomNumbers, Vec<(BingoCard, bool)>) {
        let mut lines = input.lines();
        let random_numbers = lines.next().unwrap();
        let random_numbers = random_numbers
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let lines = lines.skip(1);
        let mut bingo_cards = Vec::new();
        let mut bingo_card = Vec::new();
        for line in lines {
            if line.is_empty() {
                bingo_cards.push((bingo_card, false));
                bingo_card = Vec::new();
                continue;
            }
            bingo_card.push(
                line.split_whitespace()
                    .map(|n| (n.parse().unwrap(), false))
                    .collect(),
            );
        }
        if !bingo_card.is_empty() {
            bingo_cards.push((bingo_card, false));
        }
        (random_numbers, bingo_cards)
    }
}

#[cfg(test)]
mod test {
    use super::binary_diagnostic::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 4512);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 1924);
    }
}
