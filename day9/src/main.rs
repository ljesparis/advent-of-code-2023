use std::{env, error::Error, fs};

fn parse_numbers(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.split(' ').map(|n| {
        if n.starts_with('-') {
            let negative_number = &n[1..];
            negative_number.parse::<i64>().unwrap() * -1
        } else {
            n.parse::<i64>().unwrap()
        }
    })
}

fn get_total_extrapolated_values(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut sequence = parse_numbers(line).collect::<Vec<i64>>();
            let mut sequences = Vec::new();
            sequences.push(sequence.clone());

            loop {
                sequence = sequence
                    .windows(2)
                    .map(|chunk| chunk[1] - chunk[0])
                    .collect::<Vec<i64>>();

                sequences.push(sequence.clone());
                if sequence.iter().all(|&el| el == 0) {
                    break;
                }
            }

            sequences
                .iter()
                .map(|sequence| sequence[sequence.len() - 1] as i64)
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn get_total_extrapolated_values_backwards(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut sequence = parse_numbers(line).collect::<Vec<i64>>();
            let mut sequences = Vec::new();
            sequences.push(sequence.clone());

            loop {
                sequence = sequence
                    .windows(2)
                    .map(|chunk| chunk[1] - chunk[0])
                    .collect::<Vec<i64>>();

                sequences.push(sequence.clone());
                if sequence.iter().all(|&el| el == 0) {
                    break;
                }
            }

            let mut result: i64 = 0;
            for el in sequences.iter().rev().map(|sequence| sequence[0]) {
                result = el - result;
            }
            
            result
        })
        .sum::<i64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total: i64 = match command.as_str() {
            "part1" => get_total_extrapolated_values(&input),
            "part2" => get_total_extrapolated_values_backwards(&input),
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
    fn test_get_total_extrapolated_values() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(114, get_total_extrapolated_values(&input));
    }

    #[test]
    fn test_get_total_extrapolated_values_backwards() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(2, get_total_extrapolated_values_backwards(&input));
    }
}
