use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render_profile_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header =
        Paragraph::new("Step 1: Profile Information").block(Block::bordered().title(" Profile "));
    frame.render_widget(header, chunks[0]);

    let profile_info = if let Some(ref profile) = app.data.profile {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.name),
            ]),
            Line::from(vec![
                Span::styled("Email: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.email),
            ]),
            Line::from(vec![
                Span::styled("Phone: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.phone),
            ]),
            Line::from(vec![
                Span::styled("URL: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.url),
            ]),
            Line::from(vec![
                Span::styled("Website: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.website),
            ]),
            Line::from(vec![
                Span::styled("Location: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.location),
            ]),
            Line::from(vec![
                Span::styled("Citizenship: ", Style::default().fg(Color::Cyan)),
                Span::raw(&profile.citizenship),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled(
            "No profile data loaded",
            Style::default().fg(Color::Red),
        ))]
    };

    let content = Paragraph::new(profile_info)
        .block(Block::bordered())
        .alignment(Alignment::Left);
    frame.render_widget(content, chunks[1]);

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
