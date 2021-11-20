mod game;
use game::{get_choice_from_prompt, get_game_result, get_random_choice, GameResult};

fn main() {
    let computer_choice = get_random_choice();
    let human_choice = get_choice_from_prompt();

    println!("My choice  : {}", computer_choice);
    let (winner, message) = get_game_result(computer_choice, human_choice);
    let result_msg = match winner {
        GameResult::Computer => "I am the winner!",
        GameResult::Human => "You are the winner!",
        GameResult::Tie => "It is a tie",
    };
    println!("{}", result_msg);

    if let Some(message) = message {
        println!(" >> '{}'", message);
    }
}
