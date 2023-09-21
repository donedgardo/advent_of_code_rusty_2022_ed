use crate::rucksack::Rucksack;

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


pub fn item_priority_value(item: char) -> u32 {
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
