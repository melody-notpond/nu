use std::{io::{self, Error}, time::Duration};

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, self}, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend, Terminal, widgets, layout, text::{Span, Spans}};

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    terminal.clear()?;

    let mut running = true;

    while running {
        if let Ok(true) = crossterm::event::poll(Duration::from_millis(10)) {
            if let Ok(event) = crossterm::event::read() {
                match event {
                    crossterm::event::Event::Key(_) => running = false,

                    crossterm::event::Event::Mouse(_) => (),

                    crossterm::event::Event::Resize(_, _) => (),
                }
            }
        }

        terminal.draw(|f| {
            let size = f.size();
            let vertical = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .constraints([
                    layout::Constraint::Min(1),
                    layout::Constraint::Length(2),
                ])
                .split(size);
            let horizontal = layout::Layout::default()
                .direction(layout::Direction::Horizontal)
                .constraints([
                    layout::Constraint::Length(3),
                    layout::Constraint::Length(1),
                    layout::Constraint::Min(1),
                ])
                .split(vertical[0]);
            let text_field = widgets::Block::default();
            let text_field = widgets::Paragraph::new(vec![Spans::from(vec![Span::raw("uwu")])])
                .block(text_field)
                .alignment(layout::Alignment::Left);
            f.render_widget(text_field, horizontal[2]);
            let line_numbers = widgets::Block::default()
                .borders(widgets::Borders::RIGHT);
            let line_numbers = widgets::Paragraph::new(vec![Spans::from(vec![Span::raw("1")]), Spans::from(vec![Span::raw("2")]), Spans::from(vec![Span::raw("3")]), Spans::from(vec![Span::raw("4")])])
                .block(line_numbers)
                .alignment(layout::Alignment::Right);
            f.render_widget(line_numbers, horizontal[0]);
        })?;
    }

    terminal.clear()?;
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
