use crate::{
    app::{App, AppMode, ClipboardOperation, InputContext},
    utils::{format_size, format_time, get_file_icon, truncate_string},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header bar
            Constraint::Min(0),    // Main content view
            Constraint::Length(3), // Status bar
        ])
        .split(f.size());

    // Draw header:
    draw_header(f, chunks[0], app);

    // Draw Main content view:
    draw_main_content(f, chunks[1], app);

    // Draw status bar:
    draw_status_bar(f, chunks[2], app);

    // Draw additional dialog, modals, etc:
    match app.mode {
        AppMode::Help => {
            draw_help_modal(f, app);
        }
        AppMode::Input => {
            draw_input_modal(f, app);
        }
        AppMode::Confirm => {
            draw_confirm_modal(f, app);
        }

        AppMode::Clipboard => {
            draw_clipboard_modal(f, app);
        }

        AppMode::Command => {
            draw_input_modal(f, app);
        }

        _ => {}
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let path_text = format!(" 🗂️ {}", app.current_path.display());
    let mode_text = format!(
        " {} ",
        match app.mode {
            AppMode::Normal => "NORMAL",
            AppMode::Help => "HELP",
            AppMode::MultiSelect => "SELECT",
            AppMode::Input => "INPUT",
            AppMode::Command => "COMMAND",
            AppMode::Confirm => "CONFIRM",
            AppMode::Clipboard => "CLIPBOARD",
        }
    );

    // get area chunks for header:
    let header_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length((mode_text.len() + 2) as u16), // +2 to account for the borders
        ])
        .split(area);

    // display the path:
    let path_paragraph = Paragraph::new(path_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Clexp - Command Line Explorer"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(path_paragraph, header_chunks[0]);

    let mode_style = match app.mode {
        AppMode::Normal => Style::default().fg(Color::Green),
        AppMode::Help => Style::default().fg(Color::Magenta),
        AppMode::MultiSelect => Style::default().fg(Color::Yellow),
        AppMode::Input => Style::default().fg(Color::Blue),
        AppMode::Command => Style::default().fg(Color::Cyan),
        AppMode::Confirm => Style::default().fg(Color::Red),
        AppMode::Clipboard => Style::default().fg(Color::LightGreen),
    };

    let mode_paragraph = Paragraph::new(mode_text)
        .style(mode_style.add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(mode_paragraph, header_chunks[1]);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    let main_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // draw files:
    draw_files_list(f, main_chunks[0], app);

    // draw info side panel
    draw_info_panel(f, main_chunks[1], app);
}

fn draw_files_list(f: &mut Frame, area: Rect, app: &App) {
    let multi_select_mode = app.mode == AppMode::MultiSelect;
    // todo more selection mode stuff
    let selected_indices = &app.selection;
    let items: Vec<ListItem> = app
        .file_list
        .filtered_items()
        .iter()
        .enumerate()
        .map(|(_idx, item)| {
            let icon = get_file_icon(&item.name, item.is_dir);
            let size_text = if let Some(size) = item.size {
                format_size(size)
            } else {
                "-".to_string()
            };
            let name_width = if multi_select_mode {
                area.width.saturating_sub(24) as usize // wider to display checkbox
            } else {
                area.width.saturating_sub(20) as usize
            };

            let display_name = truncate_string(&item.display_name(), name_width);
            let is_selected = app
                .file_list
                .items
                .iter()
                .position(|f| f.name == item.name && f.path == item.path)
                .map_or(false, |i| selected_indices.contains(&i));
            let checkbox = if multi_select_mode {
                if is_selected { "[x]" } else { "[ ]" }
            } else {
                ""
            };

            let content = if multi_select_mode {
                format!(
                    "{} {} {:<width$} {:>8}",
                    checkbox,
                    icon,
                    display_name,
                    size_text,
                    width = name_width
                )
            } else {
                format!(
                    "{} {:<width$} {:>8}",
                    icon,
                    display_name,
                    size_text,
                    width = name_width
                )
            };

            let mut style = if item.is_dir {
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            if multi_select_mode && is_selected {
                style = style.bg(Color::Yellow).fg(Color::Black);
            }
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let title = if multi_select_mode {
        // todo: fix this when filtering is applied (not implemented yet)
        format!(
            " Files ({}) [{} items selected]",
            app.file_list.items.len(),
            app.selection.len()
        )
    } else {
        format!(" Files ({}) ", app.file_list.items.len())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, area, &mut app.file_list.state.clone());
}

fn draw_info_panel(f: &mut Frame, area: Rect, app: &App) {
    let content = if let Some(selected_item) = app.file_list.selected() {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&selected_item.name),
            ]),
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if selected_item.is_dir {
                    "Directory"
                } else {
                    "File"
                }),
            ]),
        ];

        if let Some(size) = selected_item.size {
            lines.push(Line::from(vec![
                Span::styled("Size: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format_size(size)),
            ]));
        }

        if let Some(modified) = selected_item.modified {
            lines.push(Line::from(vec![
                Span::styled("Modified: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format_time(modified)),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "Path: ",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(Span::raw(
            selected_item.path.display().to_string(),
        )));

        Text::from(lines)
    } else {
        Text::from("No file selected")
    };
    let details = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Details"),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(details, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let status_bar_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // left side = status / error messages:
    let status_text = if let Some(error) = &app.error_message {
        format!(" ERROR: {}", error)
    } else if let Some(status_message) = &app.status_message {
        format!(" {}", status_message)
    } else {
        " Ready".to_string()
    };
    let status_style = if app.error_message.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Blue)
    };
    let status_paragraph = Paragraph::new(status_text).style(status_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(status_paragraph, status_bar_chunks[0]);

    // right side = usage hints
    let hints = "q:quit  /:search ::run cmd";
    // todo: show different hints based on app mode
    let hints_style = Style::default().fg(Color::White);
    let hints_paragraph = Paragraph::new(hints)
        .style(hints_style)
        .alignment(ratatui::layout::Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(hints_paragraph, status_bar_chunks[1]);
}

fn draw_help_modal(f: &mut Frame, app: &App) {
    // show a clear rectangle
    let area = centered_rect(80, 80, f.size());
    f.render_widget(Clear, area);

    let height = area.height.saturating_sub(2) as usize; // account for borders
    let max_offset = if HELP_DIALOG.len() > height {
        HELP_DIALOG.len() - height
    } else {
        0
    };
    let offset = app.help_scroll_offset.min(max_offset);
    let visible_lines = &HELP_DIALOG[offset..HELP_DIALOG.len().min(offset + height)];
    let help_text = visible_lines.join("\n");

    // todo: add a close button for mouse support eventually

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Help Manual ")
                .padding(Padding {
                    left: 2,
                    right: 0,
                    top: 1,
                    bottom: 0,
                }),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_input_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);

    // determine the title based on input context:
    let title = match app.input_context {
        Some(InputContext::Rename) => "Rename File/Directory",
        Some(InputContext::CreateFile) => "Create New File",
        Some(InputContext::CreateDir) => "Create New Directory",
        Some(InputContext::Filter) => "Filter Files",
        Some(InputContext::Command) => "Command Mode",
        None => "Input",
    };

    let input_text = &app.input_buffer;
    let cursor_pos = app.cursor_position;
    let before_cursor = &input_text[..cursor_pos.min(input_text.len())];
    let after_cursor = &input_text[cursor_pos.min(input_text.len())..];
    let text_with_cursor = format!("{}█{}", before_cursor, after_cursor);
    let paragraph = Paragraph::new(text_with_cursor)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(title),
        );
    f.render_widget(paragraph, area);
}

fn draw_confirm_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 25, f.size());
    f.render_widget(Clear, area);

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Red));
    f.render_widget(outer_block, area);

    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let text = app.status_message.as_deref().unwrap_or("Confirm action?");
    let disclaimer = "Press Y to confirm, N to cancel";
    let text = format!("{}\n\n{}", text, disclaimer);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Confirm ")
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 1,
                }),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
}

fn draw_clipboard_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 60, f.size());
    f.render_widget(Clear, area);

    // create lines to display to user:
    let op = match app.clipboard.operation {
        ClipboardOperation::Copy => "Copy",
        ClipboardOperation::Cut => "Cut",
        _ => "Clipboard",
    };
    let mut lines = vec![format!("Clipboard Operation: {}", op), "".to_string()];
    if app.clipboard.items.is_empty() {
        lines.push("Clipboard is empty".to_string()); // app should never reach this state 
    } else {
        lines.push("Items:".to_string());
        for path in &app.clipboard.items {
            lines.push(format!(" - {}", path.display()));
        }
    }

    // display lines:
    let height = area.height.saturating_sub(2) as usize;
    let max_offset = if lines.len() > height {
        lines.len() - height
    } else {
        0
    };
    let offset = app.clipboard_scroll_offset.min(max_offset);
    let visible_lines = &lines[offset..lines.len().min(offset + height)];
    let text = visible_lines.join("\n");
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Clipboard ")
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 1,
                }),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

// UI-specific helper functions:
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// Consts:
pub static HELP_DIALOG: [&str; 43] = [
    "Clexp Quick Help",
    // todo: "For more help, see documentation at XYZ"
    "",
    // Navigation:
    "Navigation:",
    "  ↑↓              Move selection",
    "  ←               Up one directory",
    "  →, Enter        Enter Directory/Open File",
    "  q, Ctrl+C       Quit",
    "  :               Filter files",
    "  /               Enter command",
    "  ?, :help        Show this help",
    "  C               Show clipboard",
    "",
    "",
    // File operations:
    "File operations:",
    "  r               Rename selected file/dir",
    "  d               Delete selected file(s)/dir(s)",
    "  x               Cut selected file(s)/dir(s)",
    "  c               Copy selected file(s)/dir(s)",
    "  v               Paste selected file(s)/dir(s)",
    "  n               New file",
    "  N               New directory",
    "",
    "",
    // Modes:
    "Modes:",
    "  s               Multi-Select Mode",
    "  Space           Toggle selection in Multi-Select Mode",
    "  Esc, s          Exit Multi-Select Mode",
    "  [x]             Indicates selected files in Multi-Select Mode",
    "",
    "",
    // Mouse controls (not implemented yet)
    "Mouse Controls: (not implemented yet)",
    "  Click file      Select (or toggle in select mode)",
    "  Click folder    Select / Open directory (toggles in select mode)",
    "  Click ..        Go up a directory",
    "  Scroll wheel    Move selection up/down (or scroll modals)",
    "",
    "",
    // Commands:
    "Command Mode:",
    "  :q              Quit",
    "  :s <term>       Filter View",
    "  :h or :help     Show this help",
    "",
    "",
    //todo allow user to create own commands? need to think about how to store commands between program instances
];
