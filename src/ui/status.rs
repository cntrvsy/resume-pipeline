use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

pub fn render_generating_screen(frame: &mut Frame) {
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Fill(1),
    ])
    .split(frame.area());

    let area_centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(40),
        Constraint::Fill(1),
    ])
    .split(vertical_layout[1])[1];

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "⏳ Generating PDF...",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::bordered())
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area_centered);
}

pub fn render_success_screen(frame: &mut Frame, path: &str) {
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(9), // Increased height for buttons
        Constraint::Fill(1),
    ])
    .split(frame.area());

    let area_centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(60),
        Constraint::Fill(1),
    ])
    .split(vertical_layout[1])[1];

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "✓ PDF Generated Successfully!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Output: ", Style::default().fg(Color::Cyan)),
            Span::raw(path),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
            Span::raw(" Quit    "),
            Span::styled(
                " <Enter> ",
                Style::default().bg(Color::Green).fg(Color::Black),
            ),
            Span::raw(" Done "),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::bordered().title(" Success "))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area_centered);
}

pub fn render_error_screen(frame: &mut Frame, error: &str) {
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Fill(1),
    ])
    .split(frame.area());

    let area_centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(70),
        Constraint::Fill(1),
    ])
    .split(vertical_layout[1])[1];

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "✗ Error Generating PDF",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(error),
        Line::from(""),
        Line::from(Span::styled(
            "Press <Enter> or <q> to exit",
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::bordered().title(" Error "))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area_centered);
}
