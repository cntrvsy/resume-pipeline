# Resume Pipeline

A Terminal User Interface (TUI) application built with Ratatui that generates custom **OPINIONATED** resumes from YAML data sources and exports them as PDFs using Typst.

## Why (personal reason)
- It was depressing spending 40 minutes on a resume and a cover letter to not even get a rejection email and creating AI slop resumes/cover letters in under 20 minutes wasn't getting me anywhere either. now i can spend 5 minutes on a resume and a cover letter and get a rejection email.
- i was inspired by this video [CV Pipeline as Code: LaTeX, YAML, and GitHub Actions](https://www.youtube.com/watch?v=S2gpOr-mbf4) and i wanted to give back to the community with my own spin. 

## Why (bigger picture)
- As someone who has studied Applied Computing Technology with a focus on Software Development, most hiring managers expect a more tailored and personalized resume to the **LISTED** job title or job description. so my varied **EXPERIENCE** wasnt hitting the mark as they are looking for someone they can onboard with little friction meaning i needed to settle on a **TECH STACK**. It's no longer just "front-end engineer" or "back-end engineer" but a "product implementation engineer" (yes, this was an actual job title and role involved basically creating UIs in Svelte and basic CRUD backend with type safety, yeah...). This ultimately meant that i needed multiple resumes for different job titles and different tech stacks.
- The job market is competitive, AI is great but that means it's harder to stand out and with most job recruiters i have followed on social media( the ones who arent trying to sell me something) have expressed their frustration with candidate fraud both at large and small scale.
- Also, doing my research each job title has different expectations some like when you include statistics others don't, all agree you shouldnt have SKILLS section and they want to see how you used those skills(keyword + How it was used + the Result of using it and/or the Reason it was used). agree, disagree, i dont care because if you havent noticed this all depends on the Recruiter "influencer" you follow, some resume styles are just difficult to make work and harder to filter your content requiring you to spend even more time on whatever [method](https://www.evidenced.app/blog/8-alternatives-to-star-method) they swear by. This application's goal is to allow software engineers to create custom resumes quickly and easily by just selecting which points are relevant to said job title. Data is stored in YAML files, and the final PDF is generated using Typst. all this following this guy - [The Headless HeadHunter](https://www.headlessheadhunter.org/how-to-get-a-job).


## Project Structure
```
src/
├── main.rs           # Entry point
├── app.rs            # App state & event handling
├── pdf.rs            # PDF generation
├── typst_backend.rs  # Typst integration
├── models/           # Data models
│   ├── mod.rs
│   ├── types.rs      # Struct definitions
│   └── resume.rs     # Loading logic
└── ui/               # Render logic
    ├── mod.rs
    ├── welcome.rs
    ├── profile.rs
    ├── education.rs
    ├── experience.rs
    ├── projects.rs
    ├── job_titles.rs
    └── status.rs
```

## Features

- **Straightforward Interface**: Literally just click enter.
- **Multi-section selection**: Choose from your profile, education, experience, and projects.
- **Live selection**: Toggle which items to include in your final resume using checkboxes.
- **PDF export**: Automatically generates a professional PDF using Typst templates
- **YAML-based data**: Easy to maintain and version control your resume data


## How do I use it?
- Grab the latest release from the [releases page](https://github.com/yourusername/resume-pipeline/releases).
- Be sure to download the resume-data.zip file from the same release.
- Unzip the resume-data.zip file and place it in the same directory as the resume-pipeline executable.
- open your terminal your terminal of choice,(if your using powershell its done differently etc, im using linux so mine looks like this)
```
./resume-pipeline-linux-x86_64
```
- majority of your time should be spend editing the provided YAML files in the resume-data directory, using a text editor of your choice use the format provided it breaks otherwise.
**TAKE NOTE OF THE INTERNAL STRUCTURE OF THE YAML FILES**

## Building from Source

### Prerequisites
- Rust toolchain (1.70 or later)
- Cargo package manager

### Commands

```bash
git clone <repository-url>
cd resume-pipeline
cargo build --release
```

The binary will be available at `target/release/resume-pipeline`

## Usage

### 1. Your Data

Inside the `data/` directory you'll find:

- `profile.yaml` - Your personal information
- `education.yaml` - Educational background
- `experience.yaml` - Work experience
- `projects.yaml` - Side projects and portfolio
- `jobtitles.yaml` - Job titles

- `output/resume.pdf` - Your generated resume
- `templates/headless_head_hunter.typ` - You are also free to copy paste this template into the [typst app](https://typst.app) it works by itself plus the online editor is really amazing.

**TAKE NOTE OF THE INTERNAL STRUCTURE OF THE YAML FILES**

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
├── templates/      # Typst templates
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
