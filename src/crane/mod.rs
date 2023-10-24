use std::collections::VecDeque;
use std::io::Error;
use crate::crane::command::{CraneMoveCommand, parse_crane_commands};
use crate::crane::stacks::parse_crane_stacks;

mod command;
mod parse_error;
mod stacks;

pub fn parse_stacks_and_commands(full_input: &String) -> Result<(Vec<Vec<&str>>, Vec<CraneMoveCommand>), Error> {
    let split_input: Vec<&str> = full_input.split("\n\n").collect();
    let stacks = parse_crane_stacks(split_input[0]).unwrap();
    let commands = parse_crane_commands(split_input[1].trim()).unwrap();
    Ok((stacks, commands))
}

pub fn move_stacks(stacks: &mut Vec<Vec<&str>>, commands: &Vec<CraneMoveCommand>) {
    for command  in commands {
        for _ in 0..command.repeat {
            if let Some(cargo) = stacks[command.from - 1].pop() {
                stacks[command.to - 1].push(cargo);
            } else {
                panic!("Error moving cargo.");
            }
        }
    }
}

pub fn new_move_crane_commands(stacks: &mut Vec<Vec<&str>>, commands: &Vec<CraneMoveCommand>) {
    for command in commands {
        let mut new_stack: VecDeque<&str> = VecDeque::with_capacity(command.repeat);
        for _ in 0..command.repeat {
            if let Some(cargo) = stacks[command.from-1].pop() {
                new_stack.push_front(cargo);
            } else {
                panic!("Error moving cargo");
            };
        }
        stacks[command.to-1].append(&mut new_stack.into());
    }
}

#[cfg(test)]
mod crane_sample_test {
    use super::*;
    use std::fs;

    #[test]
    fn it_parses_commands_and_stacks() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/05/sample.txt")?;
        let (stacks, commands) = parse_stacks_and_commands(&input)?;

        assert_eq!(stacks.len(), 3);
        assert_eq!(commands.len(), 4);
        Ok(())
    }

    #[test]
    fn it_properly_moves_sample_stacks() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/05/sample.txt")?;
        let (mut stacks, commands) = parse_stacks_and_commands(&input)?;
        move_stacks(&mut stacks, &commands);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[1].len(), 1);
        assert_eq!(stacks[2].len(), 4);
        assert_eq!(stacks[0][0], "C");
        assert_eq!(stacks[1][0], "M");
        assert_eq!(stacks[2][0], "P");
        assert_eq!(stacks[2][1], "D");
        assert_eq!(stacks[2][2], "N");
        assert_eq!(stacks[2][3], "Z");
        Ok(())
    }

    #[test]
    fn new_crane_properly_moves_stacks() {
        let mut stacks = Vec::with_capacity(2);
        stacks.push(vec!["A", "B", "C"]);
        stacks.push(vec![]);
        let command = CraneMoveCommand {
            repeat: 3,
            from: 1,
            to: 2,
        };
        new_move_crane_commands(&mut stacks, &vec![command]);
        assert_eq!(stacks[0].len(), 0);
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[1][0], "A");
        assert_eq!(stacks[1][1], "B");
        assert_eq!(stacks[1][2], "C");
    }

    #[test]
    fn new_crane_properly_moves_sample_stacks() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/05/sample.txt")?;
        let (mut stacks, commands) = parse_stacks_and_commands(&input)?;
        new_move_crane_commands(&mut stacks, &commands);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[1].len(), 1);
        assert_eq!(stacks[2].len(), 4);
        assert_eq!(stacks[2][0], "P");
        assert_eq!(stacks[2][1], "Z");
        assert_eq!(stacks[2][2], "N");
        assert_eq!(stacks[2][3], "D");
        Ok(())
    }


}