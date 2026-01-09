use ratatui::Frame;

use crate::app::{App, CurrentScreen};

mod education;
mod experience;
mod job_titles;
mod profile;
mod projects;
mod status;
mod welcome;

// 5. RENDERING LOGIC
pub fn render_ui(frame: &mut Frame, app: &mut App) {
    match &app.current_screen {
        CurrentScreen::Welcome => welcome::render_welcome_screen(frame),
        CurrentScreen::ProfileView => profile::render_profile_screen(frame, app),
        CurrentScreen::JobTitleSelection => job_titles::render_job_title_screen(frame, app),
        CurrentScreen::EducationSelection => education::render_education_screen(frame, app),
        CurrentScreen::ExperienceSelection => experience::render_experience_screen(frame, app),
        CurrentScreen::ProjectsSelection => projects::render_projects_screen(frame, app),
        CurrentScreen::Generating => status::render_generating_screen(frame),
        CurrentScreen::Success(path) => status::render_success_screen(frame, path),
        CurrentScreen::Error(msg) => status::render_error_screen(frame, msg),
        CurrentScreen::Exiting => {}
    }
}
