use std::fs;
use advent_of_code_rusty_2022_ed::communication_device::{get_first_marker, get_message_marker};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/06/input.txt")?;
    let solution = get_first_marker(input.as_str()).unwrap();
    println!(
        "Solution Part 1: {}",
        solution
    );
    let solution = get_message_marker(input.as_str()).unwrap();
    println!(
        "Solution Part 2: {}",
        solution
    );
    Ok(())
}

