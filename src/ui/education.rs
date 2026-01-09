use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_education_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_text = "Step 2: Select Education | Navigate: j/k | Toggle: <Space>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Education "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = if app.data.education.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No education data found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .education
            .iter()
            .map(|edu| {
                let status = if edu.is_visible { "[x] " } else { "[ ] " };
                let content = format!("{}{} - {}", status, edu.degree, edu.school);
                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Education Items "))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.education_list_state);

    let footer = Paragraph::new(Line::from(vec![
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
