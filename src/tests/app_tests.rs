use crate::app::{App, CurrentScreen};
use crossterm::event::KeyCode;

#[test]
fn test_screen_transitions_with_empty_data() {
    // App::default() creates an empty ResumeData, so job_titles is empty
    let mut app = App::default();

    // Initial state
    assert_eq!(app.current_screen, CurrentScreen::Welcome);

    // Welcome -> ProfileView (skips JobTitleSelection because it's empty)
    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ProfileView);

    // ProfileView -> EducationSelection
    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);

    // EducationSelection -> ExperienceSelection
    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    // ExperienceSelection -> ExperienceBulletSelection
    app.handle_key_event(KeyCode::Char('e'));
    assert_eq!(app.current_screen, CurrentScreen::ExperienceBulletSelection);

    // ExperienceBulletSelection -> ExperienceSelection
    app.handle_key_event(KeyCode::Esc);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    // ExperienceSelection -> ProjectsSelection
    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ProjectsSelection);

    // ProjectsSelection -> ExperienceSelection (Backspace)
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    // ExperienceSelection -> EducationSelection (Backspace)
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);
}

#[test]
fn test_welcome_to_job_title_transition() {
    let mut app = App::default();
    // Add a dummy job title so we transition to JobTitleSelection
    app.data.job_titles.push(crate::models::types::JobTitle {
        title: "Developer".to_string(),
        professional_summary: "Developer summary".to_string(),
    });
    app.job_title_list_state.select(Some(0));

    assert_eq!(app.current_screen, CurrentScreen::Welcome);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::JobTitleSelection);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ProfileView);
    assert_eq!(app.data.job_title, Some("Developer".to_string()));
    assert_eq!(app.data.professional_summary, Some("Developer summary".to_string()));
}
