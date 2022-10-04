use std::{io::{self, Error}, time::Duration};

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, self}, event::{EnableMouseCapture, DisableMouseCapture, KeyCode}};
use tui::{backend::CrosstermBackend, Terminal, widgets, layout, text::{Span, Spans}};

enum Mode {
    Normal,
    Command,
    Insert,
    Replace
}

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    terminal.clear()?;

    let mut running = true;
    let mut mode = Mode::Normal;
    let mut editor_command = String::new();

    while running {
        if let Ok(true) = crossterm::event::poll(Duration::from_millis(10)) {
            if let Ok(event) = crossterm::event::read() {
                match event {
                    crossterm::event::Event::Key(key) => {
                        match mode {
                            Mode::Normal => {
                                match key.code {
                                    KeyCode::Backspace => (),
                                    KeyCode::Enter => (),
                                    KeyCode::Left => (),
                                    KeyCode::Right => (),
                                    KeyCode::Up => (),
                                    KeyCode::Down => (),
                                    KeyCode::Home => (),
                                    KeyCode::End => (),
                                    KeyCode::PageUp => (),
                                    KeyCode::PageDown => (),
                                    KeyCode::Tab => (),
                                    KeyCode::BackTab => (),
                                    KeyCode::Delete => (),
                                    KeyCode::Insert => (),
                                    KeyCode::F(_) => (),

                                    KeyCode::Char(':') => {
                                        mode = Mode::Command;
                                        editor_command.clear();
                                    }

                                    KeyCode::Char(_) => (),
                                    KeyCode::Null => (),
                                    KeyCode::Esc => (),
                                }
                            }

                            Mode::Command => {
                                match key.code {
                                    KeyCode::Backspace => {
                                        editor_command.pop();
                                    }

                                    KeyCode::Enter => {
                                        mode = Mode::Normal;
                                        match editor_command.as_str() {
                                            "quit" | "q" => {
                                                running = false;
                                            }

                                            _ => (),
                                        }
                                    }

                                    KeyCode::Left => (),
                                    KeyCode::Right => (),
                                    KeyCode::Up => (),
                                    KeyCode::Down => (),
                                    KeyCode::Home => (),
                                    KeyCode::End => (),
                                    KeyCode::PageUp => (),
                                    KeyCode::PageDown => (),
                                    KeyCode::Tab => (),
                                    KeyCode::BackTab => (),
                                    KeyCode::Delete => (),
                                    KeyCode::Insert => (),
                                    KeyCode::F(_) => (),

                                    KeyCode::Char(c) => {
                                        editor_command.push(c);
                                    }

                                    KeyCode::Null => (),

                                    KeyCode::Esc => {
                                        mode = Mode::Normal;
                                    }
                                }
                            }

                            Mode::Insert => (),
                            Mode::Replace => (),
                        }
                    }

                    // TODO: mouse stuff
                    crossterm::event::Event::Mouse(_) => (),

                    // We can ignore resize stuff for now at least
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
                    layout::Constraint::Length(3),
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

            let text_field = widgets::Paragraph::new(vec![Spans::from(vec![Span::raw("uwu")])])
                .alignment(layout::Alignment::Left);
            f.render_widget(text_field, horizontal[2]);

            let line_numbers = widgets::Block::default()
                .borders(widgets::Borders::RIGHT);
            let line_numbers = widgets::Paragraph::new(vec![Spans::from(vec![Span::raw("1")]), Spans::from(vec![Span::raw("2")]), Spans::from(vec![Span::raw("3")]), Spans::from(vec![Span::raw("4")])])
                .block(line_numbers)
                .alignment(layout::Alignment::Right);
            f.render_widget(line_numbers, horizontal[0]);

            let command = widgets::Block::default()
                .borders(widgets::Borders::TOP);
            let mut command_data = vec![Spans::from(vec![Span::raw("[buffer]")])];
            if let Mode::Command = mode {
                command_data.push(Spans::from(vec![Span::raw(":"), Span::raw(&editor_command)]));
            }
            let command = widgets::Paragraph::new(command_data)
                .alignment(layout::Alignment::Left)
                .block(command);
            f.render_widget(command, vertical[1]);
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
