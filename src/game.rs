use rand::{prelude::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct ChoiceCombination(Choice, Choice);

pub enum Side {
    Fisrt,
    Second,
}

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
            Some((Side::Fisrt, String::from("Rock breaks scissors")))
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
    if let Some(side) = which_side_won(&ChoiceCombination(computer_choice, human_choice)) {
        return match side {
            (Side::Fisrt, message) => (GameResult::Computer, Some(message)),
            (Side::Second, message) => (GameResult::Human, Some(message)),
        };
    };

    if let Some(side) = which_side_won(&ChoiceCombination(human_choice, computer_choice)) {
        return match side {
            (Side::Fisrt, message) => (GameResult::Human, Some(message)),
            (Side::Second, message) => (GameResult::Computer, Some(message)),
        };
    };

    (GameResult::Tie, None)
}

pub fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let mut rng = thread_rng();
    choices.choose(&mut rng).unwrap().to_owned()
}

pub fn get_choice_from_prompt() -> Choice {
    let labels = ["Rock", "Paper", "Scissors"];
    let matches = [Choice::Rock, Choice::Paper, Choice::Scissors];

    let selected_index: usize = dialoguer::Select::new()
        .with_prompt("Your choice")
        .items(&labels[..])
        .interact()
        .unwrap();

    matches[selected_index]
}
