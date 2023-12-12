use std::{collections::HashSet, env, error::Error, fs};

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn to_number(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn concat_numbers(main_part: u32, other_part: u32) -> u32 {
    main_part * 10 + other_part
}

fn get_winning_and_my_numbers(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let colon_index = line.find(':').unwrap();
    let mut winning_numbers: HashSet<u32> = HashSet::new();

    let mut limit_character_index: Option<usize> = None;

    // parse winning numbers
    let mut number: Option<u32> = None;
    for (index, character) in line[colon_index..].chars().enumerate() {
        if character == '|' {
            limit_character_index = Some(index + colon_index);
            break;
        }

        if is_number(character) && number.is_none() {
            number = Some(to_number(character))
        } else if is_number(character) && number.is_some() {
            number = Some(concat_numbers(number.unwrap(), to_number(character)));
        }

        if !is_number(character) && number.is_some() {
            winning_numbers.insert(number.unwrap());
            number = None;
        }
    }

    // parse my numbers
    let mut my_numbers: HashSet<u32> = HashSet::new();
    let unparsed_numbers = &line[limit_character_index.unwrap() + 1..];
    let mut number: Option<u32> = None;
    for (index, character) in unparsed_numbers.chars().enumerate() {
        if is_number(character) && number.is_none() {
            number = Some(to_number(character));
        } else if is_number(character) && number.is_some() {
            number = Some(concat_numbers(number.unwrap(), to_number(character)));
        }

        if (!is_number(character) || index == unparsed_numbers.len() - 1) && number.is_some() {
            my_numbers.insert(number.unwrap());
            number = None;
        }
    }

    (winning_numbers, my_numbers)
}

fn get_matched_numbers(line: &str) -> HashSet<u32> {
    let (winning_numbers, my_numbers) = get_winning_and_my_numbers(line);
    winning_numbers
        .intersection(&my_numbers)
        .cloned()
        .collect::<HashSet<_>>()
}

fn get_points_the_cards_woth(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let mut points = 0;
            for _ in get_matched_numbers(line) {
                if points == 0 {
                    points += 1
                } else {
                    points <<= 1
                }
            }

            points
        })
        .collect::<Vec<u32>>()
}

#[derive(Clone, Debug)]
struct Card {
    matched_numbers: HashSet<u32>,
    id: i32,
    ref_count: u32,
}

fn get_total_scratchcards(input: &str) -> u32 {
    let mut cards = input
        .lines()
        .enumerate()
        .map(|(index, line)| Card {
            id: index as i32 + 1,
            ref_count: 1,
            matched_numbers: get_matched_numbers(line),
        })
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        let card = cards[i].clone();
        let total_matches = card.matched_numbers.len() as i32;

        for _ in 0..card.ref_count {
            for i in card.id + 0..card.id + total_matches {
                cards[i as usize].ref_count += 1;
            }
        }
    }

    cards.iter().map(|card| card.ref_count).sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total = match command.as_str() {
            "part1" => get_points_the_cards_woth(&input).iter().sum::<u32>(),
            "part2" => get_total_scratchcards(&input),
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
    fn test_get_points_the_cards_woth() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let points = get_points_the_cards_woth(&input);

        assert_eq!(13, points.iter().sum::<u32>())
    }

    #[test]
    fn test_get_total_scratchcards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let points = get_total_scratchcards(&input);

        assert_eq!(30, points)
    }
}
