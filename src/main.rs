mod game;
use game::{get_choice_from_prompt, get_game_result, get_random_choice, GameResult};

fn main() {
    let computer_choice = get_random_choice();
    let human_choice = get_choice_from_prompt();

    println!("My choice  : {:#?}", computer_choice);
    let (winner, message) = get_game_result(computer_choice, human_choice);
    match winner {
        GameResult::Computer => println!("I am the winner!"),
        GameResult::Human => println!("You are the winner!"),
        GameResult::Tie => println!("It is a tie"),
    }
    if let Some(message) = message {
        println!("'{}'", message);
    }
}
