use serde::{Deserialize, Serialize};
use typst::foundations::{Dict, IntoValue, Value};

// 1. HELPER: Defaults new items to "Checked" in the UI
pub fn default_true() -> bool {
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

// Filtered version without UI state fields for Typst
#[derive(Debug, Clone, Serialize)]
pub struct FilteredResumeData {
    pub profile: Profile,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub job_title: String,
}

// Wrapper for education YAML parsing
#[derive(Debug, Deserialize)]
pub struct EducationWrapper {
    pub education: Vec<Education>,
}

// Wrapper for projects YAML parsing
#[derive(Debug, Deserialize)]
pub struct ProjectsWrapper {
    pub projects: Vec<Project>,
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
