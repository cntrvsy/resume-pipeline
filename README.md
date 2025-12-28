# Resume Pipeline

A Terminal User Interface (TUI) application built with Ratatui that generates customized resumes from YAML data sources and exports them as PDFs using Typst.

## Why?

- A general resume doesn't cut it anymore. As someone who has studied Applied Computing Technology with a focus on Software Development, most hiring managers expect a more tailored and personalized resume. It's no longer just "front-end engineer" or "back-end engineer" but a "product implementation engineer" (yes, this was an actual job title and role involved basically creating UIs in Svelte, yeah...).
- The job market is competitive, AI is great but that means it's harder to stand out, and having a personally crafted resume for each job you apply for can make a significant difference in getting noticed by potential employers as most recruiters apparently don't spend more than 20 seconds scanning through your resume.
- This application allows users to create custom resumes by selecting relevant experience, education, and projects from YAML files, which are then formatted and styled using Typst templates.

## Why not use Word or Google Docs?

It's time consuming and depressing to spend 2 hours on a resume to not even get a rejection email. Plus depending on the Recruiter "influencer" you follow, some styles are just difficult to make work and fit your content requiring you to spend even more time on whatever [method](https://www.evidenced.app/blog/8-alternatives-to-star-method) they swear by. This application's goal is to allow users to create custom resumes quickly and easily by just selecting which points are relevant to said job title. Data is stored in YAML files, and the final PDF is generated using Typst.

## Features

- **Interactive TUI**: Navigate through your resume data with an intuitive terminal interface
- **Multi-section selection**: Choose from your profile, education, experience, and projects
- **Live selection**: Toggle which items to include in your final resume using checkboxes
- **PDF export**: Automatically generates a professional PDF using Typst templates
- **YAML-based data**: Easy to maintain and version control your resume data

## Installation

### Prerequisites

- Rust toolchain (1.70 or later)
- Cargo package manager

### Build from source

```bash
git clone <repository-url>
cd resume-pipeline
cargo build --release
```

The binary will be available at `target/release/resume-pipeline`

## Usage

### 1. Prepare Your Data

Create YAML files in the `data/` directory:

- `data/profile.yaml` - Your personal information
- `data/education.yaml` - Educational background
- `data/experience.yaml` - Work experience
- `data/projects.yaml` - Side projects and portfolio

Example structure for `experience.yaml`:

```yaml
- role: "Software Developer"
  company: "Company Name"
  location: "City, Country"
  date: "June 2023 - Current"
  summary: "Brief description of role"
  bullets:
    - "Achievement or responsibility 1"
    - "Achievement or responsibility 2"
```

### 2. Run the Application

```bash
cargo run --release
# or if installed
./target/release/resume-pipeline
```

### 3. Navigate the Interface

1. **Welcome Screen**: Press `Enter` to start
2. **Profile View**: Review your personal information, press `Enter` to continue
3. **Education Selection**: Use `j`/`k` or arrow keys to navigate, `Space` to toggle selection
4. **Experience Selection**: Same navigation, select relevant work experience
5. **Projects Selection**: Choose projects to include in your resume
6. **Generate**: Press `Enter` on the final screen to generate your PDF

### 4. Find Your Resume

Your generated resume will be saved to `output/resume.pdf`

## Keyboard Shortcuts

- `Enter` - Proceed to next screen / Generate PDF
- `Backspace` - Go back to previous screen
- `j` / `Down Arrow` - Move selection down
- `k` / `Up Arrow` - Move selection up
- `Space` - Toggle item inclusion
- `q` - Quit application

## Customizing the Template

The Typst template is located at `.github/templates/headless_head_hunter.typ`. You can modify this file to change the appearance and layout of your resume.

The template receives your filtered data directly from Rust via Typst's `sys.inputs` mechanism - no JSON files needed! The data is passed in-memory using the `compile_with_input()` function.

## Project Structure

```
resume-pipeline/
├── data/                   # YAML data files
│   ├── profile.yaml
│   ├── education.yaml
│   ├── experience.yaml
│   └── projects.yaml
├── .github/templates/      # Typst templates
│   └── headless_head_hunter.typ
├── src/
│   ├── main.rs            # TUI application logic
│   └── models.rs          # Data structures and YAML parsing
└── output/                # Generated PDFs
    └── resume.pdf         # Your generated resume
```

## Technologies Used

- **Ratatui**: Terminal UI framework
- **Crossterm**: Cross-platform terminal manipulation
- **Serde**: Serialization/deserialization for YAML
- **Typst**: Document typesetting and PDF generation
- **typst-as-lib**: Rust wrapper for Typst compiler
- **derive_typst_intoval**: Automatic Rust-to-Typst type conversion
- **Color-eyre**: Error handling and reporting

## Troubleshooting

### "No data found" errors

If you see errors about missing YAML files, ensure:
1. You're running the application from the project root directory
2. The `data/` directory exists with all required YAML files
3. YAML files are properly formatted (use a YAML validator if needed)

### PDF generation fails

If PDF generation fails:
1. Check the error message on the error screen
2. Verify the template exists at `.github/templates/headless_head_hunter.typ`
3. Ensure the `output/` directory is writable
4. Check that `output/payload.json` was created and contains valid JSON

### Template not receiving data

The Typst template receives data directly from Rust via `sys.inputs`. This is handled automatically by:
1. Filtering the data to only visible items
2. Converting the Rust structs to Typst Dict using `derive_typst_intoval`
3. Passing data with `compile_with_input(filtered_data)`

If this fails, check:
- All data structs properly derive `DeriveIntoValue` and `IntoDict`
- The template uses `#import sys: inputs` to access the data

### Navigation doesn't work

Make sure your terminal supports the key bindings:
- Try using arrow keys instead of `j`/`k`
- Ensure your terminal emulator is properly configured

### Application crashes on startup

1. Check that all dependencies are installed: `cargo check`
2. Verify Rust version: `rustc --version` (should be 1.70+)
3. Run with detailed errors: `RUST_BACKTRACE=1 cargo run`

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Copyright (c) cntrvsy <benintangana@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
