use std::fs;
use std::io::Error;
use crate::crane::command::{CraneCommand, parse_crane_commands};
use crate::crane::stacks::parse_crane_stacks;

mod command;
mod parse_error;
mod stacks;

pub fn parse_stacks_and_commands(full_input: &String) -> Result<(Vec<Vec<&str>>, Vec<CraneCommand>), Error> {
    let split_input: Vec<&str> = full_input.split("\n\n").collect();
    let stacks = parse_crane_stacks(split_input[0]).unwrap();
    let commands = parse_crane_commands(split_input[1].trim()).unwrap();
    Ok((stacks, commands))
}


#[cfg(test)]
mod crane_sample_test {
    use super::*;

    #[test]
    fn it_parses_commands_and_stacks() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/05/sample.txt")?;
        let (stacks, commands) = parse_stacks_and_commands(&input)?;

        assert_eq!(stacks.len(), 3);
        assert_eq!(commands.len(), 4);
        Ok(())
    }


}