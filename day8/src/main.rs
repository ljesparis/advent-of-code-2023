use std::{collections::HashMap, env, error::Error, fs};

#[derive(Debug, Clone, Copy)]
enum Directions {
    LEFT = 0,
    RIGHT = 1,
}

impl Directions {
    fn new(character: char) -> Self {
        if character == 'L' {
            Directions::LEFT
        } else {
            Directions::RIGHT
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Directions> {
    input
        .chars()
        .rev()
        .map(|character| Directions::new(character))
        .collect::<Vec<_>>()
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let mut content = line.split('=');
            let key = content.nth(0).unwrap().trim_end();
            let value = content.nth(0).unwrap().split(',').map(|el| {
                if el.contains('(') {
                    el.split('(').nth(1).unwrap()
                } else {
                    el.split(')').nth(0).unwrap().trim()
                }
            });

            (key, value.collect::<Vec<&str>>())
        })
        .collect::<HashMap<&str, Vec<&str>>>()
}

fn calculate_steps_part1(input: &str) -> i64 {
    let mut content = input.split("\n\n");

    let instructions = content
        .nth(0)
        .unwrap_or_else(|| panic!("content is empty, please check out your input"));
    let instructions = parse_instructions(instructions);

    let graph = content
        .nth(0)
        .unwrap_or_else(|| panic!("content is probably empty, please checkout your input"));
    let graph = parse_graph(graph);

    let mut steps = 0;
    let mut stack: Vec<Directions> = instructions.clone();
    let mut adjacent_nodes = graph.get("AAA").unwrap();

    'outer_loop: loop {
        steps += 1;
        let instruction = stack.pop().unwrap();
        let node = adjacent_nodes[instruction as usize];

        match node {
            "ZZZ" => break 'outer_loop,
            _ => {
                adjacent_nodes = graph.get(node).unwrap();

                if stack.len() == 0 {
                    stack = instructions.clone();
                }
            }
        }
    }

    steps
}

fn calculate_steps_part2(input: &str) -> i64 {
    let mut content = input.split("\n\n");

    let instructions = content
        .nth(0)
        .unwrap_or_else(|| panic!("content is empty, please check out your input"));
    let instructions = parse_instructions(instructions);

    let graph = content
        .nth(0)
        .unwrap_or_else(|| panic!("content is probably empty, please checkout your input"));
    let graph = parse_graph(graph);

    let all_nodes_that_ends_in_a = graph
        .keys()
        .filter(|&&node_key| node_key.ends_with('A'))
        .collect::<Vec<_>>();

    let mut stack: Vec<Directions> = instructions.clone();
    let mut steps_per_node: Vec<usize> = vec![];

    for &node in all_nodes_that_ends_in_a {
        let mut steps: usize = 0;
        let mut adjacent_nodes = graph.get(node).unwrap();

        'outer_loop: loop {
            steps += 1;
            let instruction = stack.pop().unwrap();
            let node = adjacent_nodes[instruction as usize];

            if node.ends_with('Z') {
                break 'outer_loop;
            } else {
                adjacent_nodes = graph.get(node).unwrap();

                if stack.len() == 0 {
                    stack = instructions.clone();
                }
            }
        }

        steps_per_node.push(steps);
        stack = instructions.clone();
    }

    lcm(steps_per_node.as_slice()) as i64
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total: i64 = match command.as_str() {
            "part1" => calculate_steps_part1(&input),
            "part2" => calculate_steps_part2(&input),
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
    fn test_calculate_steps_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, calculate_steps_part1(input));
    }

    #[test]
    fn test_calculate_steps_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(6, calculate_steps_part2(input));
    }
}
