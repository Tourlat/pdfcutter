use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::tui::app::App;
use crate::tui::state::{CurrentScreen, OperationMode};

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
    (menu_split) => {
        Style::default().fg(Color::Blue)
    };
    (menu_help) => {
        Style::default().fg(Color::Yellow)
    };
    (menu_exit) => {
        Style::default().fg(Color::Magenta)
    };
}

// Macros for widget theming
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

pub fn create_file_list<'a>(
    files: &'a [String],
    title: &'a str,
    selected_index: Option<usize>,
) -> (List<'a>, ListState) {
    let file_items: Vec<ListItem> = files
        .iter()
        .enumerate()
        .map(|(i, file)| ListItem::new(format!("{}. {}", i + 1, file)))
        .collect();

    let file_list = List::new(file_items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(app_theme!(normal))
        .highlight_style(app_theme!(highlight))
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    if let Some(index) = selected_index {
        if index < files.len() {
            list_state.select(Some(index));
        }
    }

    (file_list, list_state)
}

pub fn create_title(text: &str) -> Paragraph {
    themed_widget!(title, text)
}

pub fn create_footer(text: &str) -> Paragraph {
    themed_widget!(footer, text)
}

pub fn create_input_field<'a>(
    content: &'a str,
    title: &'a str,
    is_editing: bool,
    error_message: Option<&'a str>,
) -> Paragraph<'a> {
    let display_text = format!(
        "{}: {}",
        title.split(' ').next().unwrap_or("Input"),
        content
    );

    if let Some(error) = error_message {
        themed_widget!(error_input, format!("ERROR: {}", error), title)
    } else if is_editing {
        Paragraph::new(display_text)
            .style(app_theme!(input).add_modifier(Modifier::UNDERLINED))
            .block(Block::default().title(title).borders(Borders::ALL))
    } else {
        themed_widget!(input, display_text, title)
    }
}

pub fn create_checkbox<'a>(title: &'a str, is_checked: bool, is_focused: bool) -> Paragraph<'a> {
    let checkbox_symbol = if is_checked { "☑" } else { "☐" };
    let display_text = format!("{} {}", checkbox_symbol, title);

    let style = if is_focused {
        app_theme!(highlight)
    } else {
        app_theme!(normal)
    };

    Paragraph::new(display_text)
        .style(style)
        .block(Block::default().borders(Borders::ALL))
}

pub fn create_standard_layout(frame_area: Rect, sections: &[u16]) -> Vec<Rect> {
    let constraints: Vec<Constraint> = sections
        .iter()
        .map(|&size| {
            if size == 0 {
                Constraint::Min(0)
            } else {
                Constraint::Length(size)
            }
        })
        .collect();

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame_area)
        .to_vec()
}

pub fn render_error_if_exists(frame: &mut Frame, error_message: Option<&str>) {
    if let Some(error) = error_message {
        draw_error_popup(frame, error);
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Main => draw_main_screen(frame, app),
        CurrentScreen::FileSelection => draw_file_selection_screen(frame, app),
        CurrentScreen::MergeConfig => draw_merge_config_screen(frame, app),
        CurrentScreen::DeleteConfig => draw_delete_config_screen(frame, app),
        CurrentScreen::SplitConfig => draw_split_config_screen(frame, app),
        // CurrentScreen::Processing => draw_processing_screen(frame, app),
        CurrentScreen::Result => draw_result_screen(frame, app),
        CurrentScreen::Help => draw_help_screen(frame),
        CurrentScreen::Exiting => draw_exit_screen(frame, app),
    }
}

/**
 * Draw the main screen UI.
 * Display the title, menu options, and footer.
 * @param frame The frame to draw on.
 * @param app The application state.
 */
fn draw_main_screen(frame: &mut Frame, app: &App) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 3]);

    frame.render_widget(create_title("📄 PDF Cutter - TUI"), chunks[0]);

    let menu_items = vec![
        ListItem::new("1. 🔗 Merge PDFs").style(app_theme!(menu_merge)),
        ListItem::new("2. ✂️  Delete Pages").style(app_theme!(menu_delete)),
        ListItem::new("3. 🔪  Split Pages").style(app_theme!(menu_split)),
        ListItem::new("4. ❓ Help").style(app_theme!(menu_help)),
        ListItem::new("q. 🚪 Exit").style(app_theme!(menu_exit)),
    ];

    let menu = List::new(menu_items)
        .block(
            Block::default()
                .title("Select Operation")
                .borders(Borders::ALL),
        )
        .style(app_theme!(normal))
        .highlight_style(app_theme!(highlight))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(
        menu,
        chunks[1],
        &mut ListState::default().with_selected(Some(app.menu_mode_index())),
    );

    frame.render_widget(
        create_footer("↑↓: Navigate • Enter: Select • 1-3: Direct select • q: Quit"),
        chunks[2],
    );
}

