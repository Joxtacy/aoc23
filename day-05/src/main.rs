use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::BTreeMap, ops::Range, vec};

fn main() {
    let input = include_str!("input.txt");

    // println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let seeds = input.lines().take(1).collect::<String>();
    let seeds: Vec<u64> = seeds
        .split_once(":")
        .expect("should have a colon")
        .1
        .trim()
        .split(" ")
        .map(|x| x.trim())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let map = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            let (section, ranges) = x.split_once(" map:\n").expect("should have map");

            let ranges: Vec<(Range<u64>, Range<u64>)> = ranges
                .trim()
                .split("\n")
                .map(|range| {
                    let mut x = range.trim().split(" ");
                    let (dest_start, source_start, length) = (
                        x.next().unwrap().parse::<u64>().unwrap(),
                        x.next().unwrap().parse::<u64>().unwrap(),
                        x.next().unwrap().parse::<u64>().unwrap(),
                    );
                    (
                        dest_start..dest_start + length,
                        source_start..source_start + length,
                    )
                })
                .collect();
            (section, ranges)
        })
        .collect::<BTreeMap<&str, Vec<(Range<u64>, Range<u64>)>>>();

    println!("Doing the mapping thing");
    seeds
        .iter()
        .map(|seed| {
            let seed_to_soil = map.get("seed-to-soil").unwrap();
            let soil = process(seed_to_soil, *seed);
            dbg!(&soil);
            let soil_to_ferilizer = map.get("soil-to-fertilizer").unwrap();
            let fertilizer = process(soil_to_ferilizer, soil);
            dbg!(&fertilizer);
            let fertilizer_to_water = map.get("fertilizer-to-water").unwrap();
            let water = process(fertilizer_to_water, fertilizer);
            dbg!(&water);
            let water_to_light = map.get("water-to-light").unwrap();
            let light = process(water_to_light, water);
            dbg!(&light);
            let light_to_temperature = map.get("light-to-temperature").unwrap();
            let temperature = process(light_to_temperature, light);
            dbg!(&temperature);
            let temperature_to_humidity = map.get("temperature-to-humidity").unwrap();
            let humidity = process(temperature_to_humidity, temperature);
            dbg!(&humidity);
            let humidity_to_location = map.get("humidity-to-location").unwrap();
            let location = process(humidity_to_location, humidity);
            dbg!(&location);
            location
        })
        .min()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let seeds = input.lines().take(1).collect::<String>();
    let seeds: Vec<u64> = seeds
        .split_once(":")
        .expect("should have a colon")
        .1
        .trim()
        .split(" ")
        .map(|x| x.trim())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut seed_ranges = vec![];
    let mut range_start = 0;

    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            range_start = *seed;
        } else {
            seed_ranges.push(range_start..range_start + *seed);
        }
    }
    // dbg!(&seed_ranges);

    for seed_range in &seed_ranges {
        dbg!(&seed_range);
    }

    let map = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            let (section, ranges) = x.split_once(" map:\n").expect("should have map");

            let ranges: Vec<(Range<u64>, Range<u64>)> = ranges
                .trim()
                .split("\n")
                .map(|range| {
                    let mut x = range.trim().split(" ");
                    let (dest_start, source_start, length) = (
                        x.next().unwrap().parse::<u64>().unwrap(),
                        x.next().unwrap().parse::<u64>().unwrap(),
                        x.next().unwrap().parse::<u64>().unwrap(),
                    );
                    (
                        dest_start..dest_start + length,
                        source_start..source_start + length,
                    )
                })
                .collect();
            (section, ranges)
        })
        .collect::<BTreeMap<&str, Vec<(Range<u64>, Range<u64>)>>>();

    println!("Doing the mapping thing");

    seed_ranges
        .into_par_iter()
        .flat_map(|seed_range| seed_range.clone())
        .map(|seed| {
            let seed_to_soil = map.get("seed-to-soil").unwrap();
            let soil = process(seed_to_soil, seed);
            //dbg!(&soil);
            let soil_to_ferilizer = map.get("soil-to-fertilizer").unwrap();
            let fertilizer = process(soil_to_ferilizer, soil);
            //dbg!(&fertilizer);
            let fertilizer_to_water = map.get("fertilizer-to-water").unwrap();
            let water = process(fertilizer_to_water, fertilizer);
            //dbg!(&water);
            let water_to_light = map.get("water-to-light").unwrap();
            let light = process(water_to_light, water);
            //dbg!(&light);
            let light_to_temperature = map.get("light-to-temperature").unwrap();
            let temperature = process(light_to_temperature, light);
            //dbg!(&temperature);
            let temperature_to_humidity = map.get("temperature-to-humidity").unwrap();
            let humidity = process(temperature_to_humidity, temperature);
            //dbg!(&humidity);
            let humidity_to_location = map.get("humidity-to-location").unwrap();
            let location = process(humidity_to_location, humidity);
            dbg!(&location);
            location
        })
        .min()
        .unwrap()
}

fn process(input: &Vec<(Range<u64>, Range<u64>)>, source: u64) -> u64 {
    // get destination by taking seed - source.start and add that diff to dest.start
    let d: Vec<u64> = input
        .iter()
        .filter_map(|(destination_range, source_range)| {
            if source_range.contains(&source) {
                let diff = source - source_range.start;
                let destination = destination_range.start + diff;
                Some(destination)
            } else {
                None
            }
        })
        .collect();

    if d.is_empty() {
        source
    } else {
        //dbg!(&d);
        d[0]
    }
    /*
    .map_or(source, |(dest_range, source_range)| {
        source_range
            .enumerate()
            .filter_map(|(i, value)| {
                if value == source {
                    Some(dest_range.start + i as u64)
                } else {
                    None
                }
            })
            .take(1)
            .next()
            .unwrap()
    })
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "seeds: 79 14 55 13

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
56 93 4";

        let res = part_1(input);

        assert_eq!(res, 35);
    }

    #[test]
    fn test_part_2() {
        let input = "seeds: 79 14 55 13

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
56 93 4";

        let res = part_2(input);

        assert_eq!(res, 46);
    }
}
