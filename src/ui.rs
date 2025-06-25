use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
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
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let path_text = format!(" 🗂️ {}", app.current_path.display());
    let mode_text = format!(" {} ", "NORMAL"); // todo: display different app modes

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

    // todo: idea: highlight mode box with colours?
    // for now leave default:
    let mode_style = Style::default().fg(Color::Green);

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
    // pass
    // todo: remove / replace the following:
    let paragraph = Paragraph::new("placeholder\nfiles go here")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(paragraph, area);
}

fn draw_info_panel(f: &mut Frame, area: Rect, app: &App) {
    // pass
    // todo: remove / replace the following:
    let paragraph = Paragraph::new("placeholder\ninfo about currently selected\nfile goes here")
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(paragraph, area);
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
