use std::fs;
use advent_of_code_rusty_2022_ed::{Tourney};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("examples/02/input.txt")?;
    let tourney = Tourney::new(input.as_str());
    println!(
        "Solution Part 1: {}",
        tourney.my_score()
    );

    Ok(())
}
