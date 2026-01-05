use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};
use std::fs;
use std::path::Path;

mod models;
mod typst_backend;
use models::ResumeData;

// 1. STATE MANAGEMENT
#[derive(Debug, PartialEq)]
enum CurrentScreen {
    Welcome,
    ProfileView,
    JobTitleSelection,
    EducationSelection,
    ExperienceSelection,
    ProjectsSelection,
    Generating,
    Success(String), // Contains the output path
    Error(String),
    Exiting,
}

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
    // UI STATE for scrolling through lists
    education_list_state: ListState,
    experience_list_state: ListState,
    projects_list_state: ListState,
    job_title_list_state: ListState,
}

impl App {
    fn new() -> Self {
        let data = ResumeData::load_from_dir().unwrap_or_else(|e| {
            eprintln!("Failed to load data: {}", e);
            ResumeData::default()
        });

        Self {
            current_screen: CurrentScreen::Welcome,
            data,
            education_list_state: ListState::default(),
            experience_list_state: ListState::default(),
            projects_list_state: ListState::default(),
            job_title_list_state: ListState::default(),
        }
    }

    // Navigation helpers for Education
    fn next_education(&mut self) {
        if self.data.education.is_empty() {
            return;
        }
        let i = match self.education_list_state.selected() {
            Some(i) => {
                if i >= self.data.education.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.education_list_state.select(Some(i));
    }

    fn previous_education(&mut self) {
        if self.data.education.is_empty() {
            return;
        }
        let i = match self.education_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.data.education.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.education_list_state.select(Some(i));
    }

    fn toggle_education(&mut self) {
        if let Some(i) = self.education_list_state.selected() {
            if let Some(item) = self.data.education.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    //Navigation helpers for Job Titles

    pub fn next_job_title(&mut self) {
        let len = self.data.job_titles.len();
        if len == 0 {
            return;
        }

        let i = match self.job_title_list_state.selected() {
            Some(i) if i + 1 < len => i + 1,
            _ => 0,
        };

        self.job_title_list_state.select(Some(i));
    }

    pub fn previous_job_title(&mut self) {
        let len = self.data.job_titles.len();
        if len == 0 {
            return;
        }

        let i = match self.job_title_list_state.selected() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };

        self.job_title_list_state.select(Some(i));
    }

    // Navigation helpers for Experience
    fn next_experience(&mut self) {
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

    fn previous_experience(&mut self) {
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

    fn toggle_experience(&mut self) {
        if let Some(i) = self.experience_list_state.selected() {
            if let Some(item) = self.data.experience.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    // Navigation helpers for Projects
    fn next_project(&mut self) {
        if self.data.projects.is_empty() {
            return;
        }
        let i = match self.projects_list_state.selected() {
            Some(i) => {
                if i >= self.data.projects.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.projects_list_state.select(Some(i));
    }

    fn previous_project(&mut self) {
        if self.data.projects.is_empty() {
            return;
        }
        let i = match self.projects_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.data.projects.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.projects_list_state.select(Some(i));
    }

    fn toggle_project(&mut self) {
        if let Some(i) = self.projects_list_state.selected() {
            if let Some(item) = self.data.projects.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match &self.current_screen {
            // ─────────────────────────────────────────────────────────────
            // Welcome → Job Title
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::Welcome => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Enter => {
                    // Skip if no job titles exist
                    if self.data.job_titles.is_empty() {
                        self.current_screen = CurrentScreen::ProfileView;
                    } else {
                        self.current_screen = CurrentScreen::JobTitleSelection;
                        self.job_title_list_state.select(Some(0));
                    }
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Job Title (single-select)
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::JobTitleSelection => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.next_job_title(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_job_title(),
                KeyCode::Enter => {
                    if let Some(i) = self.job_title_list_state.selected() {
                        self.data.job_title = Some(self.data.job_titles[i].title.clone());

                        self.current_screen = CurrentScreen::ProfileView;
                    }
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Profile
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::ProfileView => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::EducationSelection;
                    self.education_list_state.select(Some(0));
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Education
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::EducationSelection => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.next_education(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_education(),
                KeyCode::Char(' ') => self.toggle_education(),
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::ExperienceSelection;
                    self.experience_list_state.select(Some(0));
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Experience
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::ExperienceSelection => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.next_experience(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_experience(),
                KeyCode::Char(' ') => self.toggle_experience(),
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::ProjectsSelection;
                    self.projects_list_state.select(Some(0));
                }
                KeyCode::Backspace => {
                    self.current_screen = CurrentScreen::EducationSelection;
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Projects
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::ProjectsSelection => match key {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.next_project(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_project(),
                KeyCode::Char(' ') => self.toggle_project(),
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::Generating;
                    match generate_pdf(&self.data) {
                        Ok(path) => {
                            self.current_screen = CurrentScreen::Success(path);
                        }
                        Err(e) => {
                            self.current_screen = CurrentScreen::Error(format!("{}", e));
                        }
                    }
                }
                KeyCode::Backspace => {
                    self.current_screen = CurrentScreen::ExperienceSelection;
                }
                _ => {}
            },

            // ─────────────────────────────────────────────────────────────
            // Terminal states
            // ─────────────────────────────────────────────────────────────
            CurrentScreen::Success(_) | CurrentScreen::Error(_) => match key {
                KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc => {
                    self.current_screen = CurrentScreen::Exiting;
                }
                _ => {}
            },

            CurrentScreen::Generating | CurrentScreen::Exiting => {}
        }
    }
}

// 2. PDF GENERATION
fn generate_pdf(data: &ResumeData) -> Result<String> {
    use typst::foundations::Dict;
    // Import PdfOptions for 0.12 API
    use typst_backend::ResumeWorld;
    use typst_pdf::PdfOptions;

    let output_dir = Path::new("data/output");
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    // Load template
    // Note: Assuming you are using include_str! for the embedded solution
    // If you are reading from disk for dev, ensure the path is correct
    let template_path = Path::new("data/templates/headless_head_hunter.typ");
    let template_content = fs::read_to_string(template_path).expect("Could not read template file");

    // Convert Data
    let filtered_data = data.to_filtered_data();
    let inputs: Dict = filtered_data.into();

    // Create World
    let world = ResumeWorld::new(template_content, inputs);

    // Compile
    let document = typst::compile(&world)
        .output
        .map_err(|err| color_eyre::eyre::eyre!("Typst Compile Errors: {:?}", err))?;

    // FIX: Updated to match typst-pdf 0.12 signature
    // It takes 2 arguments: the document and the options.
    let options = PdfOptions::default();

    // FIX: Handle the Result returned by pdf().
    // The ? operator unwraps the Ok(Vec<u8>) or returns the Err.
    let pdf_data = typst_pdf::pdf(&document, &options)
        .map_err(|e| color_eyre::eyre::eyre!("PDF Export Error: {:?}", e))?;

    let output_path = output_dir.join("resume.pdf");
    fs::write(&output_path, pdf_data)?;

    Ok(output_path.to_string_lossy().to_string())
}

// 3. ENTRY POINT
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

// 4. APPLICATION LOOP
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

// 5. RENDERING LOGIC
fn render_ui(frame: &mut Frame, app: &mut App) {
    match &app.current_screen {
        CurrentScreen::Welcome => render_welcome_screen(frame),
        CurrentScreen::ProfileView => render_profile_screen(frame, app),
        CurrentScreen::JobTitleSelection => render_job_title_screen(frame, app),
        CurrentScreen::EducationSelection => render_education_screen(frame, app),
        CurrentScreen::ExperienceSelection => render_experience_screen(frame, app),
        CurrentScreen::ProjectsSelection => render_projects_screen(frame, app),
        CurrentScreen::Generating => render_generating_screen(frame),
        CurrentScreen::Success(path) => render_success_screen(frame, path),
        CurrentScreen::Error(msg) => render_error_screen(frame, msg),
        CurrentScreen::Exiting => {}
    }
}

fn render_welcome_screen(frame: &mut Frame) {
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

fn render_profile_screen(frame: &mut Frame, app: &App) {
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

fn render_job_title_screen(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    // Header
    let header_text = "Step 1: Select Job Title | Navigate: j/k | Confirm: <Enter>";
    let header = Paragraph::new(header_text).block(Block::bordered().title(" Job Title "));
    frame.render_widget(header, chunks[0]);

    // List items
    let items: Vec<ListItem> = if app.data.job_titles.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No job titles found",
            Style::default().fg(Color::Yellow),
        )))]
    } else {
        app.data
            .job_titles
            .iter()
            .enumerate()
            .map(|(idx, jt)| {
                let selected = app
                    .job_title_list_state
                    .selected()
                    .map(|s| s == idx)
                    .unwrap_or(false);

                let marker = if selected { "[x] " } else { "[ ] " };
                let content = format!("{}{}", marker, jt.title);

                ListItem::new(Line::from(content))
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::bordered().title(" Available Roles "))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[1], &mut app.job_title_list_state);

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

fn render_education_screen(frame: &mut Frame, app: &mut App) {
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

fn render_experience_screen(frame: &mut Frame, app: &mut App) {
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

fn render_projects_screen(frame: &mut Frame, app: &mut App) {
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

fn render_generating_screen(frame: &mut Frame) {
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

fn render_success_screen(frame: &mut Frame, path: &str) {
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

fn render_error_screen(frame: &mut Frame, error: &str) {
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
