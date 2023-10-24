use cli::CommandLine;
use crate::cli::DirectoryNode;

pub mod expedition;
pub mod tourney;
mod rucksack;
pub mod elves_group;
pub mod cleaning_elves;
pub mod crane;
pub mod communication_device;
mod cli;

pub fn find_answer_1(input: &str) -> usize {
    let cli = parse_filesystem(input).unwrap();
    let answer: usize = cli
        .directories()
        .iter()
        .map(|dir| match dir {
            DirectoryNode::Directory(_, path) =>
                cli.dir_size(path).unwrap_or(0),
            _ => 0
        })
        .filter(|&dir_size| dir_size <= 100000)
        .sum();
    answer
}

pub fn parse_filesystem(input: &str) -> Option<CommandLine> {
    if input.is_empty() {
        return None;
    }
    let mut cli = CommandLine::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let args: Vec<&str> = line.split(" ").skip(2).collect();
            assert!(!args.is_empty());
            cli.cd(args[0]);
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let args: Vec<&str> = line.split(" ").collect();
            assert_eq!(args.len(), 2);
            cli.add_directory(args[1].clone());
        } else {
            let args: Vec<&str> = line.split(" ").collect();
            assert_eq!(args.len(), 2, "args should be 2 {:?}", args);
            cli.add_file(args[1].clone(), args[0].clone().parse::<usize>().unwrap());
        }
    }
    cli.cd("/");
    Some(cli)
}

#[cfg(test)]
mod filesystem_input_tests {
    use std::fs;
    use crate::cli::DirectoryNode;
    use super::*;

    #[test]
    fn it_parses_empty() {
        let input = "";
        assert!(parse_filesystem(input).is_none())
    }

    #[test]
    fn it_interprets_root_command() {
        let input = "$ cd /";
        let cli = parse_filesystem(input).unwrap();
        assert_eq!(cli.pwd(), "/");
    }

    #[test]
    fn it_interprets_ls() {
        let input = "$ cd /\n$ ls\ndir a\n23 b.txt";
        let mut cli = parse_filesystem(input).unwrap();
        assert_eq!(cli.ls(), "dir a\n23 b.txt");
        cli.cd("a");
        assert_eq!(cli.ls(), "");
    }

    #[test]
    fn it_reads_dir_size() {
        let input = "$ cd /\n$ ls\n23 b.txt";
        let cli = parse_filesystem(input).unwrap();
        assert_eq!(cli.dir_size("/"), Some(23));
    }

    #[test]
    fn it_reads_nested_dir_size() {
        let input = "$ cd /\n$ ls\n23 b.txt\ndir c\n$ cd c\n$ ls\n123 d.txt";
        let cli = parse_filesystem(input).unwrap();
        assert_eq!(cli.dir_size("/"), Some(146));
        assert_eq!(cli.dir_size("/c"), Some(123));
    }

    #[test]
    fn it_gets_all_dir_sizes() {
        let input = "$ cd /\n$ ls\n20 b.txt\ndir c\n$ cd c\n$ ls\n120 d.txt";
        let cli = parse_filesystem(input).unwrap();
        let dir_sizes: Vec<usize> = cli
            .directories()
            .iter()
            .map(|dir| match dir {
                DirectoryNode::Directory(_, path) =>
                    cli.dir_size(path).unwrap_or(0),
                _ => 0
            }).collect();
        assert_eq!(dir_sizes.len(), 2);
        assert_eq!(dir_sizes[0], 140);
        assert_eq!(dir_sizes[1], 120);
    }

    #[test]
    fn it_answers_sample_input() {
        let input = fs::read_to_string("src/bin/07/sample.txt").unwrap();
        let answer = find_answer_1(input.as_str());
        assert_eq!(answer, 95437);
    }


}


