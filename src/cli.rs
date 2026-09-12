use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "resume-pipeline",
    author,
    version,
    about = "CLI & TUI tool for targeted resume generation"
)]
pub struct Cli {
    /// Optional command (e.g. "list")
    #[arg(value_name = "COMMAND")]
    pub command: Option<String>,

    /// Path to a Selection Preset YAML file (e.g. data/presets/job_match.yaml)
    #[arg(short, long)]
    pub preset: Option<String>,

    /// Quick job title selection override (e.g. "Rust Developer")
    #[arg(short, long)]
    pub job: Option<String>,

    /// Quick target company override (included in PDF filenames, e.g. "CloudScale Systems")
    #[arg(long)]
    pub company: Option<String>,

    /// Enforce maximum page budget constraint for the resume during validation (e.g. 1 or 2)
    #[arg(long, value_name = "PAGES")]
    pub max_pages: Option<usize>,

    /// Non-interactive export: path or directory to save compiled PDF
    #[arg(short, long)]
    pub export: Option<String>,

    /// Document type to export: "resume", "cover-letter", or "both" (default: "auto", exports both if cover_letter is present)
    #[arg(long, value_name = "TYPE", default_value = "auto")]
    pub doc: String,

    /// Dump machine-readable YAML preset schema for AI discovery
    #[arg(long)]
    pub dump_schema: bool,

    /// Render initial TUI frame to stdout as ASCII text for AI inspection
    #[arg(long)]
    pub dump_screen: bool,

    /// Machine-readable JSON output mode for AI agents
    #[arg(long)]
    pub json: bool,

    /// Dry-run preset validation without compiling PDF or launching TUI
    #[arg(short = 'c', long, alias = "check")]
    pub validate: bool,

    /// List all selectable job titles, projects, education, and experience bullets
    #[arg(long, alias = "dump-data")]
    pub list_items: bool,
}

pub fn dump_preset_schema() -> String {
    r#"# Resume Pipeline Selection Preset Schema
# Use this schema when generating tailored selection presets from job descriptions.
# Note: Item selection uses substring matching against data/*.yaml master files.

# Target company name (included in PDF filenames to avoid collisions/overwrites across applications)
target_company: "CloudScale Systems"

job_title: "Fullstack Engineer" # Target title in data/jobtitles.yaml

# Optional custom professional summary override
professional_summary: "Highly skilled Software Engineer specializing in Rust and distributed systems."

# Optional custom skills override (overrides default job_title skills)
skills:
  Languages: ["Rust", "Python", "TypeScript", "SQL"]
  Systems & Tooling: ["Tokio", "Ratatui", "Typst Engine", "Axum", "gRPC"]
  AI & Security: ["QLoRA", "Ollama (GGUF)", "Hugging Face", "InjecAgent"]
  Cloud & DevOps: ["Docker", "Linux", "GitHub Actions CI/CD"]

# Optional layout and page-budget configuration
layout:
  section_order: ["summary", "skills", "experience", "projects", "education"] # Can include "pagebreak"
  max_pages: 2 # Maximum target page count for resume validation (e.g. 1 or 2)

# List of project title substrings or detailed bullet filters from data/projects.yaml
# NOTE on Project Ordering & Content:
# - Projects are rendered in the EXACT order they appear in this preset list.
# - If 'bullets' is omitted, all master project bullets are rendered (or project 'description' if bullets are empty).
# - If specific 'bullets' are listed, ONLY those matching bullets are rendered (suppressing 'description').
projects:
  - "Securing SME Agent Skills" # Simple title substring match (includes all project bullets)
  - title: "Terminal Based Resume Generator" # Detailed match with specific bullet filters
    bullets:
      - "high-performance"
      - "sub-second latency"

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

# Optional tailored cover letter block (Omit entirely if not required for application)
cover_letter:
  company: "CloudScale Systems" # Target company name
  recipient: "Platform Engineering Team" # Specific team or hiring manager name (optional)
  date: "auto" # "auto" dynamically resolves today's date (or specify e.g. "September 2026")
  address: # Optional company address lines
    - "100 Innovation Way"
    - "San Francisco, CA"
  subject: "Application for Senior Distributed Systems Engineer" # Optional subject line override
  paragraphs: # 3-4 concise, non-cliche paragraphs (250-320 words total)
    - "When I read about CloudScale's ongoing migration to event-driven architectures, it immediately resonated with my background in high-throughput backend services..."
    - "At my previous role, I led the cache invalidation initiative using Redis and Tokio, reducing 99th-percentile tail latency by 40%..."
    - "I'd welcome the chance to discuss how my hands-on background in Rust and infrastructure reliability can support your team's roadmap."
  sign_off: "Sincerely," # Optional sign-off greeting (defaults to "Sincerely,")
"#
    .to_string()
}
