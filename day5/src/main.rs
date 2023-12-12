use std::{collections::HashMap, env, error::Error, fs, ops::Range};

#[derive(Debug)]
struct RangeMap {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Debug)]
struct AlmanacMap<'a> {
    from: &'a str,
    to: &'a str,
    range_maps: Vec<RangeMap>,
}

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

fn parse_map_header(unparsed_header: &str) -> (&str, &str) {
    let header = unparsed_header
        .split(' ')
        .nth(0)
        .unwrap()
        .split("-to-")
        .collect::<Vec<&str>>();

    (header[0], header[1])
}

fn parse_map_range_numbers(unparsed_range_numbers: &[&str]) -> Vec<RangeMap> {
    let mut range_maps: Vec<RangeMap> = vec![];
    for &unparsed_range in unparsed_range_numbers {
        let numbers = parse_numbers(unparsed_range);
        range_maps.push(RangeMap {
            source: numbers[1],
            destination: numbers[0],
            length: numbers[2],
        });
    }
    range_maps
}

fn parse_maps<'a>(input: &[&'a str]) -> HashMap<&'a str, AlmanacMap<'a>> {
    let mut maps: HashMap<&'a str, AlmanacMap<'_>> = HashMap::new();
    for map_as_str in input.iter() {
        let content = map_as_str
            .split('\n')
            .filter(|&l| l.len() > 0)
            .collect::<Vec<&str>>();

        let (from, to) = parse_map_header(content[0]);
        let mut range_maps = parse_map_range_numbers(&content[1..]);

        maps.insert(
            from,
            AlmanacMap {
                from,
                to,
                range_maps,
            },
        );
    }

    maps
}

fn walk(n: u64, maps: &HashMap<&'_ str, AlmanacMap<'_>>, from: &str) -> u64 {
    match maps.get(from) {
        Some(map) => {
            if let Some(range) = map
                .range_maps
                .iter()
                .find(|&range| range.source <= n && n < range.source + range.length)
            {
                let diff = if range.source < n {
                    n - range.source
                } else {
                    range.source - n
                };
                walk(range.destination + diff, maps, map.to)
            } else {
                walk(n, maps, map.to)
            }
        }
        _ => n,
    }
}

fn find_lowest_location_number(input: &str) -> u64 {
    let content = input
        .split("\n\n")
        .filter(|&l| l.len() > 0)
        .collect::<Vec<&str>>();
    let seeds = parse_numbers(&content[0][6..]);
    let maps = parse_maps(&content[1..]);

    let mut min_loc: Option<u64> = None;
    for &seed in seeds.iter() {
        let loc = walk(seed, &maps, "seed");
        if let Some(mloc) = min_loc {
            min_loc = Some(if mloc < loc { mloc } else { loc })
        } else {
            min_loc = Some(loc)
        }
    }

    min_loc.unwrap()
}

fn find_lowest_location_number_2(input: &str) -> u64 {
    let content = input
        .split("\n\n")
        .filter(|&l| l.len() > 0)
        .collect::<Vec<&str>>();
    let seed_ranges = parse_numbers(&content[0][6..])
        .chunks(2)
        .map(|arr| {
            let rstart = arr[0];
            let rend = arr[1] + rstart;
            (rstart, rend)
        })
        .collect::<Vec<(u64, u64)>>();

    let maps = parse_maps(&content[1..]);
    let mut min_loc: Option<u64> = None;
    let mut last_range: Option<Range<u64>> = None;

    for (seed_start, seed_end) in seed_ranges {

        let range = match &last_range {
            Some(last_range) => {
                if (last_range.start > seed_end ) || (last_range.end < seed_start){
                    seed_start..seed_end
                } else if last_range.end > seed_start {
                    last_range.end..seed_end
                }else if last_range.start < seed_end { 
                    seed_start..last_range.start
                } else {
                    seed_start..seed_end
                }
            },
            _ => seed_start..seed_end,
        };

        last_range = match &last_range {
            Some(last_range) => {
                if last_range.start > seed_end  {
                    Some(seed_start..last_range.end)
                } else if last_range.end < seed_start{
                    Some(last_range.start..seed_end)
                } else if last_range.end > seed_start {
                    Some(last_range.start..seed_end)
                }else if last_range.start < seed_end { 
                    Some(seed_start..last_range.start)
                } else {
                    Some(seed_start..seed_end)
                }
            },
            _ => Some(seed_start..seed_end),
        };

        println!("seed start {}, seed end {}", range.start, range.end);

        for seed in range {
            let loc = walk(seed, &maps, "seed");
            if let Some(mloc) = min_loc {
                min_loc = Some(if mloc < loc { mloc } else { loc });
            } else {
                min_loc = Some(loc);
            }
        }
    }

    min_loc.unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total = match command.as_str() {
            "part1" => find_lowest_location_number(&input),
            "part2" => find_lowest_location_number_2(&input),
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
    fn test_find_lowest_location_number() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let lowest_location_number = find_lowest_location_number(input);

        assert_eq!(35, lowest_location_number)
    }

    #[test]
    fn test_find_lowest_location_number_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let lowest_location_number = find_lowest_location_number_2(input);

        assert_eq!(46, lowest_location_number)
    }
}
