use std::fs;
use advent_of_code_rusty_2022_ed::cleaning_elves::{filter_pairs, does_partial_overlap, fully_contains_each_other};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/04/input.txt")?;
    let overlapping = filter_pairs(input.as_str(), fully_contains_each_other);
    println!(
        "Solution Part 1: {}",
        overlapping.len()

    );
    let overlapping = filter_pairs(input.as_str(), does_partial_overlap);
    println!(
        "Solution Part 2: {}",
        overlapping.len()

    );
    Ok(())
}
