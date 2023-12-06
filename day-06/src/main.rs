fn main() {
    let input = include_str!("input.txt");

    let res1 = part_1(input);

    println!("Part 1: {}", res1);
}

fn distance(time_pressed: u32, race_time: u32) -> u32 {
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

    dbg!(&times);
    dbg!(&distances);

    let mut total = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let distance_to_beat = distances[i];

        let mut nbr_beaten = 0;
        for time_pressed in 1..race_time {
            let distance = distance(time_pressed, race_time);
            if distance > distance_to_beat {
                nbr_beaten += 1;
            }
        }
        total = total * nbr_beaten;
    }

    total
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
}