/**
 * Draw the file selection screen UI.
 * Display selected files, input for adding files, and footer instructions.
 *
*/
fn draw_file_selection_screen(frame: &mut Frame, app: &App) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 3, 3]);

    let title_text = match app.operation_mode {
        OperationMode::Merge => "📄 Select PDFs to Merge",
        OperationMode::Delete => "📄 Select PDF for Page Deletion",
        _ => "📄 File Selection",
    };

    frame.render_widget(create_title(title_text), chunks[0]);

    let (file_list, mut list_state) = create_file_list(
        &app.selected_files(),
        "Selected Files",
        if app.file_state.is_empty() {
            None
        } else {
            Some(app.selected_file_index())
        },
    );
    frame.render_stateful_widget(file_list, chunks[1], &mut list_state);

    let binding = String::new();
    let input_text = app.current_input().unwrap_or(&binding);
    let input_title = if app.editing_input() {
        "Enter file path (Enter to add, Esc to cancel)"
    } else {
        "File path (Tab add file)"
    };

    let input_field = create_input_field(
        input_text,
        input_title,
        app.editing_input(),
        app.error_message(),
    );
    frame.render_widget(input_field, chunks[2]);

    let instructions = if app.editing_input() {
        "Enter: Add file | Esc: Cancel"
    } else {
        match app.operation_mode {
            OperationMode::Merge => {
                "↑/↓: Navigate • Tab: Add file • <-: Delete • Enter: Next • Alt+↑/↓: Reorder • Esc: Back"
            }
            OperationMode::Delete => {
                "↑/↓: Navigate • Tab: Add file • <-: Delete • Enter: Next • Esc: Back"
            }
            _ => "↑/↓: Navigate • Tab: Add file • <-: Delete • Enter: Next • Esc: Back",
        }
    };

    frame.render_widget(create_footer(instructions), chunks[3]);
    render_error_if_exists(frame, app.error_message().as_deref());
}

/**
 * Draw the merge configuration screen UI.
 * Display selected files, output filename input, and footer instructions.
 */
fn draw_merge_config_screen(frame: &mut Frame, app: &App) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 3, 3]);

    frame.render_widget(create_title("🔗 Merge Configuration"), chunks[0]);

    let (file_list, mut list_state) = create_file_list(
        &app.selected_files(),
        "Files to Merge (in order)",
        Some(app.merge_file_index()),
    );
    frame.render_stateful_widget(file_list, chunks[1], &mut list_state);

    let output_text = if app.merge_config.output_filename.is_empty() {
        "merged_output.pdf"
    } else {
        &app.merge_config.output_filename
    };

    let output_field = create_input_field(
        output_text,
        "Output Filename",
        app.merge_config.editing_output,
        None,
    );
    frame.render_widget(output_field, chunks[2]);

    frame.render_widget(
        create_footer("Tab: Edit output name • Enter: Start merge • Esc: Back"),
        chunks[3],
    );

    render_error_if_exists(frame, app.error_message().as_deref());
}

/**
 * Draw the delete configuration screen UI.
 * Display selected files, pages to delete input, output filename input, and footer instructions.
 */
fn draw_delete_config_screen(frame: &mut Frame, app: &App) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 5, 5, 3]);

    frame.render_widget(create_title("✂️ Delete Configuration"), chunks[0]);

    let (file_list, mut list_state) = create_file_list(
        &app.selected_files(),
        "File to Delete Pages From",
        Some(app.merge_file_index()),
    );
    frame.render_stateful_widget(file_list, chunks[1], &mut list_state);

    let pages_field = create_input_field(
        &app.delete_config.pages_to_delete,
        "Pages to Delete (e.g., 1,3-5)",
        app.delete_config.editing_pages,
        None,
    );
    frame.render_widget(pages_field, chunks[2]);

    let output_text = if app.delete_config.output_filename.is_empty() {
        "modified_output.pdf"
    } else {
        &app.delete_config.output_filename
    };

    let output_field = create_input_field(
        output_text,
        "Output Filename",
        app.delete_config.editing_output,
        None,
    );
    frame.render_widget(output_field, chunks[3]);

    frame.render_widget(
        create_footer(
            "p: Edit pages to delete • Tab: Edit output name • Enter: Start delete • Esc: Back",
        ),
        chunks[4],
    );

    render_error_if_exists(frame, app.error_message().as_deref());
}

