fn main() {
    let input = include_str!("input.txt");

    let res1 = part_1(input);
    let res2 = part_2(input);

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn distance(time_pressed: u64, race_time: u64) -> u64 {
    time_pressed * (race_time - time_pressed)
}

fn part_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = lines.next().expect("should have a line of times");
    let distances = lines.next().expect("should have a line of distances");
    drop(lines);

    let times: Vec<u32> = times
        .split_once("Time:")
        .expect("should have time")
        .1
        .trim()
        .split_whitespace()
        .map(|t| t.parse().expect("should be a number"))
        .collect();
    let distances: Vec<u32> = distances
        .split_once("Distance:")
        .expect("should have distance")
        .1
        .trim()
        .split_whitespace()
        .map(|t| t.parse().expect("should be a number"))
        .collect();

    let mut total = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let distance_to_beat = distances[i];

        let mut nbr_beaten = 0;
        for time_pressed in 1..race_time {
            let distance = distance(time_pressed as u64, race_time as u64);
            if distance > distance_to_beat as u64 {
                nbr_beaten += 1;
            }
        }
        total = total * nbr_beaten;
    }

    total
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time_row = lines.next().expect("should have a line of times");
    let distance_row = lines.next().expect("should have a line of distances");
    drop(lines);

    let race_time: u64 = time_row
        .split_once("Time:")
        .expect("should have time")
        .1
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("should be a number");

    let distance_to_beat: u64 = distance_row
        .split_once("Distance:")
        .expect("should have distance")
        .1
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("should be a number");

    let mut nbr_beaten = 0;
    for time_pressed in 1..race_time {
        let distance = distance(time_pressed, race_time);
        if distance > distance_to_beat {
            nbr_beaten += 1;
        }
    }
    nbr_beaten
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let res = part_1(input);

        assert_eq!(res, 288);
    }

    #[test]
    fn test_part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let res = part_2(input);

        assert_eq!(res, 71503);
    }
}
