use tree::Tree;

pub mod expedition;
pub mod tourney;
mod rucksack;
pub mod elves_group;
pub mod cleaning_elves;
pub mod crane;
mod communication_device;
mod tree;

fn parse_filesystem(input: &str) -> Option<Tree<i32>> {
    if input.is_empty() {
        return None;
    }
    let tree = Tree::new();
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

    #[ignore]
    #[test]
    fn it_parses_sample() {
        let input =
            "$ cd /\
             $ ls \
             dir a\
             14848514 b.txt\
             8504156 c.dat\
             dir d ";
        let _ = parse_filesystem(input).unwrap();
    }
}


