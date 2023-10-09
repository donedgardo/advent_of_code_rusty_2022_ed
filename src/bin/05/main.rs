use std::fs;
use advent_of_code_rusty_2022_ed::crane::{parse_stacks_and_commands, move_stacks, new_move_crane_commands};

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/bin/05/input.txt")?;
    let (mut stacks, commands) = parse_stacks_and_commands(&input).unwrap();
    let mut solution_2_stacks = stacks.clone();
    move_stacks(&mut stacks, &commands);
    let solution = cargo_in_top(stacks);
    println!(
        "Solution Part 1: {}",
        solution
    );
    new_move_crane_commands(&mut solution_2_stacks, &commands);
    let solution = cargo_in_top(solution_2_stacks);
    println!(
        "Solution Part 2: {}",
        solution
    );
    Ok(())
}

fn cargo_in_top(stacks: Vec<Vec<&str>>) -> String {
    let mut solution: String = "".to_string();
    for stack in stacks {
        solution += stack[stack.len() - 1];
    }
    solution
}
