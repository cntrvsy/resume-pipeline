# Resume Pipeline

![PR: not welcome](https://img.shields.io/badge/PR:-not_welcome-red?logo=github) ![fork: welcome](https://img.shields.io/badge/fork:-welcome-brightgreen?logo=github)

A Terminal User Interface (TUI) & CLI application built with **Ratatui** for generating targeted, zero-drift resumes from YAML data sources. Exports professional PDFs using **Typst**.

![resume_pipeline_vid](https://github.com/user-attachments/assets/7cc5fd6d-a41c-4e28-9174-e3769559cfd8)

## Quick Start

### Installation

1. Download the latest binary and `resume-data.zip` from [Releases](https://github.com/cntrvsy/resume-pipeline/releases).
2. Unzip `resume-data.zip` into the same directory as the executable.
3. Run the executable:
   ```bash
   ./resume-pipeline
   ```

### Building from Source

```bash
git clone https://github.com/cntrvsy/resume-pipeline
cd resume-pipeline
cargo run --release
```

## Features

- **Granular Control**: Toggle individual bullet points and sections on the fly.
- **Dynamic Profiles**: Quickly switch between multiple job titles and summaries.
- **YAML Driven**: Maintain your master resume data in simple, version-controllable YAML files.
- **AI Selection Presets (Zero Content Drift)**: Enforce 100% truthful item selection from `data/*.yaml` using keyword/substring matching.
- **Tailored Cover Letters**: Generate professional, matching single-page cover letters without AI cliches, directly aligned to company mission and context.
- **Decoupled Document Export**: Export resume only, cover letter only, or both simultaneously with collision-free naming.
- **Non-Interactive Batch Mode**: Headless PDF generation for scripts and automated pipelines.
- **Typst Integration**: High-quality PDF generation via custom templates.

## AI Tailored Resume & Cover Letter Workflow

You can use any LLM (e.g. Gemini, ChatGPT, Claude) to analyze a target **Job Description**, match it against your `data/*.yaml` source files, and generate a tailored selection preset file:

```bash
# 1. Discover the selection preset schema
resume-pipeline --dump-schema > preset_schema.yaml

# 2. Have your AI generate a tailored preset (e.g. data/presets/job_match.yaml)

# 3. Export both Resume and Cover Letter headlessly:
resume-pipeline --preset data/presets/job_match.yaml --export data/output/

# OR export specific documents:
resume-pipeline --preset data/presets/job_match.yaml --doc resume --export data/output/
resume-pipeline --preset data/presets/job_match.yaml --doc cover-letter --export data/output/

# OR launch TUI with AI pre-selections for visual review:
resume-pipeline --preset data/presets/job_match.yaml
```

### Manual & Human-in-the-Loop Usage

AI generation is completely optional. Because presets are plain-text YAML files, you can write or edit them manually:

1. **Manual Authoring**: Create or copy a preset file (e.g. `data/presets/company_name.yaml`) in any text editor (VS Code, Neovim, etc.):
   ```yaml
   job_title: "Fullstack Engineer"
   cover_letter:
     company: "Acme Corp"
     recipient: "Platform Engineering Team"
     date: "auto" # Resolves to current system date
     paragraphs:
       - "When I read about Acme Corp's work on event-driven infrastructure, it immediately resonated with my background..."
       - "In my previous role, I reduced latency by 40% using Redis caching strategies..."
       - "I'd welcome the chance to discuss how my hands-on background can support the team's engineering goals."
     sign_off: "Sincerely,"
   ```
2. **Human Review Before Compilation**: When an AI drafts a preset for you, you retain 100% human oversight. Simply open the generated YAML file, tweak or rewrite any sentences, remove unwanted fluff, and then run:
   ```bash
   resume-pipeline --preset data/presets/company_name.yaml --export data/output/
   ```

### System Prompt Template for AI Agents

Copy and paste this prompt to an LLM along with the job description:

```markdown
You are an expert technical career strategist and resume curator.
I am applying for the following job:
"""
[PASTE JOB DESCRIPTION HERE]
"""

### Context & Source Data

Inspect my source data files in `data/`:

- `data/jobtitles.yaml` (available titles, summaries, and skill sets)
- `data/experience.yaml` (work history and verifiable bullet points)
- `data/projects.yaml` (technical projects and architecture details)
- `data/education.yaml` (educational background)
- `data/profile.yaml` (personal contact details)

Adhere strictly to the schema from `resume-pipeline --dump-schema`.

### Requirements

#### 1. Resume Selection (Zero Content Drift)

- Select the best matching `job_title` from `data/jobtitles.yaml`.
- Craft a tailored 2-3 sentence `professional_summary` highlighting relevant core strengths.
- Select matching `projects`, `education`, and specific `experience` bullet keyword substrings.
- **CRITICAL RULE**: Do not invent facts or hallucinate tools/roles. Only select items present in `data/*.yaml`.

#### 2. Cover Letter Generation (Human, Non-Robotic Tone)

If the application benefits from a cover letter, populate the `cover_letter:` block:

- `company`: The target company name.
- `recipient`: The hiring team or manager (e.g. "Platform Engineering Team", "Jane Doe").
- `date`: Set to "auto" (or specify current date).
- `subject`: Concise subject line (e.g. "Application for Senior Distributed Systems Engineer").
- `paragraphs`: Exactly 3 to 4 concise paragraphs (250 - 320 words total, strictly fitting on one page):
  - **Paragraph 1 (The Hook & Company Alignment)**: Do NOT use generic pleasantries. Point directly to an actual technical challenge, public product initiative, architecture shift, or company value extracted from the job description and explain why it resonates with your engineering focus.
  - **Paragraph 2 (The Proven Anchor)**: Anchor your technical qualifications around 1 or 2 concrete projects or experience bullets from `data/experience.yaml` or `data/projects.yaml`. Highlight the problem, your approach, and measurable outcome (e.g., latency reduction, reliability, throughput).
  - **Paragraph 3 (The Alignment & Value Add)**: Connect how your hands-on perspective bridges what the team is currently building or scaling.
  - **Paragraph 4 (Call to Action)**: A direct, professional closing expressing enthusiasm to discuss technical roadmaps.

#### 3. Tone & Anti-AI-ese Guidelines

- **VOICE**: Write in the direct, active first-person voice of a competent senior engineer speaking to an engineering peer or manager.
- **STRICT NEGATIVE CONSTRAINTS (BANNED PHRASES)**:
  Do NOT use any of these AI cliches:
  - "I am writing to express my enthusiastic interest in..."
  - "I am confident that my unique blend/synergy of..."
  - "A proven track record of driving..."
  - "A testament to my dedication..."
  - "Thrilled by the prospect of...", "delve", "foster", "pivotal", "spearhead", "beacon".
- Avoid generic corporate praise like "I have always admired your commitment to innovation." Be specific or omit it.
- No M dashes or sentences that follow the structure "Its not X but Y"

### Output Format

Output ONLY valid YAML matching `resume-pipeline --dump-schema`. Do not wrap with conversational chatter.
```

## CLI Commands & Flags

| Flag                    | Description                                                                       |
| :---------------------- | :-------------------------------------------------------------------------------- |
| `-p`, `--preset <FILE>` | Load a Selection Preset YAML file and apply pre-selections                        |
| `-j`, `--job <TITLE>`   | Quick job title selection override                                                |
| `-e`, `--export [PATH]` | Run headlessly and export compiled PDF(s) directly                                |
| `--doc <TYPE>`          | Document to export: `resume`, `cover-letter`, `both`, or `auto` (default: `auto`) |
| `--dump-schema`         | Print machine-readable YAML preset schema for AI discovery                        |
| `--dump-screen`         | Render initial TUI frame to stdout as ASCII text for inspection                   |
| `--list-items`          | List all selectable job titles, projects, education, and experience bullets       |
| `-c`, `--validate`      | Dry-run preset validation without compiling PDF or launching TUI                  |
| `--json`                | Output machine-readable JSON status and diagnostics                               |

## Keyboard Shortcuts

| Key                    | Action                                |
| :--------------------- | :------------------------------------ |
| `Enter`                | Proceed / Generate PDF                |
| `Backspace`            | Previous screen                       |
| `Space`                | Toggle item inclusion                 |
| `j` / `k` or `↑` / `↓` | Navigate selection                    |
| `e` / `→`              | Drill down into bullets               |
| `e` / `p`              | Toggle Email / Phone (Profile screen) |
| `q`                    | Quit                                  |

## Project Structure

```text
.
├── data/
│   ├── profile.yaml                       # Personal info
│   ├── experience.yaml                    # Work history (toggable bullets)
│   ├── jobtitles.yaml                     # Titles & summaries
│   ├── presets/                           # AI selection presets (.yaml)
│   └── templates/
│       ├── default_resume_template.typ       # Resume Typst template
│       └── default_cover_letter_template.typ # Matching Cover Letter template
├── output/                                # Generated PDFs
└── src/                                   # Rust source code
```

## 📄 License

MIT © 2026 [frstudios.co.ke](https://frstudios.co.ke). Forking is welcome; pull requests are not currently accepted.
