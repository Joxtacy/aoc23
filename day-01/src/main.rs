use std::iter::from_fn;

fn main() {
    let input = include_str!("./input.txt");

    let res1 = part1(input);
    let res2 = part2(input);

    println!("part1: {}", res1);
    println!("part2: {}", res2);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
        let first = numbers.next().expect("should be a number");
        let x = match numbers.last() {
            Some(last) => {
                format!("{}{}", first, last)
            }
            None => {
                format!("{}{}", first, first)
            }
        }
        .parse::<u32>()
        .expect("should be a valid number");
        sum += x;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut index = 0;
        let line_iter = from_fn(move || {
            let reduced_line = &line[index..];

            let result = if reduced_line.starts_with("one") {
                // adding to the index with the length of the word did not work. Not sure why.
                // Seems like skipping can cause errors when you have a string like `twone` at the
                // end, `one` is correct but the skipping would give `two`.
                //index += "one".len();
                Some('1')
            } else if reduced_line.starts_with("two") {
                //index += "two".len();
                Some('2')
            } else if reduced_line.starts_with("three") {
                //index += "three".len();
                Some('3')
            } else if reduced_line.starts_with("four") {
                //index += "four".len();
                Some('4')
            } else if reduced_line.starts_with("five") {
                //index += "five".len();
                Some('5')
            } else if reduced_line.starts_with("six") {
                //index += "six".len();
                Some('6')
            } else if reduced_line.starts_with("seven") {
                //index += "seven".len();
                Some('7')
            } else if reduced_line.starts_with("eight") {
                //index += "eight".len();
                Some('8')
            } else if reduced_line.starts_with("nine") {
                //index += "nine".len();
                Some('9')
            } else {
                //index += 1;
                reduced_line.chars().next()
            };

            index += 1;

            result
        });
        let numbers = line_iter
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        if numbers.len() >= 2 {
            let x = format!("{}{}", numbers[0], numbers.last().unwrap());
            sum += x.parse::<u32>().unwrap();
        } else if numbers.len() == 1 {
            let x = format!("{}{}", numbers[0], numbers[0]);
            sum += x.parse::<u32>().unwrap();
        }
    }
    sum
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
