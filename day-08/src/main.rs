use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().expect("should have line of instructions");
    let _ = lines.next().expect("should be an empty line");

    let map: HashMap<String, (String, String)> = lines
        .map(|line| {
            let (node, path) = line.split_once(" = ").expect("should have node and path");
            let (left, right) = path
                .split_once(", ")
                .expect("should have a left and right path");
            let left = &left[1..];
            let right = &right[..right.len() - 1];

            (node.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    let mut node = map.get("AAA").expect("should have the AAA node");

    let mut steps = 0;
    for i in instructions.chars().cycle() {
        steps += 1;

        let next_node = if i == 'L' { &node.0 } else { &node.1 };

        if next_node == "ZZZ" {
            break;
        } else {
            node = map.get(next_node).expect("should have the next node");
        }
    }

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
