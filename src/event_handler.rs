use crate::utils::DOUBLE_CLICK_DURATION;
use crate::{
    app::{App, AppMode, ClipboardOperation, InputContext},
    commands::{
        Command, CopyCommand, CreateDirCommand, CreateFileCommand, DeleteCommand, MoveCommand,
        RenameCommand,
    },
    ui::HELP_DIALOG,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

// !---------------------
// !  Handle Key Events:
// !---------------------

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<()> {
    match app.mode {
        AppMode::Normal => handle_key_event_normal(key, app),
        AppMode::Help => handle_key_event_help(key, app),
        AppMode::Input => handle_key_event_input(key, app),
        AppMode::Confirm => handle_key_event_confirm(key, app),
        AppMode::MultiSelect => handle_key_event_multi_select(key, app),
        AppMode::Clipboard => handle_key_event_clipboard(key, app),
        AppMode::Command => handle_key_event_command(key, app),
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

        // Command mode:
        KeyCode::Char('/') => {
            app.mode = AppMode::Command;
            app.input_context = Some(InputContext::Command);
            app.input_buffer.clear();
            app.cursor_position = 0;
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

        // Multi-select mode:
        KeyCode::Char('s') => {
            app.mode = AppMode::MultiSelect;
            app.clear_multi_selection();
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

        // View clipboard:
        KeyCode::Char('C') => {
            if !app.clipboard.items.is_empty() {
                app.mode = AppMode::Clipboard;
                app.clipboard_scroll_offset = 0;
            } else {
                app.set_status("Clipboard is empty!".to_string());
            }
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
        // rename:
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

        // delete:
        KeyCode::Char('d') => {
            if let Some(selected) = app.file_list.selected() {
                let selected_path = selected.path.clone();
                app.mode = AppMode::Confirm;
                app.set_status(format!("Delete '{}'? (y/n)", selected.name));
                // store delete command:
                app.active_command = Some(Box::new(DeleteCommand::new_single(selected_path)));
            }
        }

        // cut:
        KeyCode::Char('x') => {
            if let Some(selected) = app.file_list.selected() {
                app.clipboard.items = vec![selected.path.clone()];
                app.clipboard.operation = ClipboardOperation::Cut;
                app.set_status("Cut to clipboard".to_string());
            }
        }

        // copy:
        KeyCode::Char('c') => {
            if let Some(selected) = app.file_list.selected() {
                app.clipboard.items = vec![selected.path.clone()];
                app.clipboard.operation = ClipboardOperation::Copy;
                app.set_status("Copied to clipboard".to_string());
            }
        }

        // paste:
        KeyCode::Char('v') => {
            if !app.clipboard.items.is_empty() {
                let dest_path = app.current_path.clone();
                let clipboard_items = app.clipboard.items.clone();
                let op = app.clipboard.operation.clone();

                // handle clipboard operation types:
                match op {
                    // copys:
                    ClipboardOperation::Copy => {
                        let mut copy_command = CopyCommand::new(clipboard_items, dest_path);
                        if let Err(e) = copy_command.execute(app) {
                            app.set_error(format!("Copy failed: {}", e));
                        }
                    }

                    // cut:
                    ClipboardOperation::Cut => {
                        let mut move_command = MoveCommand::new(clipboard_items, dest_path);
                        if let Err(e) = move_command.execute(app) {
                            app.set_error(format!("Move failed: {}", e));
                        }

                        // clear clipboard after pasting a cut:
                        app.clipboard.items.clear();
                        app.clipboard.operation = ClipboardOperation::None;
                    }

                    _ => {}
                }
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
                            let mut rename_command =
                                RenameCommand::new(selected.path.clone(), input_text);
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
                        let new_dir_path = app.current_path.join(&input_text);
                        let mut create_command = CreateDirCommand::new(new_dir_path);
                        if let Err(e) = create_command.execute(app) {
                            app.set_error(format!("Directory creation failed: {}", e));
                        }
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

pub fn handle_key_event_confirm(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        // confirm yes:
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            // Execute the stored action:
            if let Some(mut command) = app.active_command.take() {
                if let Err(e) = command.execute(app) {
                    app.set_error(format!("Command failed: {}", e));
                }
                app.set_status(format!("Executed Action: {}", command.description()));
                app.mode = AppMode::Normal;
            }
        }

        // confirm no:
        KeyCode::Char('q')
        | KeyCode::Char('Q')
        | KeyCode::Char('n')
        | KeyCode::Char('N')
        | KeyCode::Esc => {
            app.set_status("Cancelled Action".to_string());
            app.active_command = None; // clear stored command
            app.mode = AppMode::Normal;
        }

        _ => {}
    }

    Ok(())
}

pub fn handle_key_event_multi_select(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        // Movement up/down
        KeyCode::Down => {
            app.file_list.next();
        }
        KeyCode::Up => {
            app.file_list.prev();
        }

        // Navigating into/out of directories:
        KeyCode::Left => {
            app.clear_multi_selection();
            app.navigate_up()?;
        }
        KeyCode::Right | KeyCode::Enter => {
            app.clear_multi_selection();
            app.enter_selected()?;
        }

        // Toggle selection for current item:
        KeyCode::Char(' ') => {
            if let Some(selected) = app.file_list.state.selected() {
                if let Some(item) = app.file_list.items.get(selected) {
                    if item.name != ".." {
                        app.toggle_selection();
                    }
                }
            }
        }

        // Delete selection:
        KeyCode::Char('d') => {
            let targets = app
                .selected_items()
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>();
            if !targets.is_empty() {
                app.mode = AppMode::Confirm;
                app.set_status(format!("Delete {} selected item(s)? (y/n)", targets.len()));
                app.active_command = Some(Box::new(DeleteCommand::new(targets)));
            }
        }

        // Copy selection:
        KeyCode::Char('c') => {
            let targets = app
                .selected_items()
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>();
            if !targets.is_empty() {
                app.clipboard.items = targets;
                app.clipboard.operation = ClipboardOperation::Copy;
                app.set_status("Copied selected item(s)".to_string());
            }
        }

        // Cut selection:
        KeyCode::Char('x') => {
            let targets = app
                .selected_items()
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>();
            if !targets.is_empty() {
                app.clipboard.items = targets;
                app.clipboard.operation = ClipboardOperation::Cut;
                app.set_status("Cut selected item(s)".to_string());
            }
        }

        // Exit selection mode:
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('s') => {
            app.mode = AppMode::Normal;
            app.clear_multi_selection();
        }

        _ => {}
    }

    Ok(())
}

pub fn handle_key_event_clipboard(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.mode = AppMode::Normal; // todo: allow viewing clipboard from select mode as well
        }

        KeyCode::Down => {
            let content_length = app.clipboard.items.len() + 3; // add lines for header

            if let Ok((_, terminal_height)) = crossterm::terminal::size() {
                let modal_height = (terminal_height as f32 * 0.6) as usize;
                let viewport_height = modal_height.saturating_sub(2); // account for borders
                app.scroll_clipboard_down(content_length, viewport_height);
            }
        }

        KeyCode::Up => {
            app.scroll_clipboard_up();
        }

        _ => {}
    }

    Ok(())
}

pub fn handle_key_event_command(key: KeyEvent, app: &mut App) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            let command = app.input_buffer.chars().collect::<String>();
            if command.trim() == "help" {
                app.mode = AppMode::Help;
                app.clear_input_buffer();
                return Ok(());
            }
            app.clear_input_buffer();
            app.execute_command(&command)?;
        }

        // Exit the command view:
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            app.clear_input_buffer();
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
    let debounce_ms = 15;
    let now = std::time::Instant::now();
    match mouse.kind {
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            if let Some(last) = app.last_scroll_time {
                if now.duration_since(last).as_millis() < debounce_ms {
                    return Ok(());
                }
            }
            app.last_scroll_time = Some(now);
        }
        _ => {}
    }

    // handle scroll events based on the current app mode:
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            match app.mode {
                AppMode::Help => {
                    let content_length = HELP_DIALOG.len();
                    if let Ok((_, terminal_height)) = crossterm::terminal::size() {
                        let modal_height = (terminal_height as f32 * 0.8) as usize;
                        let viewport_height = modal_height.saturating_sub(2); // account for borders
                        app.scroll_help_down(content_length, viewport_height);
                    }
                }

                AppMode::Clipboard => {
                    let content_length = app.clipboard.items.len() + 4; // account for header lines
                    if let Ok((_, terminal_height)) = crossterm::terminal::size() {
                        let modal_height = (terminal_height as f32 * 0.6) as usize; // clipboard is 60% of terminal height
                        let viewport_height = modal_height.saturating_sub(6); // account for borders
                        app.scroll_clipboard_down(content_length, viewport_height);
                    }
                }

                _ => {
                    app.file_list.prev();
                }
            }
        }

        MouseEventKind::ScrollUp => {
            match app.mode {
                AppMode::Help => {
                    app.scroll_help_up();
                }

                AppMode::Clipboard => {
                    app.scroll_clipboard_up();
                }

                _ => {
                    app.file_list.next();
                }
            }
            return Ok(());
        }

        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            let y = mouse.row.saturating_sub(4) as usize;
            if y < app.file_list.filtered_items.len() {
                let idx = app.file_list.filtered_items[y];

                // Double click detection:
                let double_click = app.last_click_index == Some(idx)
                    && app.last_click_time.map_or(false, |t| {
                        now.duration_since(t).as_millis() < DOUBLE_CLICK_DURATION
                    });

                if double_click {
                    if let Some(item) = app.file_list.items.get(idx) {
                        if item.name == ".." {
                            app.navigate_up()?;
                        } else if item.is_dir {
                            app.enter_selected()?;
                        }
                    }
                    // reset click fsm after double-click action
                    app.last_click_time = None;
                    app.last_click_index = None;
                } else {
                    app.file_list.state.select(Some(idx));
                    app.last_click_index = Some(idx);
                    app.last_click_time = Some(now);
                }
            }
        }

        // MouseEventKind::Up()
        _ => {}
    }

    Ok(())
}
