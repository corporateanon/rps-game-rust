use dialoguer::console;
use rand::{prelude::SliceRandom, thread_rng};

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

// struct Round {
//     machine_choice: Option<Choice>,
//     human_choice: Option<Choice>,
// }

fn main() {
    let labels = &["R", "P", "S"];

    let sel: usize = dialoguer::Select::new()
        .with_prompt("Your choice")
        .items(&labels[..])
        .interact()
        .unwrap();

    let selection = labels[sel];

    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let mut rng = thread_rng();
    let choice = choices.choose(&mut rng).unwrap();
    println!("{}", selection);
    println!("{:#?}", choice);
}
