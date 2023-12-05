fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').expect("should have a colon");

            let (winning_numbers, numbers_i_have) = numbers
                .trim()
                .split_once('|')
                .expect("should have winning numbers and numbers i have");

            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().expect("should be a number"))
                .collect::<Vec<_>>();

            let numbers_i_have = numbers_i_have
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().expect("should be a number"))
                .collect::<Vec<_>>();

            let mut winners: u32 = 0;

            for number in winning_numbers {
                if numbers_i_have.contains(&number) {
                    if winners == 0 {
                        winners = 1;
                    } else {
                        winners *= 2;
                    }
                }
            }

            winners
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut cards = vec![1; input.lines().count()];

    input.lines().enumerate().for_each(|(index, line)| {
        let (_, numbers) = line.split_once(':').expect("should have a colon");

        let (winning_numbers, numbers_i_have) = numbers
            .trim()
            .split_once('|')
            .expect("should have winning numbers and numbers i have");

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().expect("should be a number"))
            .collect::<Vec<_>>();

        let numbers_i_have = numbers_i_have
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().expect("should be a number"))
            .collect::<Vec<_>>();

        let mut winners: u32 = 0;

        for number in winning_numbers {
            if numbers_i_have.contains(&number) {
                winners += 1;
            }
        }

        for i in 1..=winners {
            let number_of_cards = cards[index];
            cards[index + i as usize] += number_of_cards;
        }
    });

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = part_1(input);

        assert_eq!(res, 13);
    }

    #[test]
    fn test_part_2() {
        let input = "card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = part_2(input);

        assert_eq!(res, 30);
    }
}
