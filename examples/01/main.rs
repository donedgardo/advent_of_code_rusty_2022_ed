use std::fs;
use advent_of_code_rusty_2022_ed::{Elf, Expedition};

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("examples/01/input.txt")?;
    let expedition = Expedition::new(contents.as_str());
    println!(
        "Solution Part 1: {}",
        expedition.elf_with_most_calories()
            .unwrap_or(&Elf::default())
            .calories_on_inventory()
    );
    println!(
        "Solution Part 2: {}",
        expedition.top_n_elves_food_calories_total(3)
    );
    Ok(())
}
