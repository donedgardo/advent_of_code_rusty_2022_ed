use std::fs;
use advent_of_code_rusty_2022_ed::find_answer_1;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/07/input.txt")?;
    let solution = find_answer_1(input.as_str());
    println!(
        "Solution Part 1: {}",
        solution
    );
    Ok(())
}

