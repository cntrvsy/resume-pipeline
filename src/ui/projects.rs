use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_projects_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_text = "Step 4: Select Projects | Navigate: j/k | Toggle: <Space>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Projects "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = if app.data.projects.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No projects data found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .projects
            .iter()
            .map(|proj| {
                let status = if proj.is_visible { "[x] " } else { "[ ] " };
                let content = format!("{}{}", status, proj.title);
                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Projects List "))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.projects_list_state);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " <Backspace> ",
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
        Span::raw(" Back    "),
        Span::styled(
            " <Enter> ",
            Style::default().bg(Color::Magenta).fg(Color::White),
        ),
        Span::raw(" Generate PDF    "),
        Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
        Span::raw(" Quit "),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
