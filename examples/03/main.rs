use std::fs;
use advent_of_code_rusty_2022_ed::{total_duplicate_priorities, total_badge_priorities};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("examples/03/input.txt")?;
    let sum = total_duplicate_priorities(input.as_str());
    println!(
        "Solution Part 1: {}",
        sum
    );
    println!(
        "Solution Part 2: {}",
        total_badge_priorities(input.as_str())
    );
    Ok(())
}
