use std::{env, error::Error, fs};

const MAX_CHARACTERS_ALLOWED: usize = 5;

fn map_word_to_digit(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn find_first_digit(input: &str) -> u32 {
    let mut substring = String::new();
    let mut digit = 0;
    for character in input.chars() {
        if let Some(d) = character.to_digit(10) {
            digit = d;
            break;
        }

        substring.push(character);
    }

    let mut buff = String::new();
    for i in 0..substring.len() {
        for j in 0..MAX_CHARACTERS_ALLOWED {
            if let Some(character) = substring.chars().nth(i + j) {
                buff.push(character);
                if let Some(digit) = map_word_to_digit(&buff) {
                    return digit;
                }
            }
        }

        buff.clear();
    }

    digit
}

fn find_last_digit(input: &str) -> u32 {
    let mut substring = String::new();
    let mut digit = 0;
    for character in input.chars().rev() {
        if let Some(d) = character.to_digit(10) {
            digit = d;
            break;
        }

        substring = format!("{}{}", character, substring);
    }

    if substring.len() < 1 {
        return digit
    }

    let mut buff = String::new();
    for i in (0..=substring.len() - 1).rev() {
        for j in 0..MAX_CHARACTERS_ALLOWED {
            if j > i {
                continue;
            }

            if let Some(character) = substring.chars().nth(i - j) {
                buff = format!("{}{}", character, buff);

                if let Some(digit) = map_word_to_digit(&buff) {
                    return digit;
                }
            }
        }

        buff.clear();
    }

    digit
}

fn find_calibration_values(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let first = find_first_digit(line);
            let last = find_last_digit(line);
            first * 10 + last
        })
        .collect::<Vec<u32>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = fs::read_to_string(&args[1])?;
        let total: u32 = find_calibration_values(&input).iter().sum();

        println!("Total {}", total)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_and_last_digits_when_string_has_random_letters() {
        let input = "1abc2";
        assert_eq!(1, find_first_digit(input));
        assert_eq!(2, find_last_digit(input));

        let input = "pqr3stu8vwx";
        assert_eq!(3, find_first_digit(input));
        assert_eq!(8, find_last_digit(input));

        let input = "a1b2c3d4e5f";
        assert_eq!(1, find_first_digit(input));
        assert_eq!(5, find_last_digit(input));

        let input = "treb7uchet";
        assert_eq!(7, find_first_digit(input));
        assert_eq!(7, find_last_digit(input));
    }

    #[test]
    fn test_find_first_and_last_digits_when_string_has_digits_as_words_and_digits() {
        let input = "two1nine";
        assert_eq!(2, find_first_digit(input));
        assert_eq!(9, find_last_digit(input));

        let input = "eightwothree";
        assert_eq!(8, find_first_digit(input));
        assert_eq!(3, find_last_digit(input));

        let input = "abcone2threexyz";
        assert_eq!(1, find_first_digit(input));
        assert_eq!(3, find_last_digit(input));

        let input = "xtwone3four";
        assert_eq!(2, find_first_digit(input));
        assert_eq!(4, find_last_digit(input));

        let input = "4nineeightseven2";
        assert_eq!(4, find_first_digit(input));
        assert_eq!(2, find_last_digit(input));

        let input = "zoneight234";
        assert_eq!(1, find_first_digit(input));
        assert_eq!(4, find_last_digit(input));

        let input = "7pqrstsixteen";
        assert_eq!(7, find_first_digit(input));
        assert_eq!(6, find_last_digit(input));
    }

    #[test]
    fn test_find_calibration_values_should_return_all_digits_when_a_line_only_contains_digits_and_random_letter(
    ) {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected_sum: u32 = 142;

        let calibration_values = find_calibration_values(input);

        assert_eq!(expected_sum, calibration_values.iter().sum());
    }

    #[test]
    fn test_find_calibration_values_should_return_all_digits_when_a_line_contains_digits_and_digits_encoded_as_words(
    ) {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let expected_sum: u32 = 281;

        let calibration_values = find_calibration_values(input);

        assert_eq!(expected_sum, calibration_values.iter().sum());
    }
}
