use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_project_bullet_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_text = "Step 4a: Select Project Bullets | Navigate: j/k | Toggle: <Space>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Project Bullets "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = if let Some(proj_index) = app.projects_list_state.selected() {
        if let Some(proj) = app.data.projects.get(proj_index) {
            if proj.bullets.is_empty() {
                vec![ListItem::new(Line::from(Span::styled(
                    "No bullet points found for this project",
                    Style::default().fg(Color::Yellow),
                )))]
            } else {
                proj.bullets
                    .iter()
                    .enumerate()
                    .map(|(i, bullet)| {
                        let status = if proj.hidden_bullets.contains(&i) {
                            "[ ] "
                        } else {
                            "[x] "
                        };
                        let content = format!("{}{}", status, bullet);
                        ListItem::new(Line::from(content))
                    })
                    .collect()
            }
        } else {
            vec![ListItem::new(Line::from("Error finding selected project"))]
        }
    } else {
        vec![ListItem::new(Line::from("No project selected"))]
    };

    let title = if let Some(proj_index) = app.projects_list_state.selected() {
        if let Some(proj) = app.data.projects.get(proj_index) {
            format!(" Bullets: {} ", proj.title)
        } else {
            " Bullets ".to_string()
        }
    } else {
        " Bullets ".to_string()
    };

    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.project_bullet_list_state);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " <Backspace> / <Enter> ",
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
        Span::raw(" Back    "),
        Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
        Span::raw(" Quit "),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
