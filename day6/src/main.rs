use std::{env, error::Error, fs};

fn parse_number(n: &str) -> u64 {
    n.parse::<u64>().unwrap()
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .filter(|&el| el.len() > 0)
        .map(|el| parse_number(el))
        .collect::<Vec<u64>>()
}

fn bs(time: u64, distance: u64) -> u64 {
    let (mut lo, mut hi): (u64, u64) = (0, time);

    loop {
        let lo_milimeters_per_ms = lo * (time - lo);
        let hi_milimeters_per_ms = hi * (time - hi);

        if lo_milimeters_per_ms > distance && hi_milimeters_per_ms > distance {
            break
        }

        if lo_milimeters_per_ms <= distance {
            lo += 1;
        } else if hi_milimeters_per_ms <= distance {
            hi -= 1;
        }
    }

    (lo..=hi).count() as u64
}

fn find_number_of_ways_to_beat_record_part1(input: &str) -> u64 {
    let content = input.split('\n').collect::<Vec<&str>>();
    let times = parse_numbers(content[0].split(':').nth(1).unwrap());
    let distances = parse_numbers(content[1].split(':').nth(1).unwrap());

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| bs(time, distance))
        .product()
}

fn parse_complete_number(input: &str) -> u64 {
    parse_number(
        &input
            .split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|&el| el.len() > 0)
            .collect::<Vec<&str>>()
            .join(""),
    )
}

fn find_number_of_ways_to_beat_record_part2(input: &str) -> u64 {
    let content = input.split('\n').collect::<Vec<&str>>();
    let time = parse_complete_number(content[0]);
    let distance = parse_complete_number(content[1]);

    bs(time, distance)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total = match command.as_str() {
            "part1" => find_number_of_ways_to_beat_record_part1(&input),
            "part2" => find_number_of_ways_to_beat_record_part2(&input),
            _ => 0,
        };

        println!("total {}", total);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number_of_ways_to_beat_record_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, find_number_of_ways_to_beat_record_part1(input));
    }

    #[test]
    fn test_find_number_of_ways_to_beat_record_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, find_number_of_ways_to_beat_record_part2(input));
    }
}
