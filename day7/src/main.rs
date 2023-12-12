use std::{collections::HashMap, env, error::Error, fs};

fn parse_number_u32(n: &str) -> u32 {
    n.parse::<u32>().unwrap()
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

impl<'a> Hand<'a> {
    fn compute_hand_weight(&self) -> i8 {
        let frecuencies = self.get_frecuencies(self.get_letters_frecuency());
        self.map_hand_weight(frecuencies)
    }

    fn compute_hand_weight_with_joker(&self) -> i8 {
        let mut letters_frecuency = self.get_letters_frecuency();
        let extra_val = match letters_frecuency.remove_entry(&'J') {
            Some((_, value)) => value,
            _ => 0,
        };

        let mut frecuencies = self.get_frecuencies(letters_frecuency);
        if frecuencies.len() > 0 {
            frecuencies[0] += extra_val;
        } else {
            frecuencies.push(extra_val);
        }

        self.map_hand_weight(frecuencies)
    }

    fn get_frecuencies(&self, repeated_letters_map: HashMap<char, i8>) -> Vec<i8> {
        let mut repeated_letters = repeated_letters_map
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<i8>>();
        repeated_letters.sort_by(|val_a, val_b| val_b.cmp(val_a));

        repeated_letters
    }

    fn get_letters_frecuency(&self) -> HashMap<char, i8> {
        let mut repeated_letters: HashMap<char, i8> = HashMap::new();
        for character in self.cards.chars() {
            (*repeated_letters.entry(character).or_insert(0)) += 1;
        }

        repeated_letters
    }

    fn map_hand_weight(&self, repeated_letters: Vec<i8>) -> i8 {
        match repeated_letters.len() {
            1 => 7,
            2 => repeated_letters[0] + 2,
            3 => repeated_letters[0] + 1,
            4 => 2,
            _ => 1,
        }
    }
}

fn map_letter_weight(c: char) -> i8 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        _ => 1,
    }
}

fn map_letter_weight_joker(c: char) -> i8 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 1,
    }
}

fn parse_hands<'a>(input: &str) -> impl Iterator<Item = Hand<'_>> {
    input.lines().map(|line| {
        let content = line
            .split(' ')
            .filter(|&el| el.len() > 0)
            .collect::<Vec<&str>>();

        Hand {
            cards: content[0],
            bid: parse_number_u32(content[1]),
        }
    })
}

fn get_total_winning_hands(input: &str) -> u32 {
    let mut hands = parse_hands(input).collect::<Vec<Hand<'_>>>();
    hands.sort_by(|hand_a, hand_b| {
        let hand_a_weight = hand_a.compute_hand_weight();
        let hand_b_weight = hand_b.compute_hand_weight();
        if hand_a_weight == hand_b_weight {
            let hand_a_iter = hand_a.cards.chars().into_iter();
            let hand_b_iter = hand_b.cards.chars().into_iter();

            let mut both_hands_iter = hand_a_iter.zip(hand_b_iter);
            loop {
                let (card_a, card_b) = both_hands_iter.next().unwrap();

                let card_a_weight = map_letter_weight(card_a);
                let card_b_weight = map_letter_weight(card_b);

                if card_a_weight == card_b_weight {
                    continue;
                }

                return card_a_weight.cmp(&card_b_weight);
            }
        } else {
            hand_a_weight.cmp(&hand_b_weight)
        }
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn get_total_winning_hands_part2(input: &str) -> u32 {
    let mut hands = parse_hands(input).collect::<Vec<Hand<'_>>>();
    hands.sort_by(|hand_a, hand_b| {
        let hand_a_weight = hand_a.compute_hand_weight_with_joker();
        let hand_b_weight = hand_b.compute_hand_weight_with_joker();
        if hand_a_weight == hand_b_weight {
            let hand_a_iter = hand_a.cards.chars().into_iter();
            let hand_b_iter = hand_b.cards.chars().into_iter();

            let mut both_hands_iter = hand_a_iter.zip(hand_b_iter);
            loop {
                let (card_a, card_b) = both_hands_iter.next().unwrap();

                let card_a_weight = map_letter_weight_joker(card_a);
                let card_b_weight = map_letter_weight_joker(card_b);

                if card_a_weight == card_b_weight {
                    continue;
                }

                return card_a_weight.cmp(&card_b_weight);
            }
        } else {
            hand_a_weight.cmp(&hand_b_weight)
        }
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total = match command.as_str() {
            "part1" => get_total_winning_hands(&input),
            "part2" => get_total_winning_hands_part2(&input),
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
    fn test_get_total_winnings() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, get_total_winning_hands(&input))
    }

    #[test]
    fn test_get_total_winnings_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(5905, get_total_winning_hands_part2(&input))
    }
}
