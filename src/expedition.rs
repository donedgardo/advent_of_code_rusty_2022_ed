pub struct Expedition {
    elves: Vec<Elf>,
}

impl Expedition {
    pub fn default() -> Self {
        Self { elves: vec![] }
    }

    pub fn new(input: &str) -> Self {
        let mut expedition = Expedition::default();
        input.split("\n\n").for_each(|elves| {
            let mut elf = Elf::default();
            elves.split("\n").for_each(|food_item| {
                let food_calories: u32 = food_item.parse::<u32>().unwrap_or(0);
                elf.add_food_item(&food_calories);
            });
            expedition.add_elf(elf);
        });
        expedition
    }

    pub fn elf_with_most_calories(&self) -> Option<&Elf> {
        self.elves.iter().max_by_key(|elf| elf.calories_on_inventory())
    }

    pub fn top_n_elves_food_calories(&self, n: usize) -> Vec<Elf> {
        let mut elves = self.elves.to_vec();
        elves.sort_by_key(|elf| elf.calories_on_inventory());
        elves.into_iter().rev().take(n).collect()
    }

    pub fn top_n_elves_food_calories_total(&self, n: usize) -> u32 {
        self.top_n_elves_food_calories(n)
            .iter()
            .map(|elf| elf.calories_on_inventory())
            .sum::<u32>()
    }

    fn add_elf(&mut self, elf: Elf) {
        self.elves.push(elf);
    }
}

#[derive(Clone, Default)]
pub struct Elf {
    food_items: Vec<u32>,
}

impl Elf {
    pub fn add_food_item(&mut self, calories: &u32) {
        self.food_items.push(*calories);
    }

    pub fn calories_on_inventory(&self) -> u32 {
        self.food_items.iter().sum()
    }
}

#[cfg(test)]
mod elf_inventory_test
{
    use crate::expedition::{Elf, Expedition};

    #[test]
    fn zero_elves() {
        let input: &str = "0";
        let expedition: Expedition = Expedition::new(input);
        let elf: &Elf = expedition.elf_with_most_calories().unwrap();
        assert_eq!(elf.calories_on_inventory(), 0);
    }

    #[test]
    fn one_elf() {
        let input: &str = "1";
        let expedition: Expedition = Expedition::new(input);
        let elf: &Elf = expedition.elf_with_most_calories().unwrap();
        assert_eq!(elf.calories_on_inventory(), 1);
    }

    #[test]
    fn many_elves() {
        let input: &str = "1\n\n2";
        let expedition: Expedition = Expedition::new(input);
        let elf: &Elf = expedition.elf_with_most_calories().unwrap();
        assert_eq!(elf.calories_on_inventory(), 2);
    }

    #[test]
    fn many_elves_empty_spaces() {
        let input: &str = "1\n\n4\n\n\n3\n\n2";
        let expedition: Expedition = Expedition::new(input);
        let elf: &Elf = expedition.elf_with_most_calories().unwrap();
        assert_eq!(elf.calories_on_inventory(), 4);
    }

    #[test]
    fn many_elves_with_many_many_food_items() {
        let input: &str = "2\n3\n\n4";
        let expedition: Expedition = Expedition::new(input);
        let elf: &Elf = expedition.elf_with_most_calories().unwrap();
        assert_eq!(elf.calories_on_inventory(), 5);
    }

    #[test]
    fn top_n_elves_with_food_calories() {
        let input: &str = "2\n3\n\n4\n\n1\n2\n\n1";
        let expedition: Expedition = Expedition::new(input);
        assert_eq!(expedition.top_n_elves_food_calories(3).len(), 3);
        assert_eq!(
            expedition.top_n_elves_food_calories_total(3),
            12
        )
    }
}


#[cfg(test)]
mod advent_solve_code_test {
    use std::fs;
    use crate::expedition::Expedition;

    #[test]
    fn solves_day_1_example() -> std::io::Result<()> {
        let contents = fs::read_to_string("examples/01/sample.txt")?;
        let expedition = Expedition::new(contents.as_str());
        assert_eq!(expedition.elf_with_most_calories().unwrap().calories_on_inventory(), 24000);
        Ok(())
    }

    #[test]
    fn solves_day_1_example_2() -> std::io::Result<()> {
        let contents = fs::read_to_string("examples/01/sample.txt")?;
        let expedition = Expedition::new(contents.as_str());
        assert_eq!(
            expedition.top_n_elves_food_calories_total(3),
            45000);
        Ok(())
    }
}


