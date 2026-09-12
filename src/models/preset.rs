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
    #[serde(default, alias = "company")]
    pub target_company: Option<String>,
    pub job_title: Option<String>,
    pub professional_summary: Option<String>,
    pub skills: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub projects: Option<Vec<ProjectFilterItem>>,
    pub education: Option<Vec<String>>,
    pub experience: Option<Vec<ExperienceFilter>>,
    pub profile: Option<ProfileFilter>,
    pub cover_letter: Option<CoverLetterPreset>,
    pub layout: Option<crate::models::types::LayoutConfig>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LayoutTelemetry {
    pub resume_pages: usize,
    pub cover_letter_pages: Option<usize>,
    pub max_resume_pages: Option<usize>,
    pub max_cover_letter_pages: Option<usize>,
    pub resume_overflow: bool,
    pub cover_letter_overflow: bool,
}

impl LayoutTelemetry {
    pub fn has_overflow(&self) -> bool {
        self.resume_overflow || self.cover_letter_overflow
    }
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
    pub target_company: Option<String>,
    pub layout_telemetry: Option<LayoutTelemetry>,
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
            || self.layout_telemetry.as_ref().map_or(false, |lt| lt.has_overflow())
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

        if let Some(ref comp) = self.target_company {
            println!("│  ✓ Target Company: {}", comp);
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

        if let Some(ref lt) = self.layout_telemetry {
            let mut layout_parts = Vec::new();
            let resume_info = match lt.max_resume_pages {
                Some(max) => format!("Resume: {} page(s) (Limit: {})", lt.resume_pages, max),
                None => format!("Resume: {} page(s)", lt.resume_pages),
            };
            layout_parts.push(resume_info);

            if let Some(cl_pages) = lt.cover_letter_pages {
                let cl_info = match lt.max_cover_letter_pages {
                    Some(max) => format!("Cover Letter: {} page(s) (Limit: {})", cl_pages, max),
                    None => format!("Cover Letter: {} page(s)", cl_pages),
                };
                layout_parts.push(cl_info);
            }

            if lt.has_overflow() {
                println!("│  ✗ Layout Overflow: {}", layout_parts.join(" | "));
            } else {
                println!("│  ✓ Layout: {}", layout_parts.join(" | "));
            }
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
        let mut validation_obj = serde_json::json!({
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
            "target_company": self.target_company,
            "warnings": self.warnings
        });

        if let Some(ref lt) = self.layout_telemetry {
            if let Some(obj) = validation_obj.as_object_mut() {
                obj.insert(
                    "layout".to_string(),
                    serde_json::json!({
                        "resume_pages": lt.resume_pages,
                        "cover_letter_pages": lt.cover_letter_pages,
                        "max_resume_pages": lt.max_resume_pages,
                        "max_cover_letter_pages": lt.max_cover_letter_pages,
                        "resume_overflow": lt.resume_overflow,
                        "cover_letter_overflow": lt.cover_letter_overflow,
                        "status": if lt.has_overflow() { "overflow" } else { "ok" }
                    }),
                );
            }
        }

        serde_json::json!({
            "status": status,
            "output_path": output_path,
            "validation": validation_obj
        })
    }
}

fn match_score(target: &str, query: &str) -> Option<u8> {
    let t = target.to_lowercase();
    let q = query.to_lowercase();
    if t == q {
        Some(3)
    } else if t.contains(&q) {
        Some(2)
    } else if q.contains(&t) {
        Some(1)
    } else {
        None
    }
}

impl ResumeData {
    pub fn apply_preset(&mut self, preset: &SelectionPreset) -> ValidationReport {
        let mut report = ValidationReport::default();

        // 0. Target Company & Layout
        self.target_company = preset
            .target_company
            .clone()
            .or_else(|| preset.cover_letter.as_ref().map(|cl| cl.company.clone()));
        report.target_company = self.target_company.clone();
        self.layout = preset.layout.clone();

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

        // 2. Projects Matching (Preserving Preset Order)
        if let Some(ref req_projects) = preset.projects {
            report.total_projects_requested = req_projects.len();
            for proj in &mut self.projects {
                proj.is_visible = false;
            }

            let mut ordered_projects = Vec::new();
            let mut matched_indices = std::collections::HashSet::new();

            for req_proj in req_projects {
                let req_title = req_proj.title();
                let mut max_score = 0;
                for proj in self.projects.iter() {
                    if let Some(s) = match_score(&proj.title, req_title) {
                        if s > max_score {
                            max_score = s;
                        }
                    }
                }

                let mut matched_any = false;
                if max_score > 0 {
                    for (idx, proj) in self.projects.iter_mut().enumerate() {
                        if match_score(&proj.title, req_title) == Some(max_score) {
                            matched_any = true;
                            proj.is_visible = true;

                            if let Some(req_bullets) = req_proj.bullets() {
                                let mut hidden = Vec::new();
                                for (b_idx, bullet_text) in proj.bullets.iter().enumerate() {
                                    let bullet_lower = bullet_text.to_lowercase();
                                    let is_matched = req_bullets
                                        .iter()
                                        .any(|req| bullet_lower.contains(&req.to_lowercase()));
                                    if !is_matched {
                                        hidden.push(b_idx);
                                    }
                                }
                                proj.hidden_bullets = hidden;
                            }

                            if !matched_indices.contains(&idx) {
                                matched_indices.insert(idx);
                                ordered_projects.push(proj.clone());
                            }
                        }
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
                            .filter(|p| match_score(&p.title, req_title).is_some())
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

            for (idx, proj) in self.projects.iter().enumerate() {
                if !matched_indices.contains(&idx) {
                    ordered_projects.push(proj.clone());
                }
            }
            self.projects = ordered_projects;
        }

        // 3. Education Matching (Preserving Preset Order)
        if let Some(ref req_edu) = preset.education {
            report.total_education_requested = req_edu.len();
            for edu in &mut self.education {
                edu.is_visible = false;
            }

            let mut ordered_education = Vec::new();
            let mut matched_indices = std::collections::HashSet::new();

            for req in req_edu {
                let mut max_score = 0;
                for edu in self.education.iter() {
                    if let Some(s) = match_score(&edu.school, req) {
                        if s > max_score {
                            max_score = s;
                        }
                    }
                }

                let mut matched_any = false;
                if max_score > 0 {
                    for (idx, edu) in self.education.iter_mut().enumerate() {
                        if match_score(&edu.school, req) == Some(max_score) {
                            matched_any = true;
                            edu.is_visible = true;
                            if !matched_indices.contains(&idx) {
                                matched_indices.insert(idx);
                                ordered_education.push(edu.clone());
                            }
                        }
                    }
                }

                if matched_any {
                    if !report.matched_education.contains(req) {
                        report.matched_education.push(req.clone());
                    }
                } else {
                    report.unmatched_education.push(req.clone());
                    report
                        .warnings
                        .push(format!("Education '{}' not found in education.yaml", req));
                }
            }

            for (idx, edu) in self.education.iter().enumerate() {
                if !matched_indices.contains(&idx) {
                    ordered_education.push(edu.clone());
                }
            }
            self.education = ordered_education;
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
