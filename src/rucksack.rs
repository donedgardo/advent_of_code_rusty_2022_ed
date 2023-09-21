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


#[cfg(test)]
mod rucksack_test {
    use std::fs;
    use crate::elves_group::{ElvesGroup, item_priority_value, total_badge_priorities, total_duplicate_priorities};
    use crate::rucksack::Rucksack;

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
        let input = fs::read_to_string("src/bin/03/sample.txt")?;
        assert_eq!(total_duplicate_priorities(input.as_str()), 157);
        Ok(())
    }

    #[test]
    fn solves_day3_part2_sample() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/03/sample.txt")?;
        assert_eq!(total_badge_priorities(input.as_str()), 70);
        Ok(())
    }
}
