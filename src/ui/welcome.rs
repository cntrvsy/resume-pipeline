use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn render_welcome_screen(frame: &mut Frame) {
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(frame.area());

    let area_centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(60),
        Constraint::Fill(1),
    ])
    .split(vertical_layout[1])[1];

    let title_block = Block::bordered()
        .border_style(Style::default().fg(Color::Cyan))
        .title(" CV GEN v0.2 ")
        .title_alignment(Alignment::Center);

    let welcome_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Welcome, "),
            Span::styled(
                "User",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("This tool will help you generate targeted resumes"),
        Line::from("based on your YAML data source."),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .block(title_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area_centered);

    let footer_text = Line::from(vec![
        Span::styled(
            " <Enter> ",
            Style::default().bg(Color::Cyan).fg(Color::Black),
        ),
        Span::raw(" Start Builder    "),
        Span::styled(" <q> ", Style::default().bg(Color::Red).fg(Color::Black)),
        Span::raw(" Quit "),
    ]);

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(footer, vertical_layout[2]);
}
