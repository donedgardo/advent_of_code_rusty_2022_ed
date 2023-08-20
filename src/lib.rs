pub mod expedition;
pub mod tourney;


#[derive(Default)]
pub struct Rucksack(String, String);

impl Rucksack {
    pub fn new(input: &str) -> Self {
        let compartments = input.split_at(input.len() / 2);
        Rucksack(compartments.0.to_string(), compartments.1.to_string())
    }

    pub fn common_item_in_compartments(&self) -> char {
        let mut common_char = char::default();
        for char in self.0.as_str().chars() {
            if self.1.contains(char) {
                common_char = char;
                break;
            }
        }
        common_char
    }
}

pub struct ElvesGroup(String, String, String);

impl ElvesGroup {
    pub fn new(input: &str) -> Self {
        let elves: Vec<&str> = input.split("\n").collect();
        assert_eq!(elves.len(), 3);
        Self(elves[0].to_string(), elves[1].to_string(), elves[2].to_string())
    }
    pub fn badge(&self) -> char {
        let mut common_char = char::default();
        for char in self.0.as_str().chars() {
            if self.1.contains(char) && self.2.contains(char) {
                common_char = char;
                break;
            }
        }
        common_char
    }
}


fn item_priority_value(item: char) -> u32 {
    let mut value: u32 = 0;
    let priority_values = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for (i, char) in priority_values.chars().enumerate() {
        if char == item {
            value = i as u32 + 1;
        }
    }
    value
}

pub fn total_duplicate_priorities(input: &str) -> u32 {
    input
        .split("\n")
        .map(|rucksack_input|
            Rucksack::new(rucksack_input).common_item_in_compartments()
        ).map(|char| item_priority_value(char))
        .sum::<u32>()
}

pub fn total_badge_priorities(input: &str) -> u32 {
    let elves: Vec<&str> = input.split("\n").collect();
    elves.chunks(3).map(|group| {
        let elves_group = ElvesGroup::new(group.join("\n").as_str());
        item_priority_value(elves_group.badge())
    }).sum::<u32>()
}


#[cfg(test)]
mod rucksack_test {
    use std::fs;
    use crate::{ElvesGroup, item_priority_value, Rucksack, total_badge_priorities, total_duplicate_priorities};

    #[test]
    fn creates_rucksack() {
        let rucksack = Rucksack::new("abcAbC");
        assert_eq!(rucksack.common_item_in_compartments(), 'b');
        let rucksack = Rucksack::new("abcABc");
        assert_eq!(rucksack.common_item_in_compartments(), 'c');
    }

    #[test]
    fn creates_elves_group() {
        let group = ElvesGroup::new("abc\nyaz\nYaZ");
        assert_eq!(group.badge(), 'a');
    }

    #[test]
    fn gets_total_duplicate_priorities() {
        let input = "abcAbC\nabcaBC";
        assert_eq!(total_duplicate_priorities(input), 3);
        let input = "abcdABCd\nabcdaBCD";
        assert_eq!(total_duplicate_priorities(input), 5);
    }

    #[test]
    fn gets_total_badge_priorities() {
        let input = "abc\nAbC\nZNb\nnBc\nNbc\nXAc";
        assert_eq!(total_badge_priorities(input), 5);
    }

    #[test]
    fn it_gets_priority() {
        assert_eq!(item_priority_value('a'), 1);
        assert_eq!(item_priority_value('z'), 26);
        assert_eq!(item_priority_value('A'), 27);
        assert_eq!(item_priority_value('Z'), 52);
    }

    #[test]
    fn solves_day3_sample() -> std::io::Result<()> {
        let input = fs::read_to_string("examples/03/sample.txt")?;
        assert_eq!(total_duplicate_priorities(input.as_str()), 157);
        Ok(())
    }

    #[test]
    fn solves_day3_part2_sample() -> std::io::Result<()> {
        let input = fs::read_to_string("examples/03/sample.txt")?;
        assert_eq!(total_badge_priorities(input.as_str()), 70);
        Ok(())
    }
}