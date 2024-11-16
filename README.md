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