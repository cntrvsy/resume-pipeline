use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

mod models; // Import the models module
use models::ResumeData;

// 1. STATE MANAGEMENT
// We track which screen the user is looking at.
#[derive(Debug, PartialEq)]
enum CurrentScreen {
    Welcome,
    Selection,
    Exiting,
}
// We implement Default manually to control the starting state
impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::Welcome
    }
}
// The main application state
#[derive(Debug, Default)]
struct App {
    current_screen: CurrentScreen,
    data: ResumeData,
    // UI STATE for scrolling
    // We only track one list for now (Experience) to keep it simple
    experience_list_state: ListState,
}

impl App {
    fn new() -> Self {
        let data = ResumeData::load_from_dir().unwrap_or_else(|e| {
            eprintln!("Failed to load data: {}", e);
            ResumeData::default()
        });

        Self {
            // FIX: Use the specific variant name
            current_screen: CurrentScreen::Welcome,
            data,
            experience_list_state: ListState::default(),
        }
    }

    // HELPER: Select the next item in the list
    fn next_item(&mut self) {
        // SAFETY CHECK: If list is empty, do nothing
        if self.data.experience.is_empty() {
            return;
        }

        let i = match self.experience_list_state.selected() {
            Some(i) => {
                if i >= self.data.experience.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.experience_list_state.select(Some(i));
    }

    // HELPER: Select the previous item
    fn previous_item(&mut self) {
        // SAFETY CHECK
        if self.data.experience.is_empty() {
            return;
        }

        let i = match self.experience_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.data.experience.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.experience_list_state.select(Some(i));
    }

    // HELPER: Toggle the 'is_visible' flag
    fn toggle_current_selection(&mut self) {
        if let Some(i) = self.experience_list_state.selected() {
            if let Some(item) = self.data.experience.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match self.current_screen {
            CurrentScreen::Welcome => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::Selection;
                    // Auto-select the first item so the list isn't empty
                    self.experience_list_state.select(Some(0));
                }
                _ => {}
            },
            CurrentScreen::Selection => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.next_item(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_item(),
                KeyCode::Char(' ') => self.toggle_current_selection(),
                // We will add 'Enter' -> 'Generate PDF' here later
                _ => {}
            },
            CurrentScreen::Exiting => {}
        }
    }
}

// 2. ENTRY POINT
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

// 3. APPLICATION LOOP
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    while app.current_screen != CurrentScreen::Exiting {
        terminal.draw(|frame| render_ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key_event(key.code);
            }
        }
    }
    Ok(())
}

// 4. RENDERING LOGIC
fn render_ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Welcome => render_welcome_screen(frame),
        CurrentScreen::Selection => render_selection_screen(frame, app), // <--- New
        CurrentScreen::Exiting => {}
    }
}
fn render_selection_screen(frame: &mut Frame, app: &mut App) {
    // 1. Layout: Header (Instructions) + Main List
    let chunks = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Fill(1),   // The List
    ])
    .split(frame.area());

    // 2. The Header
    let header_text = "Navigate: j/k | Toggle: <Space> | Quit: q";
    let header =
        Paragraph::new(header_text).block(Block::bordered().title(" Experience Selector "));
    frame.render_widget(header, chunks[0]);

    // 3. The List Items
    // We iterate over the loaded experience data
    let items: Vec<ListItem> = if app.data.experience.is_empty() {
        // ERROR STATE: If no data loaded, show a red warning
        vec![ListItem::new(Line::from(Span::styled(
            "ERROR: No data found in data/experience.yaml",
            Style::default().fg(Color::Red),
        )))]
    } else {
        // NORMAL STATE: Render the jobs
        app.data
            .experience
            .iter()
            .map(|job| {
                let status = if job.is_visible { "[x] " } else { "[ ] " };
                let content = format!("{}{}", status, job.role);
                ListItem::new(Line::from(content))
            })
            .collect()
    };

    // 4. Create the List Widget
    let list = List::new(items)
        .block(Block::bordered().title(" Work History "))
        .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black)) // The selection color
        .highlight_symbol(">> ");

    // 5. Render using the State (which tracks the scroll position)
    frame.render_stateful_widget(
        list,
        chunks[1],
        &mut app.experience_list_state, // Now valid!
    );
}

fn render_welcome_screen(frame: &mut Frame) {
    // A. Create the Layout (Vertical Split)
    // 1. Top filler
    // 2. The Content (Title + Subtitle)
    // 3. The Footer (Instructions)
    // 4. Bottom filler
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10), // The "Box" height
        Constraint::Length(3),  // The Footer height
        Constraint::Fill(1),
    ])
    .split(frame.area());

    // B. Create the Horizontal Layout (Center the box)
    let area_centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(60), // Width of the box
        Constraint::Fill(1),
    ])
    .split(vertical_layout[1])[1]; // Select the middle column

    // C. The Title Block
    let title_block = Block::bordered()
        .border_style(Style::default().fg(Color::Cyan))
        .title(" CV GEN v0.1 ")
        .title_alignment(Alignment::Center);

    let welcome_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Welcome, "),
            Span::styled(
                "Benin",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("This tool will help you generate targeted resumes"),
        Line::from("based on your YAML data source."),
        Line::from(""),
        Line::from("Target Role: Fullstack Engineer (Default)"),
    ];

    let paragraph = Paragraph::new(welcome_text)
        .block(title_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area_centered);

    // D. The Footer (Instructions)
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
        .block(Block::default()); // No border for footer

    frame.render_widget(footer, vertical_layout[2]);
}
