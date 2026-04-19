# Resume Pipeline

![PR: not welcome](https://img.shields.io/badge/PR:-not_welcome-red?logo=github) ![fork: welcome](https://img.shields.io/badge/fork:-welcome-brightgreen?logo=github)

A Terminal User Interface (TUI) application built with **Ratatui** for generating tailored resumes from YAML data sources. Exports professional PDFs using **Typst**.

![resume_pipeline_vid](https://github.com/user-attachments/assets/7cc5fd6d-a41c-4e28-9174-e3769559cfd8)

## 🚀 Quick Start

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

## ✨ Features

- **Granular Control**: Toggle individual bullet points and sections on the fly.
- **Dynamic Profiles**: Quickly switch between multiple job titles and summaries.
- **YAML Driven**: Maintain your resume data in simple, version-controllable YAML files.
- **Typst Integration**: High-quality PDF generation via custom templates.
- **Privacy First**: Selectively omit phone numbers or emails directly from the UI.

## ⌨️ Keyboard Shortcuts

| Key | Action |
| :--- | :--- |
| `Enter` | Proceed / Generate PDF |
| `Backspace` | Previous screen |
| `Space` | Toggle item inclusion |
| `j` / `k` or `↑` / `↓` | Navigate selection |
| `e` / `→` | Drill down into experience bullets |
| `e` / `p` | Toggle Email / Phone (Profile screen) |
| `q` | Quit |

## 📂 Project Structure

```text
.
├── data/
│   ├── profile.yaml      # Personal info
│   ├── experience.yaml   # Work history (toggable bullets)
│   ├── jobtitles.yaml    # Titles & summaries
│   └── templates/        # Typst (.typ) templates
├── output/               # Generated PDFs
└── src/                  # Rust source code
```

## 🔧 Customization

1. **Data**: Edit the YAML files in `data/` to update your info. Keep the existing structure.
2. **Template**: Modify `data/templates/default_resume_template.typ` to change the PDF layout. The template receives filtered data via `sys.inputs`.

## 🛠 Troubleshooting

- **Missing Files**: Ensure the `data/` folder is in the same directory as the binary.
- **YAML Errors**: Validate your YAML syntax if the app fails to load data.
- **PDF Fails**: Check the TUI error screen for Typst compilation messages.

## 📄 License

MIT © 2026 [frstudios.co.ke](https://frstudios.co.ke). Forking is welcome; pull requests are not currently accepted.
