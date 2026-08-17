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
- **Non-Interactive Batch Mode**: Headless PDF generation for scripts and automated pipelines.
- **Typst Integration**: High-quality PDF generation via custom templates.

## AI Tailored Resume Workflow

You can use any LLM (e.g. Gemini, ChatGPT, Claude) to analyze a target **Job Description**, match it against your `data/*.yaml` source files, and generate a tailored selection preset file:

```bash
# 1. Discover the selection preset schema
resume-pipeline --dump-schema > preset_schema.yaml

# 2. Have your AI generate a tailored preset (e.g. data/presets/job_match.yaml)

# 3. Launch TUI with AI pre-selections for review:
resume-pipeline --preset data/presets/job_match.yaml

# OR generate PDF headlessly:
resume-pipeline --preset data/presets/job_match.yaml --export data/output/tailored.pdf
```

### System Prompt Template for AI Agents

> "You are an expert resume curator. I am applying for the following job:
> `[PASTE JOB DESCRIPTION]`
>
> Inspect my source files in `data/` (`jobtitles.yaml`, `experience.yaml`, `projects.yaml`, `education.yaml`, `profile.yaml`).
> Select the best matching job title, projects, education, and experience bullet keywords.
> Output a Selection Preset YAML file adhering to `resume-pipeline --dump-schema`. Do not invent or hallucinate facts; only select items present in `data/*.yaml`."

## CLI Commands & Flags

| Flag                    | Description                                                     |
| :---------------------- | :-------------------------------------------------------------- |
| `-p`, `--preset <FILE>` | Load a Selection Preset YAML file and apply pre-selections      |
| `-j`, `--job <TITLE>`   | Quick job title selection override                              |
| `-e`, `--export [PATH]` | Run headlessly and export compiled PDF directly                 |
| `--dump-schema`         | Print machine-readable YAML preset schema for AI discovery      |
| `--dump-screen`         | Render initial TUI frame to stdout as ASCII text for inspection |

## Keyboard Shortcuts

| Key                    | Action                                |
| :--------------------- | :------------------------------------ |
| `Enter`                | Proceed / Generate PDF                |
| `Backspace`            | Previous screen                       |
| `Space`                | Toggle item inclusion                 |
| `j` / `k` or `↑` / `↓` | Navigate selection                    |
| `e` / `→`              | Drill down into experience bullets    |
| `e` / `p`              | Toggle Email / Phone (Profile screen) |
| `q`                    | Quit                                  |

## Project Structure

```text
.
├── data/
│   ├── profile.yaml      # Personal info
│   ├── experience.yaml   # Work history (toggable bullets)
│   ├── jobtitles.yaml    # Titles & summaries
│   ├── presets/          # AI selection presets (.yaml)
│   └── templates/        # Typst (.typ) templates
├── output/               # Generated PDFs
└── src/                  # Rust source code
```

## 📄 License

MIT © 2026 [frstudios.co.ke](https://frstudios.co.ke). Forking is welcome; pull requests are not currently accepted.