// fn draw_processing_screen(frame: &mut Frame, app: &App) {
//     // TODO: Implement processing screen
//     return;
// }

fn draw_split_config_screen(frame: &mut Frame, app: &App) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 4, 4, 4, 3]);

    frame.render_widget(create_title("🔪 Split Configuration"), chunks[0]);

    // File to split
    let (file_list, mut list_state) = create_file_list(
        &app.selected_files(),
        "File to Split",
        Some(0), // Only one file for split
    );
    frame.render_stateful_widget(file_list, chunks[1], &mut list_state);

    // Page segments input
    let (segments_text, segments_title) = if app.split_config.use_named_segments {
        (
            if app.split_config.segments.is_empty() {
                "intro:1-3,chapter1:4-10,conclusion:11"
            } else {
                &app.split_config.segments
            },
            "Named Segments (e.g., intro:1-3,chapter1:4-10,conclusion:11)",
        )
    } else {
        (
            if app.split_config.segments.is_empty() {
                "1-3,5,7-9"
            } else {
                &app.split_config.segments
            },
            "Page Segments (e.g., 1-3,5,7-9)",
        )
    };

    let segments_field = create_input_field(
        segments_text,
        segments_title,
        app.split_config.editing_segments,
        None,
    );
    frame.render_widget(segments_field, chunks[2]);

    // Checkbox for using named segments
    let checkbox = create_checkbox(
        "Use named segments",
        app.split_config.use_named_segments,
        false,
    );
    frame.render_widget(checkbox, chunks[3]);

    let output_text = if app.split_config.output_prefix.is_empty() {
        "split_output"
    } else {
        &app.split_config.output_prefix
    };

    let output_field = create_input_field(
        output_text,
        "Output Prefix",
        app.split_config.editing_prefix,
        None,
    );
    frame.render_widget(output_field, chunks[4]);

    let instructions = if app.split_config.editing_segments {
        "Enter: Save segments • Esc: Cancel"
    } else if app.split_config.editing_prefix {
        "Enter: Save prefix • Esc: Cancel"
    } else {
        "S: Edit segments • Space: Toggle named segments • Tab: Edit prefix • Enter: Split • Esc: Back"
    };

    frame.render_widget(create_footer(instructions), chunks[5]);

    render_error_if_exists(frame, app.error_message().as_deref());
}

/**
 * Draw the result screen UI.
 * Display success or error message after operation.
 * @param frame The frame to draw on.
 * @param app The application state.
 */
fn draw_result_screen(frame: &mut Frame, app: &App) {
    frame.render_widget(Clear, frame.area());

    let popup_block = Block::default()
        .title("Operation Result")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let (message, color) = if let Some(err) = app.error_message() {
        (err, Color::Red)
    } else if let Some(success) = app.success_message() {
        (success, Color::Green)
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

fn draw_help_screen(frame: &mut Frame) {
    let chunks = create_standard_layout(frame.area(), &[3, 0, 3]);

    frame.render_widget(create_title("❓ Help"), chunks[0]);

    let help_text = Text::from_iter([
        Line::from("📄 PDF Cutter TUI Help"),
        Line::from(""),
        Line::from("🔧 Operations:"),
        Line::from("  1. 🔗 Merge PDFs: Select multiple PDF files to combine them into one."),
        Line::from("  2. ✂️  Delete Pages: Select a PDF and specify pages to remove."),
        Line::from(""),
        Line::from("🧭 Navigation:"),
        Line::from("  • Use number keys (1, 2, 3) to select operations from the main menu."),
        Line::from("  • In file selection: Tab/A to add files, D/← to remove, Enter to continue."),
        Line::from("  • In merge config: Tab to edit output filename, Enter to start merging."),
        Line::from("  • Use Esc to go back to previous screen."),
        Line::from(""),
        Line::from("⌨️  File Selection Shortcuts:"),
        Line::from("  • Tab: Add file (enter edit mode)"),
        Line::from("  • ↑↓: Navigate file list"),
        Line::from("  • Del: Delete selected file"),
        Line::from("  • Alt+↑↓: Reorder files (merge mode)"),
        Line::from("  • Enter: Continue to next step"),
        Line::from(""),
        Line::from("✏️  Edit Mode (when adding files):"),
        Line::from("  • Type: Enter file path"),
        Line::from("  • Enter: Add file and exit edit mode"),
        Line::from("  • Esc: Cancel and exit edit mode"),
        Line::from(""),
        Line::from("🎯 General Shortcuts:"),
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
    frame.render_widget(create_footer("Press Esc to return to main menu"), chunks[2]);
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
