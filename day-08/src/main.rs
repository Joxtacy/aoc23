use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        char('L').map(|_| Direction::Left),
        char('R').map(|_| Direction::Right),
    )))(input)?;

    let (input, _) = multispace1(input)?;

    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        HashMap::new,
        |mut acc: HashMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

fn part_1(input: &str) -> usize {
    let (_, (instructions, map)) = parse(input).expect("should parse without error");

    let mut current_node = "AAA";
    let steps = instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(index, instruction)| {
            let node = map.get(current_node).expect("should have the next node");

            current_node = match instruction {
                Direction::Left => &node.0,
                Direction::Right => &node.1,
            };

            if current_node == "ZZZ" {
                Some(index + 1)
            } else {
                None
            }
        })
        .expect("should have found a result");

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let res = part_1(input);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let res = part_1(input);

        assert_eq!(res, 6);
    }
}
