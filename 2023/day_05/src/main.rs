use seed_fertilizer::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(input));
    println!("{}", solution_02(input));
}

mod seed_fertilizer {
    use std::str::Lines;

    fn consume_lines_into_map(lines: &mut Lines<'_>, map: &mut Vec<(usize, usize, usize)>) {
        for line in lines.by_ref() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let mut mapping = line.split_whitespace();
            map.push((
                mapping.next().expect("source").trim().parse().unwrap(),
                mapping.next().expect("destination").trim().parse().unwrap(),
                mapping.next().expect("range").trim().parse().unwrap(),
            ))
        }
    }

    fn find_on_map(seed: usize, map: &[(usize, usize, usize)]) -> usize {
        map.iter()
            .find_map(|(dst, src, step)| {
                (seed >= *src && seed < *src + *step).then(|| dst + (seed - src))
            })
            .unwrap_or(seed)
    }

    fn get_ranges_on_map(
        mut ranges: Vec<(usize, usize)>,
        map: &[(usize, usize, usize)],
    ) -> Vec<(usize, usize)> {
        let mut new_ranges = Vec::new();
        let mut next_ranges = Vec::new();
        for (dst, src, step) in map.iter() {
            let map_range = *src..(src + step);
            while let Some((start, range_step)) = ranges.pop() {
                let end = start + range_step - 1;
                let start_is_contained =  map_range.contains(&start);
                let end_is_contained = map_range.contains(&end);
                if start_is_contained {
                    if end_is_contained {
                        new_ranges.push((dst + (start - src), range_step));
                    } else {
                        new_ranges.push((dst + (start - src), src + step - start - 1));
                        next_ranges.push((end, end - src + step - 1));
                    }
                } else if end_is_contained && start < *src {
                    next_ranges.push((start, (src - start) - 1));
                    new_ranges.push((*dst, end - src));
                } else if start < *src && end >= src + step {
                    next_ranges.push((start, (src - start) - 1));
                    new_ranges.push((*dst, *step));
                    next_ranges.push((src + step, end));
                } else {
                    next_ranges.push((start, end));
                }
            }
            if next_ranges.is_empty() {
                break;
            }
            std::mem::swap(&mut ranges, &mut next_ranges);
        }
        new_ranges.extend(ranges);
        new_ranges
    }

    pub fn solution_01(input: &str) -> usize {
        let mut lines = input.lines();
        let seeds = lines
            .next()
            .expect("seeds line")
            .trim()
            .strip_prefix("seeds: ")
            .expect("seeds line")
            .split_whitespace()
            .map(|seed| seed.parse::<usize>().expect("valid number"));
        let mut seed_to_soil_map = Vec::new();
        let mut soil_to_fertilizer_map = Vec::new();
        let mut fertilizer_to_water_map = Vec::new();
        let mut water_to_light_map = Vec::new();
        let mut light_to_temperature_map = Vec::new();
        let mut temperature_to_humidity_map = Vec::new();
        let mut humidity_to_location_map = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match line {
                "seed-to-soil map:" => {
                    consume_lines_into_map(&mut lines, &mut seed_to_soil_map);
                }
                "soil-to-fertilizer map:" => {
                    consume_lines_into_map(&mut lines, &mut soil_to_fertilizer_map)
                }
                "fertilizer-to-water map:" => {
                    consume_lines_into_map(&mut lines, &mut fertilizer_to_water_map)
                }
                "water-to-light map:" => {
                    consume_lines_into_map(&mut lines, &mut water_to_light_map)
                }
                "light-to-temperature map:" => {
                    consume_lines_into_map(&mut lines, &mut light_to_temperature_map)
                }
                "temperature-to-humidity map:" => {
                    consume_lines_into_map(&mut lines, &mut temperature_to_humidity_map)
                }
                "humidity-to-location map:" => {
                    consume_lines_into_map(&mut lines, &mut humidity_to_location_map)
                }
                _ => continue,
            }
        }
        seeds
            .map(|seed| find_on_map(seed, &seed_to_soil_map))
            .map(|seed| find_on_map(seed, &soil_to_fertilizer_map))
            .map(|seed| find_on_map(seed, &fertilizer_to_water_map))
            .map(|seed| find_on_map(seed, &water_to_light_map))
            .map(|seed| find_on_map(seed, &light_to_temperature_map))
            .map(|seed| find_on_map(seed, &temperature_to_humidity_map))
            .map(|seed| find_on_map(seed, &humidity_to_location_map))
            .min()
            .unwrap_or_default()
    }

    pub fn solution_02(input: &str) -> usize {
        let mut lines = input.lines();
        let mut seeds = lines
            .next()
            .expect("seeds line")
            .trim()
            .strip_prefix("seeds: ")
            .expect("seeds line")
            .split_whitespace()
            .map(|seed| seed.parse::<usize>().expect("valid number"));
        let mut seed_to_soil_map = Vec::new();
        let mut soil_to_fertilizer_map = Vec::new();
        let mut fertilizer_to_water_map = Vec::new();
        let mut water_to_light_map = Vec::new();
        let mut light_to_temperature_map = Vec::new();
        let mut temperature_to_humidity_map = Vec::new();
        let mut humidity_to_location_map = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match line {
                "seed-to-soil map:" => {
                    consume_lines_into_map(&mut lines, &mut seed_to_soil_map);
                }
                "soil-to-fertilizer map:" => {
                    consume_lines_into_map(&mut lines, &mut soil_to_fertilizer_map)
                }
                "fertilizer-to-water map:" => {
                    consume_lines_into_map(&mut lines, &mut fertilizer_to_water_map)
                }
                "water-to-light map:" => {
                    consume_lines_into_map(&mut lines, &mut water_to_light_map)
                }
                "light-to-temperature map:" => {
                    consume_lines_into_map(&mut lines, &mut light_to_temperature_map)
                }
                "temperature-to-humidity map:" => {
                    consume_lines_into_map(&mut lines, &mut temperature_to_humidity_map)
                }
                "humidity-to-location map:" => {
                    consume_lines_into_map(&mut lines, &mut humidity_to_location_map)
                }
                _ => continue,
            }
        }
        let mut ranges = Vec::new();
        while let Some(range_start) = seeds.next() {
            let step = seeds.next().unwrap();
            ranges.push((range_start, step));
        }
        ranges = get_ranges_on_map(ranges, &seed_to_soil_map);
        ranges = get_ranges_on_map(ranges, &soil_to_fertilizer_map);
        ranges = get_ranges_on_map(ranges, &fertilizer_to_water_map);
        ranges = get_ranges_on_map(ranges, &water_to_light_map);
        ranges = get_ranges_on_map(ranges, &light_to_temperature_map);
        ranges = get_ranges_on_map(ranges, &temperature_to_humidity_map);
        ranges = get_ranges_on_map(ranges, &humidity_to_location_map);
        ranges.into_iter().map(|(start, _)| start).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::seed_fertilizer::*;
    const INPUT: &str = r#"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4"#;

    #[test]
    fn test_01() {
        assert_eq!(solution_01(INPUT), 35);
    }

    #[test]
    fn test_02() {
        assert_eq!(solution_02(INPUT), 46);
    }
}
