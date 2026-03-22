use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_professional_summary_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    // Header
    let header_text = "Select Professional Summary | Navigate: j/k | Confirm: <Enter>";
    let header =
        Paragraph::new(header_text).block(Block::bordered().title(" Professional Summary "));
    frame.render_widget(header, chunks[0]);

    // List items
    let items: Vec<ListItem> = if app.data.professional_summaries.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No professional summaries found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .professional_summaries
            .iter()
            .enumerate()
            .map(|(idx, summary)| {
                let selected = app
                    .professional_summary_list_state
                    .selected()
                    .map(|s| s == idx)
                    .unwrap_or(false);

                let marker = if selected { "[x] " } else { "[ ] " };
                let content = format!("{}{}", marker, summary.summary);

                // Use Paragraph for wrapping, or just let ListItem handle it if it doesn't wrap natively.
                // We'll just rely on ListItem's default rendering or create a custom text.
                // Actually ratatui ListItem wraps natively if the text inside is a single line without manual breaks?
                // Let's just use ListItem::new with Line
                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Available Summaries "))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.professional_summary_list_state);

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
