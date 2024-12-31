use guard_guallivant::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod guard_guallivant {
    use std::collections::HashSet;

    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    fn move_01(
        map: &mut Vec<Vec<char>>,
        position: (char, usize, usize),
    ) -> Option<(char, usize, usize)> {
        let i = position.1;
        let j = position.2;
        let (i, j, on_collision) = match position.0 {
            '^' => i.checked_sub(1).map(|i| (i, j, '>')),
            '>' => Some((i, j + 1, 'v')),
            'v' => Some((i + 1, j, '<')),
            '<' => j.checked_sub(1).map(|j| (i, j, '^')),
            _ => None,
        }?;
        match map.get(i)?.get(j)? {
            '.' => {
                map[i][j] = position.0;
                map[position.1][position.2] = '.';
                Some((position.0, i, j))
            }
            '#' | 'O' => move_01(map, (on_collision, position.1, position.2)),
            _ => None,
        }
    }
    pub fn solution_01(input: &str) -> u32 {
        let mut position = None;
        let mut map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        match c {
                            '#' | '.' => {}
                            _ => position = Some((c, i, j)),
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut position = position;
        let mut visited = HashSet::new();
        while let Some(p) = position {
            visited.insert((p.1, p.2));
            position = move_01(&mut map, p);
        }
        visited.len() as u32
    }

    fn is_loop_with_obstacle(
        mut map: Vec<Vec<char>>,
        mut already_traversed: Vec<(char, usize, usize)>,
    ) -> (bool, (usize, usize)) {
        let obstacle_pos = already_traversed.pop().unwrap();
        map[obstacle_pos.1][obstacle_pos.2] = 'O';
        let already_visited_points = already_traversed
            .iter()
            .cloned()
            .map(|(_, i, j)| (i, j))
            .collect::<HashSet<_>>();
        if already_visited_points.contains(&(obstacle_pos.1, obstacle_pos.2)) {
            return (false, (obstacle_pos.1, obstacle_pos.2));
        }
        let mut position = already_traversed.pop();
        let mut visited: HashSet<(char, usize, usize)> = already_traversed.into_iter().collect();
        let mut is_loop = false;
        while let Some(p) = position {
            if visited.contains(&p) {
                is_loop = true;
                break;
            }
            visited.insert(p);
            position = move_01(&mut map, p);
        }
        (is_loop, (obstacle_pos.1, obstacle_pos.2))
    }

    pub fn solution_02(input: &str) -> u32 {
        let mut position = None;
        let mut map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        match c {
                            '#' | '.' => {}
                            _ => position = Some((c, i, j)),
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut position = position;
        let mut route = Vec::new();
        while let Some(p) = position {
            route.push(p);
            position = move_01(&mut map, p);
        }
        (2..route.len())
            .into_par_iter()
            .filter(|i| is_loop_with_obstacle(map.clone(), route[..=*i].to_vec()).0)
            .count() as u32
    }
}

#[cfg(test)]
mod test {
    use crate::guard_guallivant::*;

    #[test]
    fn test_01() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(solution_01(input), 41);
    }

    #[test]
    fn test_02() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(solution_02(input), 6);
    }
}
