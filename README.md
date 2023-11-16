# Rust Number Guessing Game

## Overview
This Rust-based Number Guessing Game is a simple yet engaging command-line application where players guess a randomly generated number within a specified range. It showcases fundamental Rust programming concepts, including control flow, user input handling, and basic error management.

## Features
- **Random Number Generation:** Each game generates a random number between 1 and 100.
- **Guess Validation:** User guesses are validated for format and range.
- **Attempts Limit:** Players have a limited number of attempts to guess the number.
- **Dynamic User Interaction:** Users receive real-time feedback on each guess, with hints.
- **Replayability:** After each game, players can choose to play again.
- **Colored Output:** Feedback messages are color-coded for better readability.

## Installation

### Prerequisites
Rust Programming Language: Ensure you have Rust installed on your system. If not, you can install it from [the official Rust website](https://www.rust-lang.org/tools/install).

### Steps
1. Clone this repository to your local machine:
```
git clone git@github.com:alinikan/Rust-Guessing-Game.git
```
2. Navigate to the project directory:
```
cd Rust-Guessing-Game
```
3. Build the project:
```
cargo build
```
4. Run the project:
```
cargo run
```

## How to Play
- Run the game as instructed above.
- Follow the on-screen prompts to input your guesses.
- After each guess, you'll receive feedback: "Too small!", "Too big!", or "You win!".
- You have a maximum of 10 attempts per game.
- After a game, choose to play again or exit.

## Code Structure
- `main.rs`: Contains the main game loop and user interaction logic.
- `GameState`: An enum to manage game states like Continue, Won, and Lost.
- `play_game()`: Handles the gameplay logic.
- `read_guess()`: Manages user input and validates guesses.
- `process_guess()`: Processes each guess and updates the game state.
- `ask_replay()`: Offers the option to replay after each game.