use crossterm::event::KeyCode;
use ratatui::widgets::ListState;
use crate::models::ResumeData;
use crate::pdf::generate_pdf;

// 1. STATE MANAGEMENT
#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
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
pub struct App {
    pub current_screen: CurrentScreen,
    pub data: ResumeData,
    // UI STATE for scrolling through lists
    pub education_list_state: ListState,
    pub experience_list_state: ListState,
    pub projects_list_state: ListState,
    pub job_title_list_state: ListState,
}

impl App {
    pub fn new() -> Self {
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
    pub fn next_education(&mut self) {
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

    pub fn previous_education(&mut self) {
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

    pub fn toggle_education(&mut self) {
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
    pub fn next_experience(&mut self) {
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

    pub fn previous_experience(&mut self) {
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

    pub fn toggle_experience(&mut self) {
        if let Some(i) = self.experience_list_state.selected() {
            if let Some(item) = self.data.experience.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    // Navigation helpers for Projects
    pub fn next_project(&mut self) {
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

    pub fn previous_project(&mut self) {
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

    pub fn toggle_project(&mut self) {
        if let Some(i) = self.projects_list_state.selected() {
            if let Some(item) = self.data.projects.get_mut(i) {
                item.is_visible = !item.is_visible;
            }
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) {
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
