# My Rust Journey
A repo documenting my journey in learning rust. I am primarily following alogn with the official Rust Programming Language book but will be supplementing it with additional video tutorials and small projects. 

# Table of Contents
## 1-hello_cargo
A simple 'Hello, World' program to familiarize myself with Cargo and basic Rust syntax and project structure. 

## 2-guessing_game
A classic beginner game that generates a random integer and repeatedly prompts the player to guess the number by indicating whether their previous guess was too high or too low. Correctly guessing the number will display a congratulatory message. 

I also added a simple difficulty mode that controls the number of guesses available to a player. Failure to guess correctly within the limit ends the game. 

**_Some of the concepts I learned include_**: 
- Immutable and mutable variables with `let` and `mut`
- Other variable concepts such as type assertions and constants with `const`
- Pattern matching with the `match` keyword
- Rust&apos;s scalar data types such as booleans, chars, signed and unsigned integers, floating point numbers, and strings
- Rust&apos;s compound data types such as tuples and arrays
- Using crates like `std::io`, `std::cmp`, `rand::Rng` and their functions
- Loops with `loop` and `for` for when I need to iterate over ranges with the `x..=y` syntax
- Basic control flow with if statements

## 3-tic_tac_toe
A classic game rewritten as a CLI application, currently with no single player mode, chosen primarily to practice the concepts of ownership and borrowing. The game prints the game state and allows players, alternating between X and O, to choose where to place their mark. The game loop checks to see if the board is full, in which case it registers that the game is a draw, or if either player has completed a winning combo.

Over the course of making this game I learned about: 
- Ownership and borrowing
- Loops and iterators
- Conditionals and error checking
- Error handling with Result and Option enums
- Debugging Rust in VS Code

## 4-tic_tac_toe_advanced
Tic-Tac-Toe rewritten with an improved UI and an AI player using a Minimax algorithm for single-player mode. This marks the first project where I used an external crate (crossterm) to create an enhanced terminal experience and to handle user input more efficiently than simply reading it in. 

Over the course of remaking this game I learned how to:
- Use external crates such as `crossterm`
- Split my code into modules for enhanced readability
- Propagate errors for more efficient error handling
- Define, implement, and use Structs and Enums
- Implement a Minimax algorithm that I found online to create an unbeatable CPU Player

**Unfortunately, this code doesn't function properly yet due to an error that I have yet to figure out how to fix. More specifically, the next method of the Tabs Struct I created is called more than once whenever I press Tab or Backtab despite the method only being called once.** 

## 5-weather_app
A simple weather app that fetches real-time data and serves as an introduction to web servers written in Rust. 