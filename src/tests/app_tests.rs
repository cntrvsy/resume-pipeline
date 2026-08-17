use crate::app::{App, CurrentScreen};
use crate::models::types::{Education, Experience, Profile, Project, JobTitle};
use crossterm::event::KeyCode;

#[test]
fn test_screen_transitions_with_empty_data() {
    let mut app = App::default();

    assert_eq!(app.current_screen, CurrentScreen::Welcome);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ProfileView);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);
    // With empty data, 'e' should not transition to Bullet Selection
    app.handle_key_event(KeyCode::Char('e'));
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::ProjectsSelection);

    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);
}

#[test]
fn test_welcome_to_job_title_transition() {
    let mut app = App::default();
    app.data.job_titles.push(crate::models::types::JobTitle {
        title: "Developer".to_string(),
        professional_summary: "Developer summary".to_string(),
        skills: None,
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

fn create_mock_app() -> App {
    let mut app = App::default();
    let mut skills_map = std::collections::BTreeMap::new();
    skills_map.insert("Languages".to_string(), vec!["Rust".to_string(), "TypeScript".to_string()]);

    app.data.job_titles.push(JobTitle {
        title: "Software Engineer".to_string(),
        professional_summary: "Summary".to_string(),
        skills: Some(skills_map),
    });
    app.data.job_titles.push(JobTitle {
        title: "Senior Software Engineer".to_string(),
        professional_summary: "Senior Summary".to_string(),
        skills: None,
    });

    app.data.profile = Some(Profile {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        phone: "1234567890".to_string(),
        url: "github.com/test".to_string(),
        website: "test.com".to_string(),
        location: "Test City".to_string(),
        citizenship: "Test Citizenship".to_string(),
        show_email: true,
        show_phone: true,
    });

    app.data.education.push(Education {
        school: "Test Uni".to_string(),
        degree: "BS".to_string(),
        status: "Graduated".to_string(),
        is_visible: true,
    });
    app.data.education.push(Education {
        school: "Test Uni 2".to_string(),
        degree: "MS".to_string(),
        status: "Graduated".to_string(),
        is_visible: true,
    });

    app.data.experience.push(Experience {
        role: "Dev".to_string(),
        company: "Corp".to_string(),
        location: "City".to_string(),
        date: "2020-2021".to_string(),
        summary: "Did stuff".to_string(),
        bullets: vec!["Bullet 1".to_string(), "Bullet 2".to_string()],
        is_visible: true,
        hidden_bullets: vec![],
    });

    app.data.projects.push(Project {
        title: "Project 1".to_string(),
        url: None,
        description: "Desc".to_string(),
        tech_stack: vec!["Rust".to_string()],
        is_visible: true,
    });

    app
}

#[test]
fn test_profile_view_toggles() {
    let mut app = create_mock_app();
    app.current_screen = CurrentScreen::ProfileView;

    // Email toggle
    assert!(app.data.profile.as_ref().unwrap().show_email);
    app.handle_key_event(KeyCode::Char('e'));
    assert!(!app.data.profile.as_ref().unwrap().show_email);

    // Phone toggle
    assert!(app.data.profile.as_ref().unwrap().show_phone);
    app.handle_key_event(KeyCode::Char('p'));
    assert!(!app.data.profile.as_ref().unwrap().show_phone);
}

#[test]
fn test_list_navigation_and_toggles() {
    let mut app = create_mock_app();
    
    // Education Selection
    app.current_screen = CurrentScreen::EducationSelection;
    app.education_list_state.select(Some(0));
    
    app.handle_key_event(KeyCode::Char('j'));
    assert_eq!(app.education_list_state.selected(), Some(1));
    app.handle_key_event(KeyCode::Char('k'));
    assert_eq!(app.education_list_state.selected(), Some(0));
    
    // Toggle visibility
    assert!(app.data.education[0].is_visible);
    app.handle_key_event(KeyCode::Char(' '));
    assert!(!app.data.education[0].is_visible);
    
    // Job Title Selection
    app.current_screen = CurrentScreen::JobTitleSelection;
    app.job_title_list_state.select(Some(0));
    
    app.handle_key_event(KeyCode::Down);
    assert_eq!(app.job_title_list_state.selected(), Some(1));
    app.handle_key_event(KeyCode::Up);
    assert_eq!(app.job_title_list_state.selected(), Some(0));

    // Experience Selection
    app.current_screen = CurrentScreen::ExperienceSelection;
    app.experience_list_state.select(Some(0));
    assert!(app.data.experience[0].is_visible);
    app.handle_key_event(KeyCode::Char(' '));
    assert!(!app.data.experience[0].is_visible);
    
    // Projects Selection
    app.current_screen = CurrentScreen::ProjectsSelection;
    app.projects_list_state.select(Some(0));
    assert!(app.data.projects[0].is_visible);
    app.handle_key_event(KeyCode::Char(' '));
    assert!(!app.data.projects[0].is_visible);
}

#[test]
fn test_experience_bullets_navigation() {
    let mut app = create_mock_app();
    app.current_screen = CurrentScreen::ExperienceSelection;
    app.experience_list_state.select(Some(0));
    
    // Go into bullets
    app.handle_key_event(KeyCode::Right);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceBulletSelection);
    assert_eq!(app.experience_bullet_list_state.selected(), Some(0));
    
    // Toggle bullet visibility
    assert!(app.data.experience[0].hidden_bullets.is_empty());
    app.handle_key_event(KeyCode::Char(' '));
    assert!(app.data.experience[0].hidden_bullets.contains(&0));
    
    // Move to next bullet
    app.handle_key_event(KeyCode::Char('j'));
    assert_eq!(app.experience_bullet_list_state.selected(), Some(1));
    
    // Go back via Left
    app.handle_key_event(KeyCode::Left);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);
}

#[test]
fn test_backward_navigation_preserves_state() {
    let mut app = create_mock_app();
    app.current_screen = CurrentScreen::ProfileView;
    
    // Turn off email
    app.handle_key_event(KeyCode::Char('e'));
    assert!(!app.data.profile.as_ref().unwrap().show_email);
    
    // Move to Education
    app.handle_key_event(KeyCode::Enter);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);
    
    // Move back to Profile
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ProfileView);
    
    // State should be preserved
    assert!(!app.data.profile.as_ref().unwrap().show_email);

    // Move back to Job Title
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::JobTitleSelection);
}

#[test]
fn test_quit_from_screens() {
    let mut app = create_mock_app();
    
    let screens_to_test = vec![
        CurrentScreen::Welcome,
        CurrentScreen::JobTitleSelection,
        CurrentScreen::ProfileView,
        CurrentScreen::EducationSelection,
        CurrentScreen::ExperienceSelection,
        CurrentScreen::ExperienceBulletSelection,
        CurrentScreen::ProjectsSelection,
        CurrentScreen::Success("dummy.pdf".to_string()),
        CurrentScreen::Error("dummy error".to_string()),
    ];

    for screen in screens_to_test {
        app.current_screen = screen;
        app.handle_key_event(KeyCode::Char('q'));
        assert_eq!(app.current_screen, CurrentScreen::Exiting);
    }
}

#[test]
fn test_backspace_from_all_screens() {
    let mut app = create_mock_app();

    // ProjectsSelection -> ExperienceSelection
    app.current_screen = CurrentScreen::ProjectsSelection;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    // ExperienceBulletSelection -> ExperienceSelection
    app.current_screen = CurrentScreen::ExperienceBulletSelection;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection);

    // ExperienceSelection -> EducationSelection
    app.current_screen = CurrentScreen::ExperienceSelection;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::EducationSelection);

    // EducationSelection -> ProfileView
    app.current_screen = CurrentScreen::EducationSelection;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::ProfileView);

    // ProfileView -> JobTitleSelection (since we have job titles)
    app.current_screen = CurrentScreen::ProfileView;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::JobTitleSelection);

    // JobTitleSelection -> Welcome
    app.current_screen = CurrentScreen::JobTitleSelection;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::Welcome);

    // ProfileView -> Welcome (if job titles empty)
    app.data.job_titles.clear();
    app.current_screen = CurrentScreen::ProfileView;
    app.handle_key_event(KeyCode::Backspace);
    assert_eq!(app.current_screen, CurrentScreen::Welcome);
}

#[test]
fn test_list_wrapping_behavior() {
    let mut app = create_mock_app();
    
    // Education Selection Wrap Around
    app.current_screen = CurrentScreen::EducationSelection;
    app.education_list_state.select(Some(1)); // Last item (since mock has 2)
    
    // Move next (wrap to 0)
    app.handle_key_event(KeyCode::Char('j'));
    assert_eq!(app.education_list_state.selected(), Some(0));
    
    // Move previous (wrap to 1)
    app.handle_key_event(KeyCode::Char('k'));
    assert_eq!(app.education_list_state.selected(), Some(1));
}

#[test]
fn test_empty_list_navigation() {
    let mut app = App::default(); // all lists are empty
    
    // Empty Education
    app.current_screen = CurrentScreen::EducationSelection;
    app.handle_key_event(KeyCode::Char('j'));
    assert_eq!(app.education_list_state.selected(), None);
    app.handle_key_event(KeyCode::Char('k'));
    assert_eq!(app.education_list_state.selected(), None);
    
    // Space should not panic
    app.handle_key_event(KeyCode::Char(' '));
    assert_eq!(app.education_list_state.selected(), None);

    // Empty Experience Bullets Transition Should Not Occur
    app.current_screen = CurrentScreen::ExperienceSelection;
    app.handle_key_event(KeyCode::Char('e'));
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection); // Shouldn't transition
    
    // Test transitioning fails if experience has NO bullets
    app.data.experience.push(crate::models::types::Experience {
        role: "Dev".to_string(),
        company: "Corp".to_string(),
        location: "City".to_string(),
        date: "2020".to_string(),
        summary: "Sum".to_string(),
        bullets: vec![], // Empty bullets
        is_visible: true,
        hidden_bullets: vec![],
    });
    app.experience_list_state.select(Some(0));
    app.handle_key_event(KeyCode::Right);
    assert_eq!(app.current_screen, CurrentScreen::ExperienceSelection); // Shouldn't transition
}

#[test]
fn test_terminal_screens_exit_keys() {
    let mut app = create_mock_app();
    
    let exit_keys = vec![KeyCode::Char('q'), KeyCode::Enter, KeyCode::Esc];

    for &key in &exit_keys {
        app.current_screen = CurrentScreen::Success("dummy.pdf".to_string());
        app.handle_key_event(key);
        assert_eq!(app.current_screen, CurrentScreen::Exiting);

        app.current_screen = CurrentScreen::Error("dummy error".to_string());
        app.handle_key_event(key);
        assert_eq!(app.current_screen, CurrentScreen::Exiting);
    }
}

#[test]
fn test_unmapped_keys_ignored() {
    let mut app = create_mock_app();
    app.current_screen = CurrentScreen::Welcome;

    app.handle_key_event(KeyCode::Char('z'));
    assert_eq!(app.current_screen, CurrentScreen::Welcome);
}

#[test]
fn test_skills_propagation() {
    let mut app = create_mock_app();
    app.data.job_title = Some("Software Engineer".to_string());
    let filtered = app.data.to_filtered_data();

    assert!(filtered.skills.contains_key("Languages"));
    assert_eq!(filtered.skills.get("Languages").unwrap(), &vec!["Rust".to_string(), "TypeScript".to_string()]);
}

#[test]
fn test_pdf_generation() {
    let mut app = create_mock_app();
    app.data.job_title = Some("Software Engineer".to_string());
    let pdf_path = crate::pdf::generate_pdf(&app.data);
    assert!(pdf_path.is_ok(), "PDF generation failed: {:?}", pdf_path.err());
}

#[test]
fn test_preset_application_and_validation() {
    let mut app = create_mock_app();
    let preset = crate::models::SelectionPreset {
        job_title: Some("Software Engineer".to_string()),
        professional_summary: Some("Tailored summary test".to_string()),
        projects: Some(vec!["Project 1".to_string()]),
        education: Some(vec!["Test Uni".to_string()]),
        experience: Some(vec![crate::models::ExperienceFilter {
            company: "Corp".to_string(),
            bullets: Some(vec!["Bullet 1".to_string()]),
        }]),
        profile: Some(crate::models::ProfileFilter {
            show_email: Some(false),
            show_phone: Some(true),
        }),
    };

    let report = app.data.apply_preset(&preset);

    assert_eq!(report.matched_job_title, Some("Software Engineer".to_string()));
    assert_eq!(app.data.professional_summary, Some("Tailored summary test".to_string()));
    assert_eq!(report.matched_projects, vec!["Project 1".to_string()]);
    assert_eq!(report.matched_bullets, 1);
    assert_eq!(app.data.profile.as_ref().unwrap().show_email, false);
}

#[test]
fn test_preset_dump_schema() {
    let schema = crate::cli::dump_preset_schema();
    assert!(schema.contains("job_title:"));
    assert!(schema.contains("experience:"));
    assert!(schema.contains("projects:"));
}

#[test]
fn test_unmatched_diagnostics_and_json_report() {
    let mut app = create_mock_app();
    let preset = crate::models::SelectionPreset {
        job_title: Some("Nonexistent Title".to_string()),
        projects: Some(vec!["Nonexistent Project".to_string()]),
        education: Some(vec!["Nonexistent School".to_string()]),
        experience: Some(vec![crate::models::ExperienceFilter {
            company: "FooBar Baz Inc".to_string(),
            bullets: Some(vec!["Nonexistent Bullet".to_string()]),
        }]),
        ..Default::default()
    };

    let report = app.data.apply_preset(&preset);

    assert!(report.has_unmatched());
    assert_eq!(report.unmatched_job_title, Some("Nonexistent Title".to_string()));
    assert_eq!(report.unmatched_projects, vec!["Nonexistent Project".to_string()]);
    assert_eq!(report.unmatched_education, vec!["Nonexistent School".to_string()]);
    assert_eq!(report.unmatched_companies, vec!["FooBar Baz Inc".to_string()]);
    assert_eq!(
        report.unmatched_bullets,
        vec![crate::models::preset::UnmatchedBullet {
            company: "FooBar Baz Inc".to_string(),
            query: "Nonexistent Bullet".to_string(),
        }]
    );

    let json_val = report.to_json_value("failed", None);
    assert_eq!(json_val["status"], "failed");
    assert_eq!(json_val["validation"]["job_title"]["matched"], false);
    assert_eq!(json_val["validation"]["projects"]["missing"][0], "Nonexistent Project");
}

#[test]
fn test_data_item_listing() {
    let app = create_mock_app();
    let text = app.data.list_items_text();
    assert!(text.contains("=== SELECTABLE JOB TITLES ==="));
    assert!(text.contains("Software Engineer"));
    assert!(text.contains("=== SELECTABLE PROJECTS ==="));
    assert!(text.contains("Project 1"));

    let json = app.data.list_items_json();
    assert_eq!(json["status"], "success");
    assert_eq!(json["job_titles"][0]["title"], "Software Engineer");
    assert_eq!(json["projects"][0]["title"], "Project 1");
}

#[test]
fn test_custom_export_path() {
    let mut app = create_mock_app();
    app.data.job_title = Some("Software Engineer".to_string());

    let custom_target = "data/output/test_custom_cv.pdf";
    let pdf_path = crate::pdf::generate_pdf_with_export(&app.data, Some(custom_target));
    assert!(pdf_path.is_ok(), "Custom PDF export failed: {:?}", pdf_path.err());
    assert_eq!(pdf_path.unwrap(), custom_target);
    assert!(std::path::Path::new(custom_target).exists());

    // Cleanup
    let _ = std::fs::remove_file(custom_target);
}




