use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
pub struct Education {
    pub school: String,
    pub degree: String,
    pub status: String,
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
        // CHANGED: No more unwrap_or_else. We want to see the error!
        let read_yaml = |filename: &str| -> Result<String> {
            let path = Path::new("data").join(filename);

            // Check if file exists first to give a better error message
            if !path.exists() {
                return Err(color_eyre::eyre::eyre!(
                    "File not found at: {:?}. (Looking inside {:?})",
                    path,
                    current_dir
                ));
            }

            let content = fs::read_to_string(&path)?;
            Ok(content)
        };

        // Load Experience
        // We handle the error manually to add context
        match read_yaml("experience.yaml") {
            Ok(exp_str) => {
                if !exp_str.is_empty() {
                    // This will fail if the YAML format is wrong
                    data.experience = serde_yaml::from_str(&exp_str).map_err(|e| {
                        color_eyre::eyre::eyre!("YAML Parsing Error in experience.yaml: {}", e)
                    })?;
                }
            }
            Err(e) => return Err(e), // Propagate the file not found error
        }

        // You can add the others (profile, projects) back similarly later
        // keeping it simple to debug 'experience' first.

        Ok(data)
    }
}
