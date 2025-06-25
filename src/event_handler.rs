
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use anyhow::Result;
use crate::{app::{App, AppMode}, ui::HELP_DIALOG};


// !---------------------
// !  Handle Key Events:
// !---------------------

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<()> {
    match app.mode {
        AppMode::Normal => handle_key_event_normal(key, app),
        AppMode::Help => handle_key_event_help(key, app),
        // todo: implement other modes
        _ => Ok(())
    }
}

pub fn handle_key_event_normal(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        // Navigation within current directory:
        KeyCode::Down => {
            app.file_list.next();
        }
        KeyCode::Up => {
            app.file_list.prev();
        }

        // Navigating into/out of directories:
        KeyCode::Left => {
            app.navigate_up()?;
        }
        KeyCode::Right | KeyCode::Enter => {
            app.enter_selected()?;
        }

        // Display help:
        KeyCode::Char('?') => {
            app.mode = AppMode::Help;
        }


        // Quit:
        KeyCode::Char('q') => {
            app.should_exit = true;
        }
        // allow ctrl+C to exit application too:
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_exit = true;
        }
        _ => {}
    }

    Ok(())
}

pub fn handle_key_event_help(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        // Quit:
        KeyCode::Char('q') | KeyCode::Esc => {
            app.mode = AppMode::Normal;
        }
        // allow ctrl+C to exit application too:
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_exit = true;
        }

        // Scroll down:
        KeyCode::Down => {
            let content_length = HELP_DIALOG.len();
            if let Ok((_, terminal_height)) = crossterm::terminal::size() {
                let modal_height = (terminal_height as f32 * 0.8) as usize;
                let viewport_height = modal_height.saturating_sub(2); // account for borders
                app.scroll_help_down(content_length, viewport_height);
            }
        }

        // Scroll up:
        KeyCode::Up => {
            app.scroll_help_up();
        }


        _ => {}

    }
    Ok(())
}

// !---------------------
// ! Handle Mouse Events:
// !---------------------
pub fn handle_mouse_event(mouse: MouseEvent, app: &mut App) -> Result<()> {
    // pass

    Ok(())
}