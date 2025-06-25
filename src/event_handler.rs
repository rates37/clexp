
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use anyhow::Result;
use crate::app::{App, AppMode};


// !---------------------
// !  Handle Key Events:
// !---------------------

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<()> {
    match app.mode {
        AppMode::Normal => handle_key_event_normal(key, app),
        // todo: implement other modes
        _ => Ok(())
    }
}

pub fn handle_key_event_normal(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        // Navigation:
        KeyCode::Down => {
            app.file_list.next();
        }
        KeyCode::Up => {
            app.file_list.prev();
        }


        // Quit:
        KeyCode::Char('q') => {
            app.should_exit = true;
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