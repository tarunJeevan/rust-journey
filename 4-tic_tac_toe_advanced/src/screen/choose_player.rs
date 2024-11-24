use crate::models::{Player, Tabs};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub fn choose_player() -> io::Result<(Player, bool)> {
    let mut output = io::stdout();

    let mut x_or_o = Tabs::new(vec![(17, 13, Player::X), (22, 13, Player::O)]);

    loop {
        // Clear screen
        execute!(
            output,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan)
        )?;

        print_screen();
        
        // Highlight selected tab
        execute!(
            output,
            cursor::MoveTo(x_or_o.position().0, x_or_o.position().1),
            SetBackgroundColor(Color::Red),
            Print(x_or_o.value().char()),
            ResetColor,
            cursor::MoveTo(x_or_o.position().0, x_or_o.position().1),
        )?;

        output.flush()?;

        // Adding key bindings for home screen
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => x_or_o.next(),
                KeyCode::BackTab => x_or_o.prev(),
                KeyCode::Enter => return Ok((x_or_o.value().clone(), true)),
                KeyCode::Esc => return Ok((x_or_o.value().clone(), false)),
                // Add other key bindings if necessary
                _ => continue,
            }
        }
    }
}

fn print_screen() {
    println!(
        "
        \r    +-------- TIC TAC TOE ---------+
        \r    |                              |
        \r    |    USE TAB TO MOVE CURSOR    |
        \r    |                              |
        \r    |       ENTER TO SELECT        |
        \r    |                              |
        \r    |                              |
        \r    |   +----------------------+   |
        \r    |   |  CHOOSE YOUR PLAYER  |   |
        \r    |   +----------------------+   |
        \r    |                              |
        \r    |                              |
        \r    |           <X>  <O>           |
        \r    |                              |
        \r    |                              |
        \r    |   PRESS <ESC> TO QUIT GAME   |
        \r    |                              |
        \r    +------------------------------+
        \n\r
        "
    );
}
