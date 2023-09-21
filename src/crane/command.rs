use crate::crane::parse_error::ParseError;

#[derive(Debug)]
pub struct CraneCommand {
    repeat: usize,
    from: usize,
    to: usize,
}

pub fn parse_crane_command(x: &str) -> Result<CraneCommand, ParseError>  {
    if x.is_empty() {
        return Err(ParseError::Empty);
    }
    let x_split: Vec<&str> = x.split(" ").collect();
    if let (Ok(repeat), Ok(from), Ok(to)) = (x_split[1].parse::<usize>(), x_split[3].parse::<usize>(), x_split[5].parse::<usize>()) {
        Ok(CraneCommand {
            repeat,
            from,
            to,
        })
    } else {
        Err(ParseError::Fail)
    }
}

pub fn parse_crane_commands(input: &str) -> Result<Vec<CraneCommand>, ParseError> {
    let command_lines: Vec<&str> = input.split("\n").collect();
    let mut commands: Vec<CraneCommand> = Vec::with_capacity(command_lines.len());
    for command_line in command_lines {
        let command = parse_crane_command(command_line)?;
        commands.push(command);
    }
    Ok(commands)
}

#[cfg(test)]
mod crane_parse_test {
    use super::*;

    #[test]
    fn it_parses_zero_command() {
        let command = parse_crane_command("move 0 from 0 to 0").unwrap();
        assert_eq!(command.repeat, 0);
        assert_eq!(command.from, 0);
        assert_eq!(command.to, 0);
    }

    #[test]
    fn it_parses_correctly() {
        let command = parse_crane_command("move 1 from 2 to 1").unwrap();
        assert_eq!(command.repeat, 1);
        assert_eq!(command.from, 2);
        assert_eq!(command.to, 1);
    }

    #[test]
    fn it_errors_on_invalid_repeat() {
        let command = parse_crane_command("move x from 2 to 1");
        assert!(command.is_err());
        assert_eq!(command.unwrap_err(), ParseError::Fail);
    }

    #[test]
    fn it_errors_on_invalid_from() {
        let command = parse_crane_command("move 0 from D to 1");
        assert!(command.is_err());
        assert_eq!(command.unwrap_err(), ParseError::Fail);
    }

    #[test]
    fn it_errors_on_invalid_to() {
        let command = parse_crane_command("move 0 from 0 to y");
        assert!(command.is_err());
        assert_eq!(command.unwrap_err(), ParseError::Fail);
    }

    #[test]
    fn it_errors_on_empty() {
        let command = parse_crane_command("");
        assert!(command.is_err());
        assert_eq!(command.unwrap_err(), ParseError::Empty);
    }

    #[test]
    fn it_parses_multiple_commands() {
        let commands = parse_crane_commands(
            "move 1 from 2 to 1\nmove 3 from 1 to 3"
        ).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].repeat, 1);
        assert_eq!(commands[0].from, 2);
        assert_eq!(commands[0].to, 1);
        assert_eq!(commands[1].repeat, 3);
        assert_eq!(commands[1].from, 1);
        assert_eq!(commands[1].to, 3);
    }


}
