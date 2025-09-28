use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::tui::app::{App, CurrentScreen, OperationMode};

macro_rules! app_theme {
    (title) => {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    (input) => {
        Style::default().fg(Color::Yellow)
    };
    (footer) => {
        Style::default().fg(Color::Gray)
    };
    (error) => {
        Style::default().fg(Color::Red)
    };
    (success) => {
        Style::default().fg(Color::Green)
    };
    (highlight) => {
        Style::default().add_modifier(Modifier::REVERSED)
    };
    (normal) => {
        Style::default().fg(Color::White)
    };
    (accent) => {
        Style::default().fg(Color::Magenta)
    };
    (menu_merge) => {
        Style::default().fg(Color::Green)
    };
    (menu_delete) => {
        Style::default().fg(Color::Red)
    };
    (menu_help) => {
        Style::default().fg(Color::Yellow)
    };
    (menu_exit) => {
        Style::default().fg(Color::Magenta)
    };
}

// Macros pour créer des widgets avec le thème
macro_rules! themed_widget {
    (title, $text:expr) => {
        Paragraph::new($text)
            .style(app_theme!(title))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
    };

    (footer, $text:expr) => {
        Paragraph::new($text)
            .style(app_theme!(footer))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
    };

    (input, $text:expr, $title:expr) => {
        Paragraph::new($text)
            .style(app_theme!(input))
            .block(Block::default().title($title).borders(Borders::ALL))
    };

    (error_input, $text:expr, $title:expr) => {
        Paragraph::new($text)
            .style(app_theme!(error))
            .block(Block::default().title($title).borders(Borders::ALL))
    };
}

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

    // Title avec macro
    let title = themed_widget!(title, "📄 PDF Cutter - TUI");
    frame.render_widget(title, chunks[0]);

    // Menu options avec les couleurs du thème
    let menu_items = vec![
        ListItem::new("1. 🔗 Merge PDFs").style(app_theme!(menu_merge)),
        ListItem::new("2. ✂️  Delete Pages").style(app_theme!(menu_delete)),
        ListItem::new("3. ❓ Help").style(app_theme!(menu_help)),
        ListItem::new("q. 🚪 Exit").style(app_theme!(menu_exit)),
    ];

    let menu = List::new(menu_items)
        .block(
            Block::default()
                .title("Select Operation")
                .borders(Borders::ALL),
        )
        .style(app_theme!(normal));

    frame.render_widget(menu, chunks[1]);

    let footer = themed_widget!(footer, "Press number keys to select • q: Quit");
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

    // Title avec macro
    let title = themed_widget!(title, title_text);
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
        .style(app_theme!(normal))
        .highlight_style(app_theme!(highlight))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(
        file_list,
        chunks[1],
        &mut ListState::default().with_selected(Some(app.selected_file_index)),
    );

    let binding = String::new();
    let input_text = app.current_input.as_ref().unwrap_or(&binding);
    let input_field = if app.error_message.is_some() {
        themed_widget!(
            error_input,
            format!("ERROR: {}", app.error_message.as_ref().unwrap()),
            "Add File"
        )
    } else {
        themed_widget!(input, format!("Path: {}", input_text), "Add File")
    };
    frame.render_widget(input_field, chunks[2]);

    let footer = themed_widget!(
        footer,
        "Enter: Add file | <- : Remove selected | -> : Continue | Esc: Back"
    );
    frame.render_widget(footer, chunks[3]);

    if let Some(error) = &app.error_message {
        draw_error_popup(frame, error);
    }
}

fn draw_merge_config_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // File List
            Constraint::Length(5), // Output filename
            Constraint::Length(3), // Instructions
        ])
        .split(frame.area());

    let title_text = "🔗 Merge Configuration";
    let title = themed_widget!(title, title_text);
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
                .title("Files to Merge (in order)")
                .borders(Borders::ALL),
        )
        .style(app_theme!(normal))
        .highlight_style(app_theme!(highlight))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(
        file_list,
        chunks[1],
        &mut ListState::default().with_selected(Some(app.merge_file_index)),
    );

    let output_text = if app.output_filename.is_empty() {
        "merged_output.pdf".to_string()
    } else {
        app.output_filename.clone()
    };

    let output_title = if app.editing_output {
        "Output Filename (editing)"
    } else {
        "Output Filename"
    };

    let output_field = if app.editing_output {
        Paragraph::new(format!("Output: {}", output_text))
            .style(app_theme!(input).add_modifier(Modifier::UNDERLINED))
            .block(Block::default().title(output_title).borders(Borders::ALL))
    } else {
        themed_widget!(input, format!("Output: {}", output_text), output_title)
    };
    frame.render_widget(output_field, chunks[2]);

    let instructions = themed_widget!(
        footer,
        "Tab: Edit output name • Enter: Start merge • Esc: Back"
    );
    frame.render_widget(instructions, chunks[3]);

    if let Some(error) = &app.error_message {
        draw_error_popup(frame, error);
    }
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
    frame.render_widget(Clear, frame.area());

    let popup_block = Block::default()
        .title("Operation Result")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let (message, color) = if let Some(err) = &app.error_message {
        (err.as_str(), Color::Red)
    } else if let Some(success) = &app.success_message {
        (success.as_str(), Color::Green)
    } else {
        ("No result available", Color::Yellow)
    };

    let result_text = Text::styled(message, Style::default().fg(color));

    let result_paragraph = Paragraph::new(result_text)
        .block(popup_block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(result_paragraph, area);
}

fn draw_help_screen(frame: &mut Frame, app: &App) {
     let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // content
            Constraint::Length(3), // Footer
        ])
        .split(frame.area());

    let title_text = "❓ Help";
    let title = themed_widget!(title, title_text);
    frame.render_widget(title, chunks[0]);
    
    let help_text = Text::from_iter([
        Line::from("📄 PDF Cutter TUI Help"),
        Line::from(""),
        Line::from("🔧 Operations:"),
        Line::from("  1. 🔗 Merge PDFs: Select multiple PDF files to combine them into one."),
        Line::from("  2. ✂️  Delete Pages: Select a PDF and specify pages to remove."),
        Line::from(""),
        Line::from("🧭 Navigation:"),
        Line::from("  • Use number keys (1, 2, 3) to select operations from the main menu."),
        Line::from("  • In file selection: Enter to add files, ← to remove, → to continue."),
        Line::from("  • In merge config: Tab to edit output filename, Enter to start merging."),
        Line::from("  • Use Esc to go back to previous screen."),
        Line::from(""),
        Line::from("⌨️  Keyboard Shortcuts:"),
        Line::from("  • ↑↓: Navigate lists"),
        Line::from("  • Enter: Confirm/Select"),
        Line::from("  • Tab: Switch input fields"),
        Line::from("  • Esc: Go back"),
        Line::from("  • q: Quit application"),
        Line::from(""),
        Line::from("Press Esc to return to the main menu."),
    ]);

    let help_paragraph = Paragraph::new(help_text)
        .style(app_theme!(normal))
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);

    frame.render_widget(help_paragraph, chunks[1]);

    let footer = themed_widget!(footer, "Press Esc to return to main menu");
    frame.render_widget(footer, chunks[2]);
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

fn draw_error_popup(frame: &mut Frame, message: &str) {
    frame.render_widget(Clear, frame.area());

    let popup_block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let error_text = Text::styled(message, Style::default().fg(Color::Red));

    let error_paragraph = Paragraph::new(error_text)
        .block(popup_block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(error_paragraph, area);
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
