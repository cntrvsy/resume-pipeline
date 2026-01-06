use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use typst::foundations::{Dict, IntoValue, Value};

// 1. HELPER: Defaults new items to "Checked" in the UI
fn default_true() -> bool {
    true
}

// 2. DATA STRUCTURES

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub url: String,
    pub location: String,
    pub citizenship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTitle {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub status: String,

    // UI STATE
    #[serde(skip, default = "default_true")]
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub role: String,
    pub company: String,
    pub location: String,
    pub date: String,
    pub summary: String,
    pub bullets: Vec<String>,

    // UI STATE: Not in YAML, only in App
    #[serde(skip, default = "default_true")]
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,

    // UI STATE
    #[serde(skip, default = "default_true")]
    pub is_visible: bool,
}

// The Master Container
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResumeData {
    pub profile: Option<Profile>,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub job_title: Option<String>,
    pub job_titles: Vec<JobTitle>,
}

// Wrapper for education YAML parsing
#[derive(Debug, Deserialize)]
struct EducationWrapper {
    education: Vec<Education>,
}

// Wrapper for projects YAML parsing
#[derive(Debug, Deserialize)]
struct ProjectsWrapper {
    projects: Vec<Project>,
}

impl ResumeData {
    // 3. THE LOADER
    // Reads specific files from the 'data/' directory
    pub fn load_from_dir() -> Result<Self> {
        let mut data = ResumeData::default();

        // DEBUG: Print where the app thinks it is running
        let current_dir = std::env::current_dir()?;
        eprintln!("DEBUG: Running in: {:?}", current_dir);

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

// Filtered version without UI state fields for Typst
#[derive(Debug, Clone, Serialize)]
pub struct FilteredResumeData {
    profile: Profile,
    education: Vec<Education>,
    experience: Vec<Experience>,
    projects: Vec<Project>,
    pub job_title: String,
}

// Manual implementation of IntoValue/IntoDict to resolve version conflicts

impl IntoValue for Profile {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("name".into(), self.name.into_value());
        dict.insert("email".into(), self.email.into_value());
        dict.insert("phone".into(), self.phone.into_value());
        dict.insert("url".into(), self.url.into_value());
        dict.insert("location".into(), self.location.into_value());
        dict.insert("citizenship".into(), self.citizenship.into_value());
        Value::Dict(dict)
    }
}

impl IntoValue for Education {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("school".into(), self.school.into_value());
        dict.insert("degree".into(), self.degree.into_value());
        dict.insert("status".into(), self.status.into_value());
        Value::Dict(dict)
    }
}

impl IntoValue for Experience {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("role".into(), self.role.into_value());
        dict.insert("company".into(), self.company.into_value());
        dict.insert("location".into(), self.location.into_value());
        dict.insert("date".into(), self.date.into_value());
        dict.insert("summary".into(), self.summary.into_value());
        dict.insert("bullets".into(), self.bullets.into_value());
        Value::Dict(dict)
    }
}

impl IntoValue for Project {
    fn into_value(self) -> Value {
        let mut dict = Dict::new();
        dict.insert("title".into(), self.title.into_value());
        dict.insert("description".into(), self.description.into_value());
        dict.insert("tech_stack".into(), self.tech_stack.into_value());
        Value::Dict(dict)
    }
}

impl From<FilteredResumeData> for Dict {
    fn from(val: FilteredResumeData) -> Self {
        let mut dict = Dict::new();
        dict.insert("profile".into(), val.profile.into_value());
        dict.insert("education".into(), val.education.into_value());
        dict.insert("experience".into(), val.experience.into_value());
        dict.insert("projects".into(), val.projects.into_value());
        dict.insert("job_title".into(), val.job_title.into_value());
        dict
    }
}
