use std::fmt;

use rand::{prelude::SliceRandom, thread_rng};

#[derive(Clone, Copy, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Choice::Paper => "Paper",
                Choice::Rock => "Rock",
                Choice::Scissors => "Scissors",
            }
        )
    }
}

struct ChoiceCombination(Choice, Choice);

pub enum Side {
    First,
    Second,
}

#[derive(PartialEq, Debug)]
pub enum GameResult {
    Computer,
    Human,
    Tie,
}

fn which_side_won(combination: &ChoiceCombination) -> Option<(Side, String)> {
    match combination {
        ChoiceCombination(Choice::Rock, Choice::Paper) => {
            Some((Side::Second, String::from("Paper covers rock")))
        }
        ChoiceCombination(Choice::Rock, Choice::Scissors) => {
            Some((Side::First, String::from("Rock breaks scissors")))
        }
        ChoiceCombination(Choice::Paper, Choice::Scissors) => {
            Some((Side::Second, String::from("Scissors cut paper")))
        }
        _ => None,
    }
}

pub fn get_game_result(
    computer_choice: Choice,
    human_choice: Choice,
) -> (GameResult, Option<String>) {
    if computer_choice == human_choice {
        return (GameResult::Tie, None);
    }

    if let Some(side) = which_side_won(&ChoiceCombination(computer_choice, human_choice)) {
        return match side {
            (Side::First, message) => (GameResult::Computer, Some(message)),
            (Side::Second, message) => (GameResult::Human, Some(message)),
        };
    };

    if let Some(side) = which_side_won(&ChoiceCombination(human_choice, computer_choice)) {
        return match side {
            (Side::First, message) => (GameResult::Human, Some(message)),
            (Side::Second, message) => (GameResult::Computer, Some(message)),
        };
    };

    panic!("Impossible state")
}

pub fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let mut rng = thread_rng();
    choices.choose(&mut rng).unwrap().to_owned()
}

pub fn get_choice_from_prompt() -> Choice {
    let matches = [Choice::Rock, Choice::Paper, Choice::Scissors];

    let selected_index = dialoguer::Select::new()
        .with_prompt("Your choice")
        .items(&matches)
        .interact()
        .unwrap();

    matches[selected_index]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_game_result_works() {
        let table = vec![
            (Choice::Rock, Choice::Rock, GameResult::Tie),
            (Choice::Rock, Choice::Paper, GameResult::Human),
            (Choice::Rock, Choice::Scissors, GameResult::Computer),
            //
            (Choice::Paper, Choice::Rock, GameResult::Computer),
            (Choice::Paper, Choice::Paper, GameResult::Tie),
            (Choice::Paper, Choice::Scissors, GameResult::Human),
            //
            (Choice::Scissors, Choice::Rock, GameResult::Human),
            (Choice::Scissors, Choice::Paper, GameResult::Computer),
            (Choice::Scissors, Choice::Scissors, GameResult::Tie),
        ];

        for (computer, human, expected_result) in table {
            let res = get_game_result(computer, human).0;
            assert_eq!(
                res, expected_result,
                "When computer choice is {} and human choice is {}, the result should be {:?}. Got {:?}",
                computer, human, expected_result, res
            );
        }
    }
}
