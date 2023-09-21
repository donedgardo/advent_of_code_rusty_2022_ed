use std::fs;
use advent_of_code_rusty_2022_ed::tourney::Tourney;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/02/input.txt")?;
    let tourney = Tourney::new(input.as_str());
    println!(
        "Solution Part 1: {}",
        tourney.my_score()
    );

    let tourney = Tourney::new_2(input.as_str());
    println!(
        "Solution Part 2: {}",
        tourney.my_score()
    );

    Ok(())
}
