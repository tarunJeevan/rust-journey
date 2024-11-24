use crate::models::{Player, Tabs};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub fn end_menu(winner: &Option<Player>, human: &Player, board: &[char; 9]) -> io::Result<bool> {
    let mut output = io::stdout();

    let mut options = Tabs::new(vec![(16, 15, ("RESTART", true)), (17, 17, ("QUIT", false))]);

    loop {
        // Clear screen
        execute!(
            output,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan)
        )?;

        print_screen(*board);

        let (statement, color) = match winner {
            Some(winner) => {
                if winner == human {
                    ("YOU WIN!", Color::Green)
                } else {
                    ("YOU LOSE!", Color::Red)
                }
            },
            None => ("IT'S A TIE!", Color::Yellow)
        };

        execute!(
            output,
            SetForegroundColor(color),
            cursor::MoveTo(14, 4),
            Print(statement),
            ResetColor
        )?;

        // Highlight selected tab
        execute!(
            output,
            cursor::MoveTo(options.position().0, options.position().1),
            SetBackgroundColor(Color::Red),
            Print(options.value().0),
            ResetColor,
            cursor::MoveTo(options.position().0, options.position().1),
        )?;

        output.flush()?;

        // Adding key bindings for home screen
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => options.next(),
                KeyCode::BackTab => options.prev(),
                KeyCode::Enter => return Ok(options.value().1),
                KeyCode::Esc => return Ok(false),
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
        \r    |   +----------------------+   |
        \r    |   |                      |   |
        \r    |   +----------------------+   |
        \r    |                              |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |        | {} | {} | {} |         |
        \r    |        +---+---+---+         |
        \r    |                              |
        \r    |          <RESTART>           |
        \r    |                              |
        \r    |           <QUIT>             |
        \r    |                              |
        \r    +------------------------------+
        \n\r",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}
