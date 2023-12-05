use iter_tools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");

    let res1 = part_1(input);
    let res2 = part_2(input);

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

#[derive(Debug)]
enum Bob {
    Number(u32),
    Symbol(char),
    Gear,
    Empty,
}

fn part_1(input: &str) -> u32 {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (y as i32, x as i32),
                    match c {
                        '.' => Bob::Empty,
                        '0'..='9' => Bob::Number(c.to_digit(10).expect("should be a number")),
                        c => Bob::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Bob>>();

    // dbg!(&map);

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Bob::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_x, _), _)) => {
                            if last_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("shouldn't happen"),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }
    dbg!(&numbers);

    let mut total = 0;
    for num_list in numbers {
        // (x, y)
        let positions: [(i32, i32); 8] = [
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions
                    .iter()
                    .map(|outer_pos| (outer_pos.0 + pos.1, outer_pos.1 + pos.0))
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);

        let is_part_number = pos_to_check.iter().any(|pos| {
            if let Some(Bob::Symbol(_)) = map.get(pos) {
                true
            } else {
                false
            }
        });

        if is_part_number {
            let res = num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .join("")
                .parse::<u32>()
                .unwrap();
            dbg!(&res);
            total += res;
        }
    }
    total
}

fn part_2(input: &str) -> u32 {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (y as i32, x as i32),
                    match c {
                        '.' => Bob::Empty,
                        '0'..='9' => Bob::Number(c.to_digit(10).expect("should be a number")),
                        '*' => Bob::Gear,
                        c => Bob::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), Bob>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Bob::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_x, _), _)) => {
                            if last_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("shouldn't happen"),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }
    // dbg!(&numbers);

    let mut total = 0;

    for symbol in map.iter().filter(|d| matches!(d.1, Bob::Gear)) {
        dbg!(symbol);

        // (x, y)
        let positions: [(i32, i32); 8] = [
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| (outer_pos.0 + symbol.0 .1, outer_pos.1 + symbol.0 .0))
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);

        let mut indexes_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list.iter().any(|(num_pos, _)| num_pos == &pos) {
                    indexes_of_numbers.push(i);
                }
            }
        }
        dbg!(&indexes_of_numbers);

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .join("")
                        .parse::<u32>()
                        .unwrap()
                })
                .product::<u32>();
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let res = part_1(input);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let res = part_2(input);
        assert_eq!(res, 467835);
    }
}
