use hill_climbing_algorithm::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod hill_climbing_algorithm {
    use std::collections::{HashMap, HashSet, VecDeque};
    type Input = Vec<Vec<char>>;
    type SolutionOne = usize;
    type SolutionTwo = u128;

    type Id = (usize, usize);

    pub fn solution_01(input: Input) -> SolutionOne {
        let digit_weights = ('a'..='z').enumerate().map(|(weight, c)| (c, weight)).collect::<HashMap<_,_>>();
        let mut weights = HashMap::<Id, HashSet<Id>>::new();
        let height = input.len();
        let width = input[0].len();
        for i in 0..height {
            for j in 0..width {
                let weights_for_position = [
                    i.checked_sub(1).and_then(|i| Some((i, j))),
                    j.checked_sub(1).and_then(|j| Some((i, j))),
                    (i + 1 < height).then(|| (i +1, j)),
                    (j + 1 < width).then(|| (i, j + 1))
                ].into_iter()
                .filter_map(
                        |possible_move|
                        possible_move.map(|pos| {
                            calculate_weight(&(i, j), pos, &input, &digit_weights)
                        })
                        .flatten()
                )
                .collect();
                weights.insert((i, j), weights_for_position);
            }
        }
        let (start, end) = (start_position(&input), end_position(&input));
        let mut distances = HashMap::<Id, usize>::new();
        let mut queue = VecDeque::new();
        distances.insert(start.clone(), 0);
        let mut visited = HashSet::<Id>::new();
        queue.push_back((0, start));
        while let Some((current_distance, current_pos)) = queue.pop_front() {
            if !visited.contains(&current_pos) {
                visited.insert(current_pos.clone());
                for pos in &weights[&current_pos] {
                    let should_update_weights = match distances.get(pos) {
                        None => true,
                        Some(distance) => *distance > current_distance + 1,
                    };
                    if should_update_weights {
                        distances.insert(pos.clone(), current_distance + 1);
                        queue.push_back((distances[&pos], pos.clone()))
                    }
                }
            }
        }
        distances[&end]
    }

    fn calculate_weight(start: &Id, destination: Id, input: &Input, digit_weights: &HashMap<char, usize>) -> Option<Id> {
        let start_char = *input.get(start.0)?.get(start.1)?;
        let destination_char = *input.get(destination.0)?.get(destination.1)?;
        if start_char == 'E' {
            return None;
        }
        if start_char == 'S' {
            return Some(destination)
        }
        match destination_char {
            'S' => None,
            'E' => (digit_weights[&'z']  <= (digit_weights[&start_char] + 1)).then(|| destination),
            destination_char => (digit_weights[&destination_char]  <= (digit_weights[&start_char] + 1)).then(|| destination)
        }
    }

    pub fn first_position_of(searched: char, input: &Input) -> Option<Id> {
        input
            .iter()
            .enumerate()
            .find_map(
                    |(i, line)|
                    line
                        .iter()
                        .enumerate()
                        .find_map(|(j, c)| ( *c == searched).then(|| (i, j)))
        )
    }

    pub fn start_position(input: &Input) -> Id {
        first_position_of('S', input).unwrap()
    }

    pub fn end_position(input: &Input) -> Id {
        first_position_of('E', input).unwrap()
    }

    // We will have to study and use the Chinese Reminder Theorem to handle this solution
    pub fn solution_02(input: Input) -> SolutionTwo {
        0
    }


    pub fn transform_input(input: &str) -> Input {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::hill_climbing_algorithm::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 31);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 0);
    }
}
