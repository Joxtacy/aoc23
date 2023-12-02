use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let res1 = part1(input);
    let res2 = part2(input);

    println!("{}", res1);
    println!("{}", res2);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game, rest) = line
                .split_once(':')
                .expect("should have a game id and a game state");

            let id = game
                .split_once(' ')
                .expect("should have `Game` and id")
                .1
                .parse::<u32>()
                .expect("should have a game id");

            let impossible_game = rest
                .trim()
                .split(';')
                .map(|hand| {
                    hand.trim()
                        .split(',')
                        .map(|c| {
                            let (num, col) = c
                                .trim()
                                .split_once(' ')
                                .expect("should have number and color");

                            let num = num.parse::<u32>().expect("should be a number");
                            (num, col)
                        })
                        .map(|(num, col)| {
                            if col == "red" {
                                num <= 12
                            } else if col == "green" {
                                num <= 13
                            } else {
                                num <= 14
                            }
                        })
                        .filter(|b| !b)
                        .count()
                })
                .sum::<usize>()
                > 0;

            if impossible_game {
                0
            } else {
                id
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line
                .split_once(':')
                .expect("should have a game id and a game state");

            let mut map: HashMap<&str, u32> = HashMap::new();

            rest.trim().split(';').for_each(|hand| {
                hand.trim().split(',').for_each(|c| {
                    let (num, col) = c
                        .trim()
                        .split_once(' ')
                        .expect("should have number and color");

                    let mut num = num.parse::<u32>().expect("should be a number");
                    map.entry(col)
                        .and_modify(|e| {
                            if e < &mut num {
                                *e = num
                            }
                        })
                        .or_insert(num);
                })
            });
            map.get("red").unwrap() * map.get("green").unwrap() * map.get("blue").unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let res = part1(input);

        assert_eq!(res, 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let res = part2(input);

        assert_eq!(res, 2286);
    }
}
