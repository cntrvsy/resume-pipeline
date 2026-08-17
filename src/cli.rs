use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "resume-pipeline",
    author,
    version,
    about = "CLI & TUI tool for targeted resume generation"
)]
pub struct Cli {
    /// Path to a Selection Preset YAML file (e.g. data/presets/job_match.yaml)
    #[arg(short, long)]
    pub preset: Option<String>,

    /// Quick job title selection override (e.g. "Rust Developer")
    #[arg(short, long)]
    pub job: Option<String>,

    /// Non-interactive export: path or directory to save compiled PDF
    #[arg(short, long)]
    pub export: Option<String>,

    /// Dump machine-readable YAML preset schema for AI discovery
    #[arg(long)]
    pub dump_schema: bool,

    /// Render initial TUI frame to stdout as ASCII text for AI inspection
    #[arg(long)]
    pub dump_screen: bool,
}

pub fn dump_preset_schema() -> String {
    r#"# Resume Pipeline Selection Preset Schema
# Use this schema when generating tailored selection presets from job descriptions.
# Note: Item selection uses substring matching against data/*.yaml master files.

job_title: "Fullstack Engineer" # Target title in data/jobtitles.yaml

# Optional custom professional summary override
professional_summary: "Highly skilled Software Engineer specializing in Rust and distributed systems."

# List of project title substrings to include from data/projects.yaml
projects:
  - "Distributed Task Scheduler"
  - "P2P File Transfer Protocol"

# List of education institution substrings to include from data/education.yaml
education:
  - "University of Tech"

# Experience bullet filters matching data/experience.yaml
experience:
  - company: "CloudScale Systems" # Company name substring match
    bullets: # Bullet keyword/phrase substring filters
      - "Redis caching"
      - "CI/CD best practices"

# Profile visibility toggles
profile:
  show_email: true
  show_phone: true
"#
    .to_string()
}
