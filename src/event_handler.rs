
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use anyhow::Result;
use crate::{app::{App, AppMode, InputContext}, commands::{Command, CreateFileCommand, RenameCommand}, ui::HELP_DIALOG};


// !---------------------
// !  Handle Key Events:
// !---------------------

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<()> {
    match app.mode {
        AppMode::Normal => handle_key_event_normal(key, app),
        AppMode::Help => handle_key_event_help(key, app),
        AppMode::Input => handle_key_event_input(key, app),
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

        // Clear messages:
        KeyCode::Esc => {
            app.clear_messages();
        }

        // Create new file:
        KeyCode::Char('n') => {
            app.mode = AppMode::Input;
            app.input_context = Some(InputContext::CreateFile);
            app.clear_input_buffer();
            app.set_status("Create new file: ".to_string());
        }

        // Create new directory:
        KeyCode::Char('N') => {
            app.mode = AppMode::Input;
            app.input_context = Some(InputContext::CreateDir);
            app.clear_input_buffer();
            app.set_status("Create new directory: ".to_string());
        }



        // Quit:
        KeyCode::Char('q') => {
            app.should_exit = true;
        }
        // allow ctrl+C to exit application too:
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_exit = true;
        }


        // Quick actions:
        KeyCode::Char('r') => {
            if let Some(selected) = app.file_list.selected() {
                app.mode = AppMode::Input;
                app.input_buffer.clear();
                app.input_buffer.push_str(&selected.name);
                app.cursor_position = selected.name.len(); // position cursor at the end
                app.input_context = Some(InputContext::Rename);
                app.set_status("Rename to: ".to_string());
            }
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


pub fn handle_key_event_input(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            // handle execution based on input context
            if !app.input_buffer.is_empty() {
                let input_text = app.input_buffer.clone();
                match app.input_context {
                    // rename:
                    Some(InputContext::Rename) => {
                        if let Some(selected) = app.file_list.selected() {
                            let mut rename_command = RenameCommand::new(selected.path.clone(), input_text);
                            if let Err(e) = rename_command.execute(app) {
                                app.set_error(format!("Rename failed: {}", e));
                            } else {
                                if let Err(e) = app.refresh_file_list() {
                                    app.set_error(format!("Failed to refresh after rename: {}", e));
                                }
                            }
                        }
                    }

                    // create file:
                    Some(InputContext::CreateFile) => {
                        let new_file_path = app.current_path.join(&input_text);
                        let mut create_command = CreateFileCommand::new(new_file_path);
                        if let Err(e) = create_command.execute(app) {
                            app.set_error(format!("File creation failed: {}", e));
                        }
                    }

                    // create directory:
                    Some(InputContext::CreateDir) => {

                    }
                    //todo: implement the rest of the commands:

                    _ => {}
                }
                app.mode = AppMode::Normal;
                app.input_context = None;
                app.clear_input_buffer();
            }
        }

        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            app.input_context = None;
            app.clear_input_buffer();
            app.clear_messages();
        }

        // Cursor movement:
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Up | KeyCode::Home => {
            app.move_cursor_home();
        }
        KeyCode::Down | KeyCode::End => {
            app.move_cursor_end();
        }

        // Text editing:
        KeyCode::Backspace => {
            app.delete_char_before_cursor();
        }
        KeyCode::Delete => {
            app.delete_char_at_cursor();
        }
        // Text input:
        KeyCode::Char(c) => {
            app.insert_char_at_cursor(c);
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