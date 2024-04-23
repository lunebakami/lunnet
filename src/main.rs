mod app;
mod ui;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dbus::blocking::Connection;
use networkmanager::NetworkManager;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::error::Error;
use std::io;

use crate::{
    app::{App, CurrentScreen},
    ui::ui::render,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(f, app))?;

        match app.current_screen {
            CurrentScreen::Exiting => return Ok(()),
            _ => {}
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;

    // stderr here is a special case.
    // Normaly using stdout is fine
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app an run it
    let dbus_connection = Connection::new_system()?;
    let nm = NetworkManager::new(&dbus_connection);
    let app = App::new(nm);

    match app {
        Ok(mut app) => {
            let _ = run_app(&mut terminal, &mut app);
        }
        Err(_) => {},
    };

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
