use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_job_title_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    // Header
    let header_text = "Step 1: Select Job Title | Navigate: j/k | Confirm: <Enter>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Job Title "));
    frame.render_widget(header, chunks[0]);

    // List items
    let items: Vec<ListItem> = if app.data.job_titles.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No job titles found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .job_titles
            .iter()
            .enumerate()
            .map(|(idx, jt)| {
                let selected = app
                    .job_title_list_state
                    .selected()
                    .map(|s| s == idx)
                    .unwrap_or(false);

                let marker = if selected { "[x] " } else { "[ ] " };
                let content = format!("{}{}", marker, jt.title);

                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Available Roles "))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.job_title_list_state);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " <Enter> ",
            Style::default().bg(Color::Green).fg(Color::Black),
        ),
        Span::raw(" Select & Continue    "),
        Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
        Span::raw(" Quit "),
    ]))
    .alignment(Alignment::Center);

    frame.render_widget(footer, chunks[2]);
}
