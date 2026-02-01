use super::types::{
    Education, EducationWrapper, Experience, FilteredResumeData, JobTitle, Prof, ProfWrapper,
    Profile, Project, ProjectsWrapper, Skill, SkillsWrapper,
};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;

// The Master Container
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResumeData {
    pub profile: Option<Profile>,
    pub education: Vec<Education>,
    pub prof: Vec<Prof>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub job_title: Option<String>,
    pub job_titles: Vec<JobTitle>,
}

impl ResumeData {
    // 3. THE LOADER
    // Reads specific files from the 'data/' directory
    pub fn load_from_dir() -> Result<Self> {
        let mut data = ResumeData::default();

        // DEBUG: Print where the app thinks it is running
        //let current_dir = std::env::current_dir()?;
        //eprintln!("DEBUG MESSAGE: I am running in: {:?}", current_dir);

        // Helper to read a file
        let read_yaml = |filename: &str| -> Result<String> {
            // FIX: Use current_dir() to find the folder relative to where the user runs the app
            // instead of baking in the build-server path.
            let current_dir = std::env::current_dir()?;
            let path = current_dir.join("data").join(filename);

            if !path.exists() {
                return Err(color_eyre::eyre::eyre!(
                    "File not found at: {:?}.\nPlease ensure the 'data' folder is in the same directory as the executable.",
                    path
                ));
            }

            Ok(fs::read_to_string(&path)?)
        };

        // Load Profile
        match read_yaml("profile.yaml") {
            Ok(profile_str) => {
                if !profile_str.is_empty() {
                    data.profile = Some(serde_yaml::from_str(&profile_str).map_err(|e| {
                        color_eyre::eyre::eyre!("YAML Parsing Error in profile.yaml: {}", e)
                    })?);
                }
            }
            Err(e) => eprintln!("Warning: Could not load profile.yaml: {}", e),
        }

        // Load Job Titles
        match read_yaml("jobtitles.yaml") {
            Ok(job_titles_str) => {
                if !job_titles_str.is_empty() {
                    data.job_titles = serde_yaml::from_str(&job_titles_str).map_err(|e| {
                        color_eyre::eyre::eyre!("YAML Parsing Error in jobtitles.yaml: {}", e)
                    })?;
                }
            }
            Err(e) => eprintln!("Warning: Could not load jobtitles.yaml: {}", e),
        }

        // Load Education
        match read_yaml("education.yaml") {
            Ok(edu_str) => {
                if !edu_str.is_empty() {
                    // Parse the wrapper structure
                    let wrappers: Vec<EducationWrapper> =
                        serde_yaml::from_str(&edu_str).map_err(|e| {
                            color_eyre::eyre::eyre!("YAML Parsing Error in education.yaml: {}", e)
                        })?;

                    // Flatten the nested structure
                    for wrapper in wrappers {
                        data.education.extend(wrapper.education);
                    }
                }
            }
            Err(e) => eprintln!("Warning: Could not load education.yaml: {}", e),
        }

        // Load Professonal Profile
        match read_yaml("prof.yaml") {
            Ok(edu_str) => {
                if !edu_str.is_empty() {
                    // Parse the wrapper structure
                    let wrappers: Vec<ProfWrapper> =
                        serde_yaml::from_str(&edu_str).map_err(|e| {
                            color_eyre::eyre::eyre!("YAML Parsing Error in prof.yaml: {}", e)
                        })?;

                    // Flatten the nested structure
                    for wrapper in wrappers {
                        data.prof.extend(wrapper.prof);
                    }
                }
            }
            Err(e) => eprintln!("Warning: Could not load prof.yaml: {}", e),
        }

        // Load Skills
        match read_yaml("skills.yaml") {
            Ok(skills_str) => {
                if !skills_str.is_empty() {
                    // Parse the wrapper structure
                    let wrappers: Vec<SkillsWrapper> =
                        serde_yaml::from_str(&skills_str).map_err(|e| {
                            color_eyre::eyre::eyre!("YAML Parsing Error in skills.yaml: {}", e)
                        })?;

                    // Flatten the nested structure
                    for wrapper in wrappers {
                        data.skills.extend(wrapper.skills);
                    }
                }
            }
            Err(e) => eprintln!("Warning: Could not load skills.yaml: {}", e),
        }

        // Load Experience
        match read_yaml("experience.yaml") {
            Ok(exp_str) => {
                if !exp_str.is_empty() {
                    data.experience = serde_yaml::from_str(&exp_str).map_err(|e| {
                        color_eyre::eyre::eyre!("YAML Parsing Error in experience.yaml: {}", e)
                    })?;
                }
            }
            Err(e) => eprintln!("Warning: Could not load experience.yaml: {}", e),
        }

        // Load Projects
        match read_yaml("projects.yaml") {
            Ok(proj_str) => {
                if !proj_str.is_empty() {
                    // Parse the wrapper structure
                    let wrappers: Vec<ProjectsWrapper> =
                        serde_yaml::from_str(&proj_str).map_err(|e| {
                            color_eyre::eyre::eyre!("YAML Parsing Error in projects.yaml: {}", e)
                        })?;

                    // Flatten the nested structure
                    for wrapper in wrappers {
                        data.projects.extend(wrapper.projects);
                    }
                }
            }
            Err(e) => eprintln!("Warning: Could not load projects.yaml: {}", e),
        }

        Ok(data)
    }

    /// Create a filtered dataset with only visible items
    pub fn to_filtered_data(&self) -> FilteredResumeData {
        FilteredResumeData {
            profile: self.profile.clone().unwrap_or_else(|| Profile {
                name: "Unknown".to_string(),
                email: "unknown@example.com".to_string(),
                phone: "N/A".to_string(),
                url: "N/A".to_string(),
                location: "N/A".to_string(),
                citizenship: "N/A".to_string(),
            }),
            education: self
                .education
                .iter()
                .filter(|e| e.is_visible)
                .cloned()
                .collect(),
            prof: self.prof.iter().filter(|e| e.is_visible).cloned().collect(),
            skills: self
                .skills
                .iter()
                .filter(|s| s.is_visible)
                .cloned()
                .collect(),
            experience: self
                .experience
                .iter()
                .filter(|e| e.is_visible)
                .cloned()
                .collect(),
            projects: self
                .projects
                .iter()
                .filter(|p| p.is_visible)
                .cloned()
                .collect(),
            job_title: self.job_title.clone().unwrap_or_else(|| " N/A".to_string()),
        }
    }
}
