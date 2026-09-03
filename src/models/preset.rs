use crate::models::resume::ResumeData;
use crate::models::types::CoverLetterPreset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectFilterItem {
    Simple(String),
    Detailed {
        title: String,
        bullets: Option<Vec<String>>,
    },
}

impl ProjectFilterItem {
    pub fn title(&self) -> &str {
        match self {
            ProjectFilterItem::Simple(s) => s,
            ProjectFilterItem::Detailed { title, .. } => title,
        }
    }

    pub fn bullets(&self) -> Option<&Vec<String>> {
        match self {
            ProjectFilterItem::Simple(_) => None,
            ProjectFilterItem::Detailed { bullets, .. } => bullets.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectionPreset {
    pub job_title: Option<String>,
    pub professional_summary: Option<String>,
    pub skills: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub projects: Option<Vec<ProjectFilterItem>>,
    pub education: Option<Vec<String>>,
    pub experience: Option<Vec<ExperienceFilter>>,
    pub profile: Option<ProfileFilter>,
    pub cover_letter: Option<CoverLetterPreset>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UnmatchedBullet {
    pub company: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UnmatchedProjectBullet {
    pub project: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationReport {
    pub matched_job_title: Option<String>,
    pub requested_job_title: Option<String>,
    pub unmatched_job_title: Option<String>,
    pub matched_projects: Vec<String>,
    pub total_projects_requested: usize,
    pub unmatched_projects: Vec<String>,
    pub matched_project_bullets: usize,
    pub total_project_bullets_requested: usize,
    pub unmatched_project_bullets: Vec<UnmatchedProjectBullet>,
    pub matched_education: Vec<String>,
    pub total_education_requested: usize,
    pub unmatched_education: Vec<String>,
    pub matched_bullets: usize,
    pub total_bullets_requested: usize,
    pub unmatched_companies: Vec<String>,
    pub unmatched_bullets: Vec<UnmatchedBullet>,
    pub cover_letter_company: Option<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn has_unmatched(&self) -> bool {
        self.unmatched_job_title.is_some()
            || !self.unmatched_projects.is_empty()
            || !self.unmatched_project_bullets.is_empty()
            || !self.unmatched_education.is_empty()
            || !self.unmatched_companies.is_empty()
            || !self.unmatched_bullets.is_empty()
    }

    pub fn print_summary(&self) {
        println!("\n┌─────────────────── PRESET VALIDATION REPORT ───────────────────┐");
        if let Some(ref title) = self.matched_job_title {
            println!("│  ✓ Job Title: {}", title);
        } else if let Some(ref req_title) = self.unmatched_job_title {
            println!("│  ✗ Job Title unmatched: \"{}\"", req_title);
        } else {
            println!("│  ⚠ Job Title: Not specified");
        }

        if self.total_projects_requested > 0 {
            println!(
                "│  ✓ Projects: {}/{} matched ({})",
                self.matched_projects.len(),
                self.total_projects_requested,
                if self.matched_projects.is_empty() {
                    "none".to_string()
                } else {
                    self.matched_projects.join(", ")
                }
            );
            for un in &self.unmatched_projects {
                println!("│  ✗ Project unmatched: \"{}\"", un);
            }
        }

        if self.total_project_bullets_requested > 0 {
            println!(
                "│  ✓ Project Bullets: {}/{} keyword matches",
                self.matched_project_bullets, self.total_project_bullets_requested
            );
            for un_b in &self.unmatched_project_bullets {
                println!("│  ✗ Project Bullet unmatched: [{}] \"{}\"", un_b.project, un_b.query);
            }
        }

        if self.total_education_requested > 0 {
            println!(
                "│  ✓ Education: {}/{} matched ({})",
                self.matched_education.len(),
                self.total_education_requested,
                if self.matched_education.is_empty() {
                    "none".to_string()
                } else {
                    self.matched_education.join(", ")
                }
            );
            for un in &self.unmatched_education {
                println!("│  ✗ Education unmatched: \"{}\"", un);
            }
        }

        if self.total_bullets_requested > 0 || !self.unmatched_companies.is_empty() {
            println!(
                "│  ✓ Experience Bullets: {}/{} keyword matches",
                self.matched_bullets, self.total_bullets_requested
            );
            for un_comp in &self.unmatched_companies {
                println!("│  ✗ Company unmatched: \"{}\"", un_comp);
            }
            for un_b in &self.unmatched_bullets {
                println!("│  ✗ Bullet unmatched: [{}] \"{}\"", un_b.company, un_b.query);
            }
        }

        if let Some(ref company) = self.cover_letter_company {
            println!("│  ✓ Cover Letter: Included for \"{}\"", company);
        }

        if !self.warnings.is_empty() {
            println!("├─────────────────────────────────────────────────────────────────┤");
            for warning in &self.warnings {
                println!("│  ⚠ Warning: {}", warning);
            }
        }
        println!("└─────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn to_json_value(&self, status: &str, output_path: Option<&str>) -> serde_json::Value {
        serde_json::json!({
            "status": status,
            "output_path": output_path,
            "validation": {
                "job_title": {
                    "matched": self.matched_job_title.is_some(),
                    "requested": self.requested_job_title,
                    "selected": self.matched_job_title,
                    "unmatched": self.unmatched_job_title
                },
                "projects": {
                    "requested": self.total_projects_requested,
                    "matched": self.matched_projects.len(),
                    "matched_items": self.matched_projects,
                    "missing": self.unmatched_projects
                },
                "project_bullets": {
                    "requested": self.total_project_bullets_requested,
                    "matched": self.matched_project_bullets,
                    "missing": self.unmatched_project_bullets
                },
                "education": {
                    "requested": self.total_education_requested,
                    "matched": self.matched_education.len(),
                    "matched_items": self.matched_education,
                    "missing": self.unmatched_education
                },
                "experience": {
                    "missing_companies": self.unmatched_companies
                },
                "experience_bullets": {
                    "requested": self.total_bullets_requested,
                    "matched": self.matched_bullets,
                    "missing": self.unmatched_bullets
                },
                "cover_letter": {
                    "included": self.cover_letter_company.is_some(),
                    "company": self.cover_letter_company
                },
                "warnings": self.warnings
            }
        })
    }
}

impl ResumeData {
    pub fn apply_preset(&mut self, preset: &SelectionPreset) -> ValidationReport {
        let mut report = ValidationReport::default();

        // 1. Job Title & Professional Summary
        if let Some(ref target_title) = preset.job_title {
            report.requested_job_title = Some(target_title.clone());
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
                report.unmatched_job_title = Some(target_title.clone());
                report
                    .warnings
                    .push(format!("Job title '{}' not found in jobtitles.yaml", target_title));
            }
        }

        // 1b. Skills Override
        if let Some(ref custom_skills) = preset.skills {
            self.custom_skills = Some(custom_skills.clone());
        }

        // 2. Projects Matching
        if let Some(ref req_projects) = preset.projects {
            report.total_projects_requested = req_projects.len();
            for proj in &mut self.projects {
                proj.is_visible = false;
            }

            for req_proj in req_projects {
                let req_title = req_proj.title();
                let req_lower = req_title.to_lowercase();
                let mut matched_any = false;

                for proj in self.projects.iter_mut().filter(|p| {
                    let p_lower = p.title.to_lowercase();
                    p_lower.contains(&req_lower) || req_lower.contains(&p_lower)
                }) {
                    matched_any = true;
                    proj.is_visible = true;

                    if let Some(req_bullets) = req_proj.bullets() {
                        let mut hidden = Vec::new();
                        for (idx, bullet_text) in proj.bullets.iter().enumerate() {
                            let bullet_lower = bullet_text.to_lowercase();
                            let is_matched = req_bullets
                                .iter()
                                .any(|req| bullet_lower.contains(&req.to_lowercase()));
                            if !is_matched {
                                hidden.push(idx);
                            }
                        }
                        proj.hidden_bullets = hidden;
                    }
                }

                if matched_any {
                    if !report.matched_projects.contains(&req_title.to_string()) {
                        report.matched_projects.push(req_title.to_string());
                    }
                } else {
                    report.unmatched_projects.push(req_title.to_string());
                    report
                        .warnings
                        .push(format!("Project '{}' not found in projects.yaml", req_title));
                }

                if let Some(req_bullets) = req_proj.bullets() {
                    report.total_project_bullets_requested += req_bullets.len();
                    for req_bullet in req_bullets {
                        let req_b_lower = req_bullet.to_lowercase();
                        let matched_bullet = self
                            .projects
                            .iter()
                            .filter(|p| {
                                let p_lower = p.title.to_lowercase();
                                p_lower.contains(&req_lower) || req_lower.contains(&p_lower)
                            })
                            .any(|p| {
                                p.bullets
                                    .iter()
                                    .any(|b| b.to_lowercase().contains(&req_b_lower))
                            });

                        if matched_bullet {
                            report.matched_project_bullets += 1;
                        } else {
                            report.unmatched_project_bullets.push(UnmatchedProjectBullet {
                                project: req_title.to_string(),
                                query: req_bullet.clone(),
                            });
                        }
                    }
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
                if matched && !report.matched_education.contains(&edu.school) {
                    report.matched_education.push(edu.school.clone());
                }
            }

            for req in req_edu {
                let req_lower = req.to_lowercase();
                let exists = self.education.iter().any(|e| {
                    let s_lower = e.school.to_lowercase();
                    s_lower.contains(&req_lower) || req_lower.contains(&s_lower)
                });
                if !exists {
                    report.unmatched_education.push(req.clone());
                    report
                        .warnings
                        .push(format!("Education '{}' not found in education.yaml", req));
                }
            }
        }

        // 4. Experience & Bullet Substring Matching
        if let Some(ref exp_filters) = preset.experience {
            for exp_filter in exp_filters {
                let comp_req_lower = exp_filter.company.to_lowercase();

                let has_company_match = self.experience.iter().any(|e| {
                    let c_lower = e.company.to_lowercase();
                    c_lower.contains(&comp_req_lower) || comp_req_lower.contains(&c_lower)
                });

                if has_company_match {
                    for exp in self.experience.iter_mut().filter(|e| {
                        let c_lower = e.company.to_lowercase();
                        c_lower.contains(&comp_req_lower) || comp_req_lower.contains(&c_lower)
                    }) {
                        exp.is_visible = true;

                        if let Some(ref req_bullets) = exp_filter.bullets {
                            let mut hidden = Vec::new();

                            for (idx, bullet_text) in exp.bullets.iter().enumerate() {
                                let bullet_lower = bullet_text.to_lowercase();
                                let is_matched = req_bullets
                                    .iter()
                                    .any(|req| bullet_lower.contains(&req.to_lowercase()));

                                if !is_matched {
                                    hidden.push(idx);
                                }
                            }

                            exp.hidden_bullets = hidden;
                        }
                    }

                    if let Some(ref req_bullets) = exp_filter.bullets {
                        report.total_bullets_requested += req_bullets.len();
                        for req_bullet in req_bullets {
                            let req_b_lower = req_bullet.to_lowercase();
                            let matched_any = self
                                .experience
                                .iter()
                                .filter(|e| {
                                    let c_lower = e.company.to_lowercase();
                                    c_lower.contains(&comp_req_lower)
                                        || comp_req_lower.contains(&c_lower)
                                })
                                .any(|exp| {
                                    exp.bullets
                                        .iter()
                                        .any(|b| b.to_lowercase().contains(&req_b_lower))
                                });

                            if matched_any {
                                report.matched_bullets += 1;
                            } else {
                                report.unmatched_bullets.push(UnmatchedBullet {
                                    company: exp_filter.company.clone(),
                                    query: req_bullet.clone(),
                                });
                            }
                        }
                    }
                } else {
                    report.unmatched_companies.push(exp_filter.company.clone());
                    report.warnings.push(format!(
                        "Company '{}' not found in experience.yaml",
                        exp_filter.company
                    ));
                    if let Some(ref req_bullets) = exp_filter.bullets {
                        report.total_bullets_requested += req_bullets.len();
                        for req_bullet in req_bullets {
                            report.unmatched_bullets.push(UnmatchedBullet {
                                company: exp_filter.company.clone(),
                                query: req_bullet.clone(),
                            });
                        }
                    }
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

        // 6. Cover Letter
        self.cover_letter = preset.cover_letter.clone();
        if let Some(ref cl) = preset.cover_letter {
            report.cover_letter_company = Some(cl.company.clone());
        }

        report
    }
}
