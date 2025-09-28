use std::os::unix::raw::time_t;

use lopdf::content::Operation;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::tui::app::{App, CurrentScreen, OperationMode};

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Main => draw_main_screen(frame, app),
        CurrentScreen::FileSelection => draw_file_selection_screen(frame, app),
        CurrentScreen::MergeConfig => draw_merge_config_screen(frame, app),
        CurrentScreen::DeleteConfig => draw_delete_config_screen(frame, app),
        CurrentScreen::Processing => draw_processing_screen(frame, app),
        CurrentScreen::Result => draw_result_screen(frame, app),
        CurrentScreen::Help => draw_help_screen(frame, app),
        CurrentScreen::Exiting => draw_exit_screen(frame, app),
    }
}

fn draw_main_screen(frame: &mut Frame, _app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Menu
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("📄 PDF Cutter - TUI")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Menu options
    let menu_items = vec![
        ListItem::new("1. 🔗 Merge PDFs").style(Style::default().fg(Color::Green)),
        ListItem::new("2. ✂️  Delete Pages").style(Style::default().fg(Color::Red)),
        ListItem::new("3. ❓ Help").style(Style::default().fg(Color::Yellow)),
        ListItem::new("q. 🚪 Exit").style(Style::default().fg(Color::Magenta)),
    ];

    let menu = List::new(menu_items)
        .block(
            Block::default()
                .title("Select Operation")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(menu, chunks[1]);

    // Footer
    let footer = Paragraph::new("Press number keys to select • q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}

fn draw_file_selection_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // File List
            Constraint::Length(3), // Input Zone
            Constraint::Length(3), // Footer Zone
        ])
        .split(frame.area());

    let title_text = match app.operation_mode {
        OperationMode::Merge => "📄 Select PDFs to Merge",
        OperationMode::Delete => "📄 Select PDF for Page Deletion",
        _ => "📄 File Selection",
    };

    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let file_items: Vec<ListItem> = app
        .selected_files
        .iter()
        .enumerate()
        .map(|(i, file)| ListItem::new(format!("{}. {}", i + 1, file)))
        .collect();


    let file_list = List::new(file_items)
        .block(
            Block::default()
                .title("Selected Files")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(
        file_list,
        chunks[1],
        &mut ListState::default().with_selected(Some(app.selected_file_index)),
    );

    // Input zone
    let binding = String::new();
    let input_text = app.current_input.as_ref().unwrap_or(&binding);
    let input_field = Paragraph::new(format!("Path: {}", input_text))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().title("Add File").borders(Borders::ALL));
    frame.render_widget(input_field, chunks[2]);

    // Instructions/Footer
    let instructions =
        Paragraph::new("Enter: Add file | <- : Remove selected | -> : Continue | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
    frame.render_widget(instructions, chunks[3]);
}

fn draw_merge_config_screen(frame: &mut Frame, app: &App) {
    // TODO: Implement merge config screen
    return;
}

fn draw_delete_config_screen(frame: &mut Frame, app: &App) {
    // TODO: Implement delete config screen
    return;
}

fn draw_processing_screen(frame: &mut Frame, app: &App) {
    // TODO: Implement processing screen
    return;
}

fn draw_result_screen(frame: &mut Frame, app: &App) {
    return;
}

fn draw_help_screen(frame: &mut Frame, app: &App) {
    // TODO: Implement help screen
    return;
}

fn draw_exit_screen(frame: &mut Frame, _app: &App) {
    frame.render_widget(Clear, frame.area());

    let popup_block = Block::default()
        .title("Exit Confirmation")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Are you sure you want to exit PDF Cutter? (y/n)",
        Style::default().fg(Color::Red),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
