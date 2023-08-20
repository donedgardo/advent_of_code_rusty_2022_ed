#[derive(Default)]
pub struct Tourney {
    rounds: Vec<Round>,
}

impl Tourney {
    pub fn new(input: &str) -> Self {
        Self::process_input(input, Round::new)
    }

    pub fn new_2(input: &str) -> Self {
        Self::process_input(input, Round::new_2)
    }

    pub fn my_score(self) -> u32 {
        self.rounds.iter().map(|round| round.score()).map(|score| score.1).sum()
    }

    fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    fn process_input(input: &str, operation: fn(&str) -> Round) -> Self {
        let mut tourney = Self::default();
        let rounds: Vec<Round> = input.split("\n")
            .map(|round_input| operation(round_input)).collect();
        rounds.into_iter().
            for_each(|round| tourney.add_round(round.into()));
        tourney

    }
}


#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum GameSymbols {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}


impl GameSymbols {
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum GameResult {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl GameResult {
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

pub struct Round(GameSymbols, GameSymbols);

impl Round {
    pub fn new(input: &str) -> Self {
        let symbols: Vec<&str> = input.split(" ").collect();
        if symbols.len() != 2 { panic!("Incorrect format for round"); }
        Self(Self::parse_symbol(symbols[0]), Self::parse_symbol(symbols[1]))
    }

    pub fn new_2(input: &str) -> Self {
        let symbols: Vec<&str> = input.split(" ").collect();
        if symbols.len() != 2 { panic!("Incorrect format for round"); }
        let symbol1 = Self::parse_symbol(symbols[0]);
        let result = Self::parse_result(symbols[1]);
        let symbol2 = Self::symbols_for_result(&symbol1, &result);
        Self(symbol1, symbol2)
    }

    pub fn score(&self) -> (u32, u32) {
        let result = &self.result();
        (
            *&self.0.discriminant() as u32 + result.0.discriminant() as u32,
            *&self.1.discriminant() as u32 + result.1.discriminant() as u32,
        )
    }

    pub fn result(&self) -> (GameResult, GameResult) {
        let diff = (*&self.0.discriminant() as i8 - *&self.1.discriminant() as i8) % 3;
        match diff {
            0 => (GameResult::Draw, GameResult::Draw),
            1 | -2 => (GameResult::Win, GameResult::Lost),
            _ => (GameResult::Lost, GameResult::Win)
        }
    }

    fn parse_result(symbol: &str) -> GameResult {
        match symbol {
            "X" => GameResult::Lost,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => { panic!("Incorrect result format!")}
        }
    }

    fn parse_symbol(symbol: &str) -> GameSymbols {
        let symbol = match symbol {
            "A" => GameSymbols::Rock,
            "B" => GameSymbols::Paper,
            "C" => GameSymbols::Scissors,
            "X" => GameSymbols::Rock,
            "Y" => GameSymbols::Paper,
            "Z" => GameSymbols::Scissors,
            _ => {
                panic!("Incorect format for symbol");
            }
        };
        symbol
    }

    fn symbols_for_result(player1: &GameSymbols, result: &GameResult) -> GameSymbols {
        let player1_discriminant = player1.discriminant();
        let result_discriminant = result.discriminant();
        let player2_discriminant: i8 = result_discriminant as i8 - player1_discriminant as i8;
        match player2_discriminant {
            2 | -2 | 3 => GameSymbols::Rock,
            1 | -3 | 5 => GameSymbols::Paper,
            0 | -1 | 4 => GameSymbols::Scissors,
            _ => panic!("No valid player 2 symbol for given result"), // No valid player2 symbol for the given result and player1 symbol
        }
    }
}

#[cfg(test)]
mod rock_paper_scissor_tourney_test {
    use std::fs;
    use crate::tourney::{GameResult, GameSymbols, Round, Tourney};

    #[test]
    fn it_scores_tourney_correctly() {
        let tourney = Tourney::new("A Y\nB X\nC Z");
        assert_eq!(tourney.my_score(), 15);
    }

    #[test]
    fn it_scores_tourney_correctly_2() {
        let tourney = Tourney::new_2("A Y\nB X\nC Z");
        assert_eq!(tourney.my_score(), 12);
    }

    // Round Test
    #[test]
    fn it_creates_a_turn() {
        let turn = Round::new("A X");
        assert_eq!(turn.0, GameSymbols::Rock);
        assert_eq!(turn.1, GameSymbols::Rock);

        let turn = Round::new("B Y");
        assert_eq!(turn.0, GameSymbols::Paper);
        assert_eq!(turn.1, GameSymbols::Paper);

        let turn = Round::new("C Z");
        assert_eq!(turn.0, GameSymbols::Scissors);
        assert_eq!(turn.1, GameSymbols::Scissors);
    }

    #[test]
    fn it_detects_draws() {
        let round = Round::new("A X");
        assert_eq!(round.result(), (GameResult::Draw, GameResult::Draw));
        let round = Round::new("B Y");
        assert_eq!(round.result(), (GameResult::Draw, GameResult::Draw));
        let round = Round::new("C Z");
        assert_eq!(round.result(), (GameResult::Draw, GameResult::Draw));
    }

    #[test]
    fn it_detects_lost_win() {
        let round = Round::new("A Y");
        assert_eq!(round.result(), (GameResult::Lost, GameResult::Win));
        let round = Round::new("B Z");
        assert_eq!(round.result(), (GameResult::Lost, GameResult::Win));
        let round = Round::new("C X");
        assert_eq!(round.result(), (GameResult::Lost, GameResult::Win));
    }

    #[test]
    fn it_detects_win_lost() {
        let round = Round::new("Y A");
        assert_eq!(round.result(), (GameResult::Win, GameResult::Lost));
        let round = Round::new("Z B");
        assert_eq!(round.result(), (GameResult::Win, GameResult::Lost));
        let round = Round::new("X C");
        assert_eq!(round.result(), (GameResult::Win, GameResult::Lost));
    }

    #[test]
    fn it_scores_correctly() -> std::io::Result<()> {
        let saved_snapshot = fs::read_to_string("examples/02/test_snapshots.txt")?;
        let symbols = ["A", "B", "C"];
        let mut current_snapshot = "".to_owned();
        for symbol1 in symbols.into_iter() {
            for symbol2 in symbols.into_iter() {
                let symbol1_name = format_symbol(symbol1);
                let symbol2_name = format_symbol(symbol2);
                let round = Round::new(format!("{} {}", symbol1, symbol2).as_str());
                let result = round.result();
                let outcome = match result.0 {
                    GameResult::Lost => "Lost to",
                    GameResult::Draw => "Draws to",
                    GameResult::Win => "Beats"
                };
                let score = round.score();
                current_snapshot.push_str(
                    format!(
                        "{} {} {}: Score {} | {}\n",
                        symbol1_name, outcome, symbol2_name, score.0, score.1).as_str()
                );
            }
        }
        if current_snapshot != saved_snapshot {
            fs::write("examples/02/test_snapshots.txt", current_snapshot.clone())?;
        }
        assert_eq!(current_snapshot, saved_snapshot);
        Ok(())
    }

    #[test]
    fn new_input_for_rounds_rock_draw() {
        let round = Round::new_2("A Y");
        assert_eq!(round.0, GameSymbols::Rock);
        assert_eq!(round.1, GameSymbols::Rock);
    }

    #[test]
    fn new_input_for_rounds_paper_draw() {
        let round = Round::new_2("B Y");
        assert_eq!(round.0, GameSymbols::Paper);
        assert_eq!(round.1, GameSymbols::Paper);
    }

    #[test]
    fn new_input_for_rounds_scissor_draw() {
        let round = Round::new_2("C Y");
        assert_eq!(round.0, GameSymbols::Scissors);
        assert_eq!(round.1, GameSymbols::Scissors);
    }

    #[test]
    fn new_input_for_rounds_rock_lost() {
        let round = Round::new_2("A X");
        assert_eq!(round.0, GameSymbols::Rock);
        assert_eq!(round.1, GameSymbols::Scissors);
    }

    #[test]
    fn new_input_for_rounds_paper_lost() {
        let round = Round::new_2("B X");
        assert_eq!(round.0, GameSymbols::Paper);
        assert_eq!(round.1, GameSymbols::Rock);
    }

    #[test]
    fn new_input_for_rounds_scissor_lost() {
        let round = Round::new_2("C X");
        assert_eq!(round.0, GameSymbols::Scissors);
        assert_eq!(round.1, GameSymbols::Paper);
    }

    #[test]
    fn new_input_for_rounds_rock_win() {
        let round = Round::new_2("A Z");
        assert_eq!(round.0, GameSymbols::Rock);
        assert_eq!(round.1, GameSymbols::Paper);
    }

    #[test]
    fn new_input_for_rounds_paper_win() {
        let round = Round::new_2("B Z");
        assert_eq!(round.0, GameSymbols::Paper);
        assert_eq!(round.1, GameSymbols::Scissors);
    }

    #[test]
    fn new_input_for_rounds_scissor_win() {
        let round = Round::new_2("C Z");
        assert_eq!(round.0, GameSymbols::Scissors);
        assert_eq!(round.1, GameSymbols::Rock);
    }




    #[test]
    fn parses_lost_result_correctly() {
        assert_eq!(Round::parse_result("X"), GameResult::Lost);
        assert_eq!(Round::parse_result("Y"), GameResult::Draw);
        assert_eq!(Round::parse_result("Z"), GameResult::Win);
    }

    fn format_symbol(symbol: &str) -> &str {
        match symbol {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissor",
            _ => "None"
        }
    }
}
