use std::{
    fs,
    io::{self, Error},
    time::Duration,
};

use crossterm::{
    cursor::{CursorShape, SetCursorShape},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use nu::buffer::{Buffer, Buffers};
use tui::{
    backend::CrosstermBackend,
    layout,
    text::{Span, Spans},
    widgets, Terminal,
};

enum Mode {
    Normal,
    Command,
    Insert,
}

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut stdout = io::stdout();
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    terminal.clear()?;

    let mut running = true;
    let mut mode = Mode::Normal;
    let mut editor_command = String::new();
    let mut buffers = Buffers::new(Buffer::new("[buffer]", false, ""));
    let mut message = None;

    while running {
        if let Ok(true) = crossterm::event::poll(Duration::from_millis(10)) {
            if let Ok(event) = crossterm::event::read() {
                match event {
                    crossterm::event::Event::Key(key) => match mode {
                        Mode::Normal => match key.code {
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

                            KeyCode::Char('i') => {
                                mode = Mode::Insert;
                            }

                            KeyCode::Char('h') => {
                                buffers.get_current_mut().move_left();
                            }

                            KeyCode::Char('j') => {
                                buffers.get_current_mut().move_down();
                            }

                            KeyCode::Char('k') => {
                                buffers.get_current_mut().move_up();
                            }

                            KeyCode::Char('l') => {
                                buffers.get_current_mut().move_right();
                            }

                            KeyCode::Char('[') => {
                                buffers.prev();
                            }

                            KeyCode::Char(']') => {
                                buffers.next();
                            }

                            KeyCode::Char(_) => (),
                            KeyCode::Null => (),
                            KeyCode::Esc => (),
                        },

                        Mode::Command => match key.code {
                            KeyCode::Backspace => {
                                if editor_command.pop().is_none() {
                                    mode = Mode::Normal;
                                }
                            }

                            KeyCode::Enter => {
                                mode = Mode::Normal;
                                message = None;
                                let args: Vec<_> = editor_command.split_whitespace().collect();
                                match args.first().cloned() {
                                    Some("quit" | "q") => {
                                        if let Some(buffer) = buffers.modified() {
                                            message = Some(format!(
                                                "Cannot quit: unsaved buffer `{}`",
                                                buffer.name
                                            ));
                                        } else {
                                            running = false;
                                        }
                                    }

                                    Some("quit!" | "q!") => {
                                        running = false;
                                    }

                                    Some("close" | "c") => {
                                        if buffers.get_current().modified {
                                            message = Some(format!("Cannot close unsaved buffer `{}`", buffers.get_current().name));
                                        } else {
                                            buffers.remove_current();
                                        }
                                    }

                                    Some("close!" | "c!") => {
                                        buffers.remove_current();
                                    }

                                    Some("new" | "n") => {
                                        if args.len() > 2 {
                                            message = Some(String::from(
                                                "`new` takes in at most 1 argument",
                                            ))
                                        } else {
                                            let mut buffer = Buffer::new("[buffer]", false, "");
                                            if args.len() == 2 {
                                                buffer.name = String::from(args[1]);
                                                buffer.is_file = true;
                                            }

                                            let id = buffers.add_buffer(buffer);
                                            buffers.switch(id);
                                        }
                                    }

                                    Some("write" | "w") => {
                                        let buffer = buffers.get_current_mut();
                                        if args.len() > 2 {
                                            message = Some(String::from(
                                                "`write` takes in at most 1 argument",
                                            ))
                                        } else {
                                            if args.len() == 2 {
                                                buffer.name = String::from(args[1]);
                                                buffer.is_file = true;
                                            }

                                            if buffer.is_file {
                                                match fs::write(&buffer.name, buffer.to_string()) {
                                                    Ok(_) => {
                                                        message = Some(format!(
                                                            "Saved file `{}`",
                                                            buffer.name
                                                        ));
                                                        buffer.modified = false;
                                                    }

                                                    Err(e) => {
                                                        message = Some(format!(
                                                            "Could not save file `{}`: {}",
                                                            buffer.name, e
                                                        ))
                                                    }
                                                }
                                            } else {
                                                message = Some(format!(
                                                    "Cannot save nonfile buffer `{}`",
                                                    buffer.name
                                                ));
                                            }
                                        }
                                    }

                                    Some(v) => {
                                        message = Some(format!("`{}` is not a valid command", v));
                                    }

                                    None => (),
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
                        },

                        Mode::Insert => match key.code {
                            KeyCode::Backspace => {
                                buffers.get_current_mut().backspace();
                            }

                            KeyCode::Enter => {
                                buffers.get_current_mut().enter();
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
                                buffers.get_current_mut().char(c)
                            }

                            KeyCode::Null => (),

                            KeyCode::Esc => {
                                mode = Mode::Normal;
                            }
                        },
                    },

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
                .constraints([layout::Constraint::Min(1), layout::Constraint::Length(3)])
                .split(size);
            let horizontal = layout::Layout::default()
                .direction(layout::Direction::Horizontal)
                .constraints([
                    layout::Constraint::Length(
                        1 + ((buffers.get_current().vscroll + vertical[0].height as usize + 1) as f64)
                            .log10()
                            .ceil() as u16,
                    ),
                    layout::Constraint::Length(1),
                    layout::Constraint::Min(1),
                ])
                .split(vertical[0]);

            buffers.get_current_mut().update_scrolls(horizontal[2].width as isize, horizontal[2].height as isize);
            let buffer = buffers.get_current();

            let text_field = widgets::Paragraph::new(
                buffer.window(horizontal[2].width as usize, horizontal[2].height as usize)
                    .map(|v| Spans::from(v.into_iter().map(Span::raw).collect::<Vec<_>>()))
                    .collect::<Vec<_>>())
            .alignment(layout::Alignment::Left);
            f.render_widget(text_field, horizontal[2]);

            let line_numbers = widgets::Block::default().borders(widgets::Borders::RIGHT);
            let line_numbers = widgets::Paragraph::new(
                (buffer.vscroll + 1..buffer.vscroll + horizontal[0].height as usize + 1)
                    .map(|v| Spans::from(vec![Span::raw(format!("{}", v))]))
                    .collect::<Vec<_>>(),
            )
            .block(line_numbers)
            .alignment(layout::Alignment::Right);
            f.render_widget(line_numbers, horizontal[0]);

            let command = widgets::Block::default().borders(widgets::Borders::TOP);
            let mut command_data = vec![Spans::from(vec![
                Span::raw(&buffer.name),
                if buffer.modified {
                    Span::raw(" [+]")
                } else {
                    Span::raw("")
                },
            ])];
            if let Mode::Command = mode {
                command_data.push(Spans::from(vec![
                    Span::raw(":"),
                    Span::raw(&editor_command),
                ]));
            } else if let Some(message) = message.as_ref() {
                command_data.push(Spans::from(vec![Span::raw(message)]));
            }
            let command = widgets::Paragraph::new(command_data)
                .alignment(layout::Alignment::Left)
                .block(command);
            f.render_widget(command, vertical[1]);

            if let Mode::Insert = mode {
                execute!(stdout, SetCursorShape(CursorShape::Line))
                    .expect("could not set cursor shape");
                let (x, y) = buffer.cursor_pos(horizontal[2].x as usize, horizontal[2].y as usize);
                f.set_cursor(x as u16, y as u16);
            } else if let Mode::Normal = mode {
                execute!(stdout, SetCursorShape(CursorShape::Block))
                    .expect("could not set cursor shape");
                let (x, y) = buffer.cursor_pos(horizontal[2].x as usize, horizontal[2].y as usize);
                f.set_cursor(x as u16, y as u16);
            }
        })?;
    }

    terminal.clear()?;
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
