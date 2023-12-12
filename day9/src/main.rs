use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args[1..].len() == 2 {
        let command = &args[1];
        let input = fs::read_to_string(&args[2])?;

        let total: i64 = match command.as_str() {
            "part1" => 0,
            "part2" => 0,
            _ => 0,
        };

        println!("total {}", total);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
