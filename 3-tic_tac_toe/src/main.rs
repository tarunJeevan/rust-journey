#![allow(clippy::manual_range_contains)]
use std::io;

fn main() {
    // Initial display containing game rules
    println!(
        "
    \rPlay Tic-Tac-Toe!
    \rRules:
    \r\t1. Input the number in the space you want to choose when prompted then press Enter.
    \r\t2. You cannot select a space already containing either an 'X' or an 'O'.
    \r\t3. Type 'q' or 'quit' to exit the game.
    "
    );

    // Initializing board and game state
    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let players: [char; 2] = ['X', 'O'];
    let mut turn: usize = 0;
    let winning_combos = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    let mut continue_game = true;

    print_board(&board);

    // Game loop
    while continue_game {
        // Check if board is full
        continue_game = board.iter().any(|val| (*val != 'X' && *val != 'O'));

        if !continue_game {
            println!("GAME ENDED IN A DRAW!!");
            break;
        }

        println!("Choose a position for player {}: ", players[turn]);
        let index = get_index_from_input();

        // If there was an error
        if let Err(e) = index {
            println!("{e}");
            continue;
        }

        // Already validated so there should be no issue with unwrap()
        let index = index.unwrap();

        // If player wants to quit
        if index.is_none() {
            println!("Thank you for playing!");
            break;
        }

        // Get index value
        let index = index.unwrap();

        // If player chose invalid position
        if board[index] == 'X' || board[index] == 'O' {
            println!("Invalid choice. Please select a new position.");
            continue;
        }

        // Update board state
        board[index] = players[turn];

        // Print board
        print_board(&board);

        // Increment turn and determine player turn
        turn = (turn + 1) % 2;

        // Check if current player has won
        for combo in winning_combos {
            let [a, b, c] = combo;

            if board[a] == players[turn] && board[b] == players[turn] && board[c] == players[turn] {
                // Current player won
                continue_game = false;
                println!("PLAYER {} HAS WON!!", players[turn]);
            }
        }
    }
}

fn print_board(board: &[char; 9]) {
    // Printing game board
    println!(
        "
    \r\t+---+---+---+
    \r\t| {} | {} | {} |
    \r\t+---+---+---+
    \r\t| {} | {} | {} |
    \r\t+---+---+---+
    \r\t| {} | {} | {} |
    \r\t+---+---+---+
    ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}

fn get_index_from_input() -> Result<Option<usize>, String> {
    let mut input: String = String::new();

    // Read in user input and propagate any error
    let _ = io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input: &str = input.trim();

    // Validate user input
    if input.to_lowercase() == "q" || input.to_lowercase() == "quit" {
        return Ok(None);
    }

    // Convert input into index and propagate any error
    let index = input
        .parse::<usize>()
        .map_err(|_| "Input should be an integer!".to_string())?;

    // Confirm index is within expected range
    if index > 9 || index < 1 {
        return Err("Please enter a number from 1 to 9.".to_string());
    }

    // Return index
    Ok(Some(index - 1))
}
