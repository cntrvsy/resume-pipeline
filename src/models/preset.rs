use crate::models::resume::ResumeData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectionPreset {
    pub job_title: Option<String>,
    pub professional_summary: Option<String>,
    pub projects: Option<Vec<String>>,
    pub education: Option<Vec<String>>,
    pub experience: Option<Vec<ExperienceFilter>>,
    pub profile: Option<ProfileFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperienceFilter {
    pub company: String,
    pub bullets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileFilter {
    pub show_email: Option<bool>,
    pub show_phone: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub matched_job_title: Option<String>,
    pub matched_projects: Vec<String>,
    pub total_projects_requested: usize,
    pub matched_education: Vec<String>,
    pub total_education_requested: usize,
    pub matched_bullets: usize,
    pub total_bullets_requested: usize,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn print_summary(&self) {
        println!("\n┌─────────────────── PRESET VALIDATION REPORT ───────────────────┐");
        if let Some(ref title) = self.matched_job_title {
            println!("│  ✓ Job Title: {}", title);
        } else {
            println!("│  ⚠ Job Title: Not specified / Unmatched");
        }

        if self.total_projects_requested > 0 {
            println!(
                "│  ✓ Projects: {}/{} matched ({})",
                self.matched_projects.len(),
                self.total_projects_requested,
                self.matched_projects.join(", ")
            );
        }

        if self.total_education_requested > 0 {
            println!(
                "│  ✓ Education: {}/{} matched",
                self.matched_education.len(),
                self.total_education_requested
            );
        }

        if self.total_bullets_requested > 0 {
            println!(
                "│  ✓ Experience Bullets: {}/{} keyword matches",
                self.matched_bullets, self.total_bullets_requested
            );
        }

        if !self.warnings.is_empty() {
            println!("├─────────────────────────────────────────────────────────────────┤");
            for warning in &self.warnings {
                println!("│  ⚠ Warning: {}", warning);
            }
        }
        println!("└─────────────────────────────────────────────────────────────────┘\n");
    }
}

impl ResumeData {
    pub fn apply_preset(&mut self, preset: &SelectionPreset) -> ValidationReport {
        let mut report = ValidationReport::default();

        // 1. Job Title & Professional Summary
        if let Some(ref target_title) = preset.job_title {
            let target_lower = target_title.to_lowercase();
            if let Some(matched) = self
                .job_titles
                .iter()
                .find(|jt| jt.title.to_lowercase().contains(&target_lower))
            {
                self.job_title = Some(matched.title.clone());
                report.matched_job_title = Some(matched.title.clone());

                if let Some(ref custom_summary) = preset.professional_summary {
                    self.professional_summary = Some(custom_summary.clone());
                } else {
                    self.professional_summary = Some(matched.professional_summary.clone());
                }
            } else {
                report
                    .warnings
                    .push(format!("Job title '{}' not found in jobtitles.yaml", target_title));
            }
        }

        // 2. Projects Matching
        if let Some(ref req_projects) = preset.projects {
            report.total_projects_requested = req_projects.len();
            for proj in &mut self.projects {
                let proj_title_lower = proj.title.to_lowercase();
                let matched = req_projects.iter().any(|req| {
                    let req_lower = req.to_lowercase();
                    proj_title_lower.contains(&req_lower) || req_lower.contains(&proj_title_lower)
                });
                proj.is_visible = matched;
                if matched {
                    report.matched_projects.push(proj.title.clone());
                }
            }

            for req in req_projects {
                let req_lower = req.to_lowercase();
                let exists = self.projects.iter().any(|p| {
                    let p_lower = p.title.to_lowercase();
                    p_lower.contains(&req_lower) || req_lower.contains(&p_lower)
                });
                if !exists {
                    report
                        .warnings
                        .push(format!("Project '{}' not found in projects.yaml", req));
                }
            }
        }

        // 3. Education Matching
        if let Some(ref req_edu) = preset.education {
            report.total_education_requested = req_edu.len();
            for edu in &mut self.education {
                let school_lower = edu.school.to_lowercase();
                let matched = req_edu.iter().any(|req| {
                    let req_lower = req.to_lowercase();
                    school_lower.contains(&req_lower) || req_lower.contains(&school_lower)
                });
                edu.is_visible = matched;
                if matched {
                    report.matched_education.push(edu.school.clone());
                }
            }
        }

        // 4. Experience & Bullet Substring Matching
        if let Some(ref exp_filters) = preset.experience {
            for exp_filter in exp_filters {
                let comp_req_lower = exp_filter.company.to_lowercase();

                if let Some(exp) = self.experience.iter_mut().find(|e| {
                    let c_lower = e.company.to_lowercase();
                    c_lower.contains(&comp_req_lower) || comp_req_lower.contains(&c_lower)
                }) {
                    exp.is_visible = true;

                    if let Some(ref req_bullets) = exp_filter.bullets {
                        report.total_bullets_requested += req_bullets.len();
                        let mut hidden = Vec::new();

                        for (idx, bullet_text) in exp.bullets.iter().enumerate() {
                            let bullet_lower = bullet_text.to_lowercase();
                            let is_matched = req_bullets
                                .iter()
                                .any(|req| bullet_lower.contains(&req.to_lowercase()));

                            if is_matched {
                                report.matched_bullets += 1;
                            } else {
                                hidden.push(idx);
                            }
                        }

                        exp.hidden_bullets = hidden;
                    }
                } else {
                    report.warnings.push(format!(
                        "Company '{}' not found in experience.yaml",
                        exp_filter.company
                    ));
                }
            }
        }

        // 5. Profile Toggles
        if let Some(ref prof_filter) = preset.profile {
            if let Some(ref mut prof) = self.profile {
                if let Some(show) = prof_filter.show_email {
                    prof.show_email = show;
                }
                if let Some(show) = prof_filter.show_phone {
                    prof.show_phone = show;
                }
            }
        }

        report
    }
}
