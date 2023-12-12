use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs,
};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Hash, PartialEq, Eq, Debug)]
struct Point {
    y: usize,
    x_start: usize,
    x_end: Option<usize>,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Symbol {
    value: char,
    point: Point,
}

#[derive(Debug)]
struct Number {
    value: u32,
    point: Point,
}

impl Number {
    fn new(v: u32, x_start: usize, y: usize) -> Self {
        Self {
            value: v,
            point: Point {
                y,
                x_start,
                x_end: None,
            },
        }
    }

    fn concat_digit(&mut self, n: u32) {
        self.value = self.value * 10 + n
    }

    fn get_adjacent_symbols(
        &self,
        schematic: &Vec<Vec<char>>,
        schematic_dimensions: (usize, usize),
    ) -> HashSet<Symbol> {
        let (schematic_y_dimension, schematic_x_dimension) = schematic_dimensions;
        let number_x_start = self.point.x_start as i32;
        let number_x_end = self.point.x_end.unwrap() as i32 + 1;

        let mut adjacent_symbols: HashSet<Symbol> = HashSet::new();
       
        for tmp_x in number_x_start..number_x_end {
            for (dy, dx) in DIRECTIONS {
                let y = dy + self.point.y as i32;
                let x = tmp_x + dx;

                if y < 0 || y >= schematic_y_dimension as i32 {
                    continue;
                } else if x < 0 || x >= schematic_x_dimension as i32 {
                    continue;
                }

                let chararacter = schematic[y as usize][x as usize];
                if is_symbol(chararacter) {
                    adjacent_symbols.insert(Symbol {
                        value: chararacter,
                        point: Point {
                            y: y as usize,
                            x_start: x as usize,
                            x_end: Some(x as usize),
                        },
                    });
                }
            }
        }

        adjacent_symbols
    }
}

fn is_dot(c: char) -> bool {
    return c == '.';
}

fn is_digit(c: char) -> bool {
    return c.is_digit(10);
}

fn is_symbol(c: char) -> bool {
    !is_dot(c) && !is_digit(c)
}

fn to_digit(c: char) -> u32 {
    return c.to_digit(10).unwrap();
}

fn generate_schematic(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_schematic_dimensions(schematic: &Vec<Vec<char>>) -> (usize, usize) {
    (schematic.len(), schematic[0].len())
}

fn get_parsed_numbers(
    schematic: &Vec<Vec<char>>,
    schematic_dimensions: (usize, usize),
) -> Vec<Number> {
    let (y_len, x_len) = schematic_dimensions;
    let mut parsed_numbers: Vec<Number> = vec![];
    for y in 0..y_len {
        let mut number: Option<Number> = None;
        for x in 0..x_len {
            let chararacter = schematic[y][x];

            if is_digit(chararacter) && number.is_none() {
                number = Some(Number::new(to_digit(chararacter), x, y));
            } else if is_digit(chararacter) && number.is_some() {
                let mut tmp = number.unwrap();
                tmp.concat_digit(to_digit(chararacter));
                number = Some(tmp)
            }

            // x == x_len -1 means that we reach eof :S
            if (is_dot(chararacter) || is_symbol(chararacter) || x == x_len - 1) && number.is_some()
            {
                let mut tmp = number.unwrap();
                tmp.point.x_end = Some(if x == x_len - 1 { x } else { x - 1 });
                parsed_numbers.push(tmp);
                number = None
            }
        }
    }

    parsed_numbers
}

fn find_adjacent_numbers_to_symbol(input: &str) -> Vec<u32> {
    let schematic = generate_schematic(input); // generating the matrix
    let schematic_dimensions = get_schematic_dimensions(&schematic);
    let mut numbers_adjacent_to_symbols: Vec<u32> = vec![];

    let numbers = get_parsed_numbers(&schematic, schematic_dimensions);
    for number in numbers {
        let adjacent_symbols = number.get_adjacent_symbols(&schematic, schematic_dimensions);
        if adjacent_symbols.len() > 0 {
            numbers_adjacent_to_symbols.push(number.value);
        }
    }

    numbers_adjacent_to_symbols
}

fn find_gear_ratios(input: &str) -> Vec<u32> {
    let schematic = generate_schematic(input); // generating the matrix
    let schematic_dimensions = get_schematic_dimensions(&schematic);
    let numbers = get_parsed_numbers(&schematic, schematic_dimensions);

    let mut gear_ratios_by_start_symbol: HashMap<Symbol, Vec<u32>> = HashMap::new();

    for number in numbers {
        let adjacent_symbols = number.get_adjacent_symbols(&schematic, schematic_dimensions);
        for symbol in adjacent_symbols {
            if symbol.value != '*' {
                continue;
            

            (*gear_ratios_by_start_symbol
                .entry(symbol)
                .or_insert(Vec::new()))
            .push(number.value);
        }
    }

    gear_ratios_by_start_symbol
        .values()
        .filter_map(|vec| {
            if vec.len() > 1 {
                Some(vec.iter().product())
            } else {
                None
            }
        })
        .collect::<Vec<u32>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total = match command.as_str() {
            "part1" => find_adjacent_numbers_to_symbol(&input).iter().sum::<u32>(),
            "part2" => find_gear_ratios(&input).iter().sum::<u32>(),
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
    fn test_find_adjacent_numbers_to_symbol() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let adjacent_numbers_to_symbol = find_adjacent_numbers_to_symbol(input);

        assert_eq!(467, adjacent_numbers_to_symbol[0]);
        assert_eq!(35, adjacent_numbers_to_symbol[1]);
        assert_eq!(633, adjacent_numbers_to_symbol[2]);
        assert_eq!(617, adjacent_numbers_to_symbol[3]);
        assert_eq!(592, adjacent_numbers_to_symbol[4]);
        assert_eq!(755, adjacent_numbers_to_symbol[5]);
        assert_eq!(664, adjacent_numbers_to_symbol[6]);
        assert_eq!(598, adjacent_numbers_to_symbol[7]);
        assert_eq!(4361, adjacent_numbers_to_symbol.iter().sum::<u32>());
    }

    #[test]
    fn test_find_gear_ratios() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        let adjacent_numbers_to_symbol = find_gear_ratios(input);

        assert_eq!(467835, adjacent_numbers_to_symbol.iter().sum::<u32>());
    }
}
