use std::{io::{self, Error}, time::Duration};

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, self}, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend, Terminal, widgets};

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = widgets::Block::default()
            .title("uwu")
            .borders(widgets::Borders::ALL);
        f.render_widget(block, size);
    })?;

    std::thread::sleep(Duration::from_secs(2));

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
