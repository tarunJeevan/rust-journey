use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use tic_tac_toe_advanced::screen;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut output = io::stdout();

    loop {
        // Clear screen
        execute!(
            output,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Clear(ClearType::Purge)
        )?;

        output.flush()?;

        let (player, continue_game) = screen::choose_player()?;

        if !continue_game {
            break;
        }

        let (winner, board, continue_game) = screen::gameplay(&player)?;

        if !continue_game {
            break;
        }
        
        let continue_game = screen::end_menu(&winner, &player, &board)?;
        
        if !continue_game {
            break;
        }
    }

    // Clear screen
    execute!(
        output,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge)
    )?;

    output.flush()?;

    terminal::disable_raw_mode()?;
    
    Ok(())
}
