use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_experience_bullet_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header_text = "Step 3a: Select Bullet Points | Navigate: j/k | Toggle: <Space>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Experience Bullets "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = if let Some(job_index) = app.experience_list_state.selected() {
        if let Some(job) = app.data.experience.get(job_index) {
            if job.bullets.is_empty() {
                vec![ListItem::new(Line::from(Span::styled(
                    "No bullet points found for this experience",
                    Style::default().fg(Color::Yellow),
                )))]
            } else {
                job.bullets
                    .iter()
                    .enumerate()
                    .map(|(i, bullet)| {
                        let status = if job.hidden_bullets.contains(&i) {
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
            vec![ListItem::new(Line::from("Error finding selected job"))]
        }
    } else {
        vec![ListItem::new(Line::from("No job selected"))]
    };

    let title = if let Some(job_index) = app.experience_list_state.selected() {
        if let Some(job) = app.data.experience.get(job_index) {
            format!(" {} at {} ", job.role, job.company)
        } else {
            " Highlights ".to_string()
        }
    } else {
        " Highlights ".to_string()
    };

    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.experience_bullet_list_state);

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
