use crate::crane::parse_error::ParseError;

pub fn parse_crane_stacks(input: &str) -> Result<Vec<Vec<&str>>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }
    let mut input_lines: Vec<&str> = input.split("\n").collect();
    if (input_lines[0].len() % 4) != 0 {
        return Err(ParseError::Fail);
    }
    let stack_count = input_lines[0].len() / 4;
    let mut stacks = vec!(vec![]; stack_count);
    input_lines.pop();
    for line in input_lines.into_iter().rev() {
        for stack_number in 0..stack_count {
            let begin = stack_number * 4;
            let cargo = &line[begin..begin + 4];
            stacks[stack_number].push(&cargo[1..=1])
        }
    }
    Ok(stacks)
}

#[cfg(test)]
mod stacks_parse_test {
    use super::*;
    use crate::crane::parse_error::ParseError;

    #[test]
    fn it_errors_empty_stacks_input() {
        let stacks = parse_crane_stacks("");
        assert!(stacks.is_err());
        assert_eq!(stacks.unwrap_err(), ParseError::Empty);
    }

    #[test]
    fn it_calculates_total_stacks() {
        let stacks = parse_crane_stacks(" 1   2   3   4   5  ").unwrap();
        assert_eq!(stacks.len(), 5);
        for i in 0..stacks.len() {
            assert!(stacks[i].is_empty())
        }
    }

    #[test]
    fn it_calculates_single_simple_stack() {
        let stacks = parse_crane_stacks("[H] \n 1  ").unwrap();
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0], "H");
    }

    #[test]
    fn it_calculates_two_stacks() {
        let stacks = parse_crane_stacks("[H] [a] \n 1  2  ").unwrap();
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[1].len(), 1);
        assert_eq!(stacks[0][0], "H");
        assert_eq!(stacks[1][0], "a");
    }

    #[test]
    fn it_calculates_order_of_single_stack() {
        let stacks = parse_crane_stacks("[H] \n[C] \n 1  ").unwrap();
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[0][0], "C");
        assert_eq!(stacks[0][1], "H");
    }

    #[test]
    fn it_calculates_order_of_multiple_stacks() {
        let stacks = parse_crane_stacks("[A] [X] \n[B] [Y] \n 1   2  ").unwrap();
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[1].len(), 2);
        assert_eq!(stacks[0][0], "B");
        assert_eq!(stacks[0][1], "A");
        assert_eq!(stacks[1][0], "Y");
        assert_eq!(stacks[1][1], "X");
    }
}
