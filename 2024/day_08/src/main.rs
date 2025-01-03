use resonant_colinearity::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod resonant_colinearity {
    use std::collections::{HashMap, HashSet};

    type MapAndAntennas = (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>);
    fn parse(input: &str) -> MapAndAntennas {
        input
            .lines()
            .enumerate()
            .fold(Default::default(), |mut acc, (i, line)| {
                acc.0.push(
                    line.chars()
                        .enumerate()
                        .map(|(j, c)| {
                            if c != '.' {
                                match acc.1.entry(c) {
                                    std::collections::hash_map::Entry::Occupied(
                                        mut occupied_entry,
                                    ) => {
                                        occupied_entry.get_mut().push((i, j));
                                    }
                                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                                        vacant_entry.insert(vec![(i, j)]);
                                    }
                                }
                            }
                            c
                        })
                        .collect(),
                );
                acc
            })
    }

    pub fn solution_01(input: &str) -> u64 {
        let (map, antennas) = parse(input);
        let map_size = (map.len(), map[0].len());
        let mut antinodes = HashSet::new();
        for (_, locations) in antennas.iter() {
            for i in 0..locations.len() {
                let loc_a = &locations[i];
                for loc_b in locations.iter().skip(i + 1) {
                    let dist = (loc_a.0.abs_diff(loc_b.0), loc_a.1.abs_diff(loc_b.1));
                    for antinode in match (loc_a.0 < loc_b.0, loc_a.1 < loc_b.1) {
                        (true, true) => [
                            loc_a.0.checked_sub(dist.0).zip(loc_a.1.checked_sub(dist.1)),
                            loc_b.0.checked_add(dist.0).zip(loc_b.1.checked_add(dist.1)),
                        ],
                        (true, false) => [
                            loc_a.0.checked_sub(dist.0).zip(loc_a.1.checked_add(dist.1)),
                            loc_b.0.checked_add(dist.0).zip(loc_b.1.checked_sub(dist.1)),
                        ],
                        (false, true) => [
                            loc_a.0.checked_add(dist.0).zip(loc_a.1.checked_sub(dist.1)),
                            loc_b.0.checked_sub(dist.0).zip(loc_b.1.checked_add(dist.1)),
                        ],
                        (false, false) => [
                            loc_a.0.checked_add(dist.0).zip(loc_a.1.checked_add(dist.1)),
                            loc_b.0.checked_sub(dist.0).zip(loc_b.1.checked_sub(dist.1)),
                        ],
                    }
                    .into_iter()
                    .flatten()
                    .filter(|(i, j)| *i < map_size.0 && *j < map_size.1)
                    {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes.len() as u64
    }

    pub fn solution_02(input: &str) -> u64 {
        let (map, antennas) = parse(input);
        let map_size = (map.len(), map[0].len());
        let mut antinodes = HashSet::new();
        for (_, locations) in antennas.iter() {
            for i in 0..locations.len() {
                let loc_a = &locations[i];
                if !locations.is_empty() {
                    antinodes.insert(*loc_a);
                }
                for loc_b in locations.iter().skip(i + 1) {
                    let diff = (loc_a.0.abs_diff(loc_b.0), loc_a.1.abs_diff(loc_b.1));
                    let mut dist = diff;
                    while dist.0 < map_size.0 && dist.1 < map_size.1 {
                        for antinode in match (loc_a.0 < loc_b.0, loc_a.1 < loc_b.1) {
                            (true, true) => [
                                loc_a.0.checked_sub(dist.0).zip(loc_a.1.checked_sub(dist.1)),
                                loc_b.0.checked_add(dist.0).zip(loc_b.1.checked_add(dist.1)),
                            ],
                            (true, false) => [
                                loc_a.0.checked_sub(dist.0).zip(loc_a.1.checked_add(dist.1)),
                                loc_b.0.checked_add(dist.0).zip(loc_b.1.checked_sub(dist.1)),
                            ],
                            (false, true) => [
                                loc_a.0.checked_add(dist.0).zip(loc_a.1.checked_sub(dist.1)),
                                loc_b.0.checked_sub(dist.0).zip(loc_b.1.checked_add(dist.1)),
                            ],
                            (false, false) => [
                                loc_a.0.checked_add(dist.0).zip(loc_a.1.checked_add(dist.1)),
                                loc_b.0.checked_sub(dist.0).zip(loc_b.1.checked_sub(dist.1)),
                            ],
                        }
                        .into_iter()
                        .flatten()
                        .filter(|(i, j)| *i < map_size.0 && *j < map_size.1)
                        {
                            antinodes.insert(antinode);
                        }
                        dist.0 += diff.0;
                        dist.1 += diff.1;
                    }
                }
            }
        }
        antinodes.len() as u64
    }
}

#[cfg(test)]
mod test {
    use crate::resonant_colinearity::*;

    static INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn test_01() {
        assert_eq!(solution_01(INPUT), 14);
    }

    #[test]
    fn test_02() {
        assert_eq!(solution_02(INPUT), 34);
    }
}
