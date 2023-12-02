fn main() {
    let input = include_str!("./input.txt");

    let res1 = part1(input);
    let res2 = part2(input);

    println!("part1: {}", res1);
    println!("part2: {}", res2);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
            let first = numbers.next().expect("should be a number");
            match numbers.last() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];

        if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().unwrap().to_digit(10)
        }
    });
    let first = it.next().expect("should be a number");
    match it.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = part2(input);

        assert_eq!(result, 281);
    }
}
