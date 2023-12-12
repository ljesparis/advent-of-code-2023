use std::{env, error::Error, fs, collections::HashMap};

#[derive(PartialEq, Eq, Hash, Debug)]
enum Color {
    RED,
    BLUE,
    GREEN,
}

impl Color {
    fn map_str_to_color(color_as_str: &str) -> Option<Self> {
        match color_as_str {
            "red" => Some(Color::RED),
            "blue" => Some(Color::BLUE),
            "green" => Some(Color::GREEN),
            _ => None,
        }
    }

    fn is_valid(&self, digit: u32) -> bool {
        match &self {
            Color::RED => digit <= 12,
            Color::BLUE => digit <= 14,
            Color::GREEN => digit <= 13,
        }
    }
}

struct Game {
    id: u32,
    end_offset: usize,
}

impl Game {
    fn parse_game(line: &str) -> Self {
        let mut end_offset: Option<usize> = None;
        let mut game_id_as_string = String::new();
        for (index, character) in line.char_indices() {
            if character.is_digit(10) {
                game_id_as_string.push(character);
            } else if character == ':' {
                end_offset = Some(index);
                break;
            }
        }

        /*
         * All games are going to be the same
         * and for that reason, the unwrap is never going to fail
         * */
        let id = game_id_as_string.parse::<u32>().unwrap();

        Game {
            id,
            end_offset: end_offset.unwrap(),
        }
    }
}

fn find_possible_games(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|line| {
            // find game id
            let game = Game::parse_game(line);

            let mut substring = String::new();
            let line_with_only_colors = &line[game.end_offset + 2..];
            for (i, character) in line_with_only_colors.char_indices() {
                substring.push(character);
                if character == ';' || i == line_with_only_colors.len() - 1 {
                    let mut digit: u32 = 0;
                    let mut digit_as_str = String::new();
                    let mut color_as_str = String::new();
                    for substring_character in substring.clone().chars() {
                        if substring_character.is_digit(10) {
                            digit_as_str.push(substring_character);
                        }

                        if substring_character.is_alphabetic() {
                            digit = digit_as_str.parse::<u32>().unwrap();
                            color_as_str.push(substring_character);

                            if let Some(color) = Color::map_str_to_color(&color_as_str) {
                                if !color.is_valid(digit) {
                                    return None;
                                }

                                color_as_str.clear();
                                digit_as_str.clear();
                            }
                        }
                    }

                    substring.clear();
                }
            }

            Some(game.id)
        })
        .collect::<Vec<u32>>()
}

fn find_minimum_sets(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|line| {
            // find game id
            let game = Game::parse_game(line);

            let mut color_map: HashMap<Color, u32> = HashMap::new();
            let mut substring = String::new();
            let line_with_only_colors = &line[game.end_offset + 2..];
            for (i, character) in line_with_only_colors.char_indices() {
                substring.push(character);
                if character == ';' || i == line_with_only_colors.len() - 1 {
                    let mut digit: u32 = 0;
                    let mut digit_as_str = String::new();
                    let mut color_as_str = String::new();
                    for substring_character in substring.clone().chars() {
                        if substring_character.is_digit(10) {
                            digit_as_str.push(substring_character);
                        }

                        if substring_character.is_alphabetic() {
                            digit = digit_as_str.parse::<u32>().unwrap();
                            color_as_str.push(substring_character);

                            if let Some(color) = Color::map_str_to_color(&color_as_str) {
                                if let Some(value) = color_map.get(&color) {
                                    let max_value = (*value).max(digit);
                                    color_map.insert(color, max_value);
                                } else {
                                    color_map.insert(color, digit);
                                }

                                color_as_str.clear();
                                digit_as_str.clear();
                            }
                        }
                    }

                    substring.clear();
                }
            }

            
            Some(color_map.values().product())
        })
        .collect::<Vec<u32>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = fs::read_to_string(&args[1])?;
        // let possible_games = find_possible_games(&input); // part 1
        let minimum_sets = find_minimum_sets(&input);

        println!("total {}", minimum_sets.iter().sum::<u32>());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_games_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let possible_games = find_possible_games(input);

        assert_eq!(1, possible_games[0]);
        assert_eq!(2, possible_games[1]);
        assert_eq!(5, possible_games[2]);
        assert_eq!(8, possible_games.iter().sum::<u32>());
    }

    #[test]
    fn test_get_possible_games_part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let possible_games = find_minimum_sets(input);

        assert_eq!(2286, possible_games.iter().sum::<u32>());
    }
}
