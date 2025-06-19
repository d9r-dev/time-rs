mod app;
mod db;
mod ui;

use crate::app::{App, CurrentScreen};
use crate::ui::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use std::error::Error;
use std::io;
use std::io::Stderr;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut terminal, mut app) = initialize_app()?;

    run_app(&mut terminal, &mut app).expect("TODO: panic message");

    // restore terminal
    restore_terminal(&mut terminal);

    Ok(())
}

fn initialize_app() -> Result<(Terminal<CrosstermBackend<Stderr>>, App), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let terminal = Terminal::new(backend)?;

    let mut app = App::new("timers.db");
    app.timers = app.db.get_timers_from_db().expect("Unable to load timers");

    Ok((terminal, app))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stderr>>) {
    disable_raw_mode().expect("Unable to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    terminal.show_cursor().expect("Unable to show cursor");
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_frame = Instant::now();
    let mut time_accumulator = Duration::ZERO;

    loop {
        if let Some(last_timer) = app.timers.last_mut() {
            let now = Instant::now();
            let delta = now - last_frame;
            last_frame = now;
            time_accumulator += delta;
            if time_accumulator >= Duration::from_secs(1) {
                last_timer.tick();
                time_accumulator -= Duration::from_secs(1);
            }
        } else {
            last_frame = Instant::now();
            time_accumulator = Duration::ZERO;
        }

        // why are timers created twice?

        app.db
            .update_timers_in_db(&app.timers)
            .expect("Unable to update timers");
        terminal.draw(|f| ui(f, app))?;
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exit;
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            app.current_screen = CurrentScreen::Exit;
                        }
                        KeyCode::Char('j') => {
                            app.next_row();
                        }
                        KeyCode::Char('k') => {
                            app.previous_row();
                        }
                        KeyCode::Char(' ') => {
                            app.current_screen = CurrentScreen::Add;
                            app.currently_editing = Some(app::CurrentlyEditing::Name);
                        }
                        _ => {}
                    },
                    CurrentScreen::Exit => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            return Ok(());
                        }
                        KeyCode::Char('y') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    CurrentScreen::Add if key.kind == event::KeyEventKind::Press => {
                        match key.code {
                            KeyCode::Enter => {
                                if let Some(edit_mode) = &app.currently_editing {
                                    match edit_mode {
                                        app::CurrentlyEditing::Name => {
                                            app.currently_editing =
                                                Some(app::CurrentlyEditing::Description)
                                        }
                                        app::CurrentlyEditing::Description => {
                                            app.add_timer();
                                            app.current_screen = CurrentScreen::Main
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                if let Some(edit_mode) = &app.currently_editing {
                                    match edit_mode {
                                        app::CurrentlyEditing::Name => {
                                            app.name_input.pop();
                                        }
                                        app::CurrentlyEditing::Description => {
                                            app.description_input.pop();
                                        }
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                app.current_screen = CurrentScreen::Main;
                                app.currently_editing = None;
                            }
                            KeyCode::Tab => {
                                app.toggle_editing();
                            }
                            KeyCode::Char(c) => {
                                if let Some(edit_mode) = &app.currently_editing {
                                    match edit_mode {
                                        app::CurrentlyEditing::Name => {
                                            app.name_input.push(c);
                                        }
                                        app::CurrentlyEditing::Description => {
                                            app.description_input.push(c);
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    CurrentScreen::Add => {}
                }
            }
        } else {
            continue;
        }
    }
}
