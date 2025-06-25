use std::{
    io,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};

mod app;
mod ui;
mod event_handler;
mod utils;

use app::App;

use crate::event_handler::{handle_key_event, handle_mouse_event};

fn main() -> Result<()> {
    // Setup error handling:
    color_eyre::install().unwrap();

    // Setup terminal:
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap(); // ?
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // initialise app:
    let mut app = App::new()?;

    // run the app:
    let result = run_app(&mut terminal, &mut app);

    // after app finishes running:
    // restore original terminal:
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle errors that occurred during app execution:
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    // Main app event loop:
    loop {
        // Draw the UI:
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events:
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key_event) => {
                    // only handle press events:
                    if key_event.kind == crossterm::event::KeyEventKind::Press {
                        // todo: process event through event handler
                        if let Err(e) = handle_key_event(key_event, app) {
                            app.set_error(format!("Key Event Error: {}", e));
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    // todo: process event through event handler
                    if let Err(e) = handle_mouse_event(mouse_event, app) {
                        app.set_error(format!("Mouse Event Error: {}", e));
                    }
                }
                _ => {}
            }
        }

        // periodic update for background tasks
        if last_tick.elapsed() >= tick_rate {
            // todo
        }

        if app.should_exit {
            break;
        }
    }

    Ok(())
}
