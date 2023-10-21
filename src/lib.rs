use tree::Tree;
use crate::tree::TreeNode;

pub mod expedition;
pub mod tourney;
mod rucksack;
pub mod elves_group;
pub mod cleaning_elves;
pub mod crane;
pub mod communication_device;
mod tree;

fn parse_filesystem(input: &str) -> Option<Tree<i32>> {
    if input.is_empty() {
        return None;
    }
    let mut tree = Tree::new();
    let root = tree.create_root(0);
    for line in input.lines() {
        if line.starts_with("$ cd") {}
        else if line.starts_with("$ ls") {}
        else {
            let child = TreeNode::new(line[0..=0].parse::<i32>().unwrap_or(0));
            root.add_child(child);
        }
    }
    Some(tree)
}

#[cfg(test)]
mod filesystem_input_tests {
    use super::*;

    #[test]
    fn it_parses_empty() {
        let input = "";
        assert!(parse_filesystem(input).is_none())
    }

    #[test]
    fn it_interprets_root_command() {
        let input = "$ cd /";
        let tree = parse_filesystem(input).unwrap();
        assert!(!tree.is_empty());
    }

    #[test]
    fn it_interprets_ls_command() {
        let input = "$ cd /\n$ ls\n1 a.txt";
        let tree = parse_filesystem(input).unwrap();
        assert!(!tree.is_empty());
        assert_eq!(tree.read_depth_first(), &[&1, &0]);
    }
}


