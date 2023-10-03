use std::fs;
use advent_of_code_rusty_2022_ed::crane::{parse_stacks_and_commands};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/05/sample.txt")?;
    let (stack, commands) = parse_stacks_and_commands(&input).unwrap();
    println!(
        "Solution Part 1: {}",
        1
    );
    Ok(())
}
