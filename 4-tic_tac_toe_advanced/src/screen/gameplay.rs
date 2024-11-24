use crate::{
    models::{Player, Tabs},
    utils::{get_best_move, get_winner},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub fn gameplay(player: &Player) -> io::Result<(Option<Player>, [char; 9], bool)> {
    let mut output = io::stdout();
    let mut board = [' '; 9];
    let mut board_spaces = Tabs::<usize>::new(vec![
        (15, 9, 0),
        (19, 9, 1),
        (23, 9, 2),
        (15, 11, 3),
        (19, 11, 4),
        (23, 11, 5),
        (15, 13, 6),
        (19, 13, 7),
        (23, 13, 8),
    ]);

    let comp = player.other();
    let mut turn = Player::X;

    loop {
        // Clear screen
        execute!(
            output,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan)
        )?;

        print_screen(board);

        // Highlight selected tab
        execute!(
            output,
            cursor::MoveTo(board_spaces.position().0, board_spaces.position().1),
            SetBackgroundColor(Color::Red),
            Print(board[*board_spaces.value()]),
            ResetColor,
            cursor::MoveTo(board_spaces.position().0, board_spaces.position().1),
        )?;

        output.flush()?;

        let winner = get_winner(&board);

        if winner.is_some() {
            return Ok((winner, board, true));
        }
        if !board.contains(&' ') {
            return Ok((None, board, true));
        }

        if turn == comp {
            let best_move_index = get_best_move(&board, &comp);
            board[best_move_index] = comp.char();
            turn = turn.other();

            loop {
                board_spaces.next();
                if !board.contains(&' ') || board[*board_spaces.value()] == ' ' {
                    break;
                }
            }
            continue;
        }

        // Adding key bindings for home screen
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => loop {
                    board_spaces.next();
                    if board[*board_spaces.value()] == ' ' {
                        break;
                    }
                },
                KeyCode::BackTab => loop {
                    board_spaces.prev();
                    if board[*board_spaces.value()] == ' ' {
                        break;
                    }
                },
                KeyCode::Enter => {
                    board[*board_spaces.value()] = player.char();
                    turn = turn.other();
                }
                KeyCode::Esc => return Ok((None, board, false)),
                // Add other key bindings if necessary
                _ => continue,
            }
        }
    }
}

fn print_screen(board: [char; 9]) {
    println!(
        "
        \r    +-------- TIC TAC TOE ---------+
        \r    |                              |
        \r    |    USE TAB TO MOVE CURSOR    |
        \r    |                              |
        \r    |       ENTER TO SELECT        |
        \r    |                              |
        \r    |                              |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |                              |
        \r    |                              |
        \r    |   PRESS <ESC> TO QUIT GAME   |
        \r    |                              |
        \r    +------------------------------+
        \n\r",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}
