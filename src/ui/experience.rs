use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_experience_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_text = "Step 3: Select Experience | Navigate: j/k | Toggle: <Space>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Experience "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = if app.data.experience.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No experience data found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .experience
            .iter()
            .map(|job| {
                let status = if job.is_visible { "[x] " } else { "[ ] " };
                let content = format!("{}{} at {}", status, job.role, job.company);
                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Work History "))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.experience_list_state);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " <Backspace> ",
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
        Span::raw(" Back    "),
        Span::styled(
            " <Enter> ",
            Style::default().bg(Color::Green).fg(Color::Black),
        ),
        Span::raw(" Continue    "),
        Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
        Span::raw(" Quit "),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
