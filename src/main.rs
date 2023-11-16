use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

enum GameState {
    Continue,
    Won,
    Lost,
}

fn main() {
    loop {
        play_game();
        if !ask_replay() {
            break;
        }
    }
    println!("Thanks for playing!");
}

fn play_game() {
    println!("Guess the number!");

    let (min, max) = (1, 100); // Range for the secret number
    let max_attempts = 10;     // Maximum number of attempts
    let secret_number = rand::thread_rng().gen_range(min, max);

    let mut attempts = 0;

    loop {
        println!("Please input your guess (Attempt {} of {})", attempts + 1, max_attempts);

        let guess = match read_guess(min, max) {
            Some(g) => g,
            None => continue,
        };

        attempts += 1;

        match process_guess(guess, secret_number, attempts, max_attempts) {
            GameState::Continue => continue,
            GameState::Won => {
                println!("{}", "You win!".green());
                break;
            },
            GameState::Lost => {
                println!("You've run out of guesses. The number was: {}", secret_number);
                break;
            },
        }
    }
}

fn read_guess(min: u32, max: u32) -> Option<u32> {
    let mut guess = String::new();

    if io::stdin().read_line(&mut guess).is_err() {
        println!("Failed to read line");
        return None;
    }

    match guess.trim().parse() {
        Ok(num) => {
            if num < min || num > max {
                println!("Please enter a number between {} and {}.", min, max);
                None
            } else {
                Some(num)
            }
        }
        Err(_) => {
            println!("Please enter a valid number.");
            None
        }
    }
}

fn process_guess(guess: u32, secret_number: u32, attempts: u32, max_attempts: u32) -> GameState {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("{}", "Too small!".red());
            if attempts >= max_attempts {
                GameState::Lost
            } else {
                GameState::Continue
            }
        },
        Ordering::Greater => {
            println!("{}", "Too big!".red());
            if attempts >= max_attempts {
                GameState::Lost
            } else {
                GameState::Continue
            }
        },
        Ordering::Equal => GameState::Won,
    }
}

fn ask_replay() -> bool {
    loop {
        println!("Do you want to play again? (yes/no)");
        let mut answer = String::new();

        io::stdin().read_line(&mut answer).expect("Failed to read line");

        match answer.trim().to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Invalid input. Please type 'yes' or 'no'."),
        }
    }
}
