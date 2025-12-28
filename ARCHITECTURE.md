# Architecture Documentation

## Overview

This document explains how the Resume Pipeline application works, from data loading to PDF generation.

## Application Flow

```
┌─────────────┐
│  Welcome    │
│  Screen     │
└──────┬──────┘
       │ Enter
       ▼
┌─────────────┐
│  Profile    │
│  View       │
└──────┬──────┘
       │ Enter
       ▼
┌─────────────┐
│  Education  │
│  Selection  │ ◄─┐
└──────┬──────┘   │
       │ Enter    │ Backspace
       ▼          │
┌─────────────┐   │
│ Experience  │   │
│  Selection  │ ──┤
└──────┬──────┘   │
       │ Enter    │ Backspace
       ▼          │
┌─────────────┐   │
│  Projects   │   │
│  Selection  │ ──┘
└──────┬──────┘
       │ Enter (Generate)
       ▼
┌─────────────┐
│ Generating  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Success/    │
│  Error      │
└─────────────┘
```

## Data Flow

### 1. Data Loading (`models.rs`)

```rust
ResumeData::load_from_dir()
  ↓
Read YAML files from data/
  ├── profile.yaml    → Profile struct
  ├── education.yaml  → Vec<Education>
  ├── experience.yaml → Vec<Experience>
  └── projects.yaml   → Vec<Project>
  ↓
Return ResumeData with all sections
```

Each item in the collections has an `is_visible` field (defaults to `true`) that tracks whether it should be included in the final resume.

### 2. User Interaction (`main.rs`)

The application uses multiple `ListState` objects to track user selection:
- `education_list_state`: Tracks current selection in education list
- `experience_list_state`: Tracks current selection in experience list
- `projects_list_state`: Tracks current selection in projects list

When the user presses `Space`, the `is_visible` flag is toggled for the currently selected item.

### 3. PDF Generation

When the user presses `Enter` on the Projects Selection screen, the following happens:

```rust
generate_pdf(&data)
  ↓
1. Create output/ directory
  ↓
2. Read Typst template from .github/templates/
   - Load as string (no file I/O needed)
  ↓
3. Build TypstEngine with template
   TypstEngine::builder().main_file(template_content).build()
  ↓
4. Filter data (only is_visible=true items)
   data.to_filtered_data()
   - Returns FilteredResumeData struct
   - Implements Into<Dict> for Typst
  ↓
5. Compile with data passed directly
   engine.compile_with_input(filtered_data)
   - No JSON file written to disk!
   - Data passed directly as Typst Dict
  ↓
6. Convert Typst document to PDF
   typst_pdf::pdf(&doc, &options)
  ↓
7. Write PDF to output/resume.pdf
```

## Key Components

### State Management (`CurrentScreen` enum)

```rust
enum CurrentScreen {
    Welcome,              // Initial welcome screen
    ProfileView,          // Display profile (no selection)
    EducationSelection,   // Select education items
    ExperienceSelection,  // Select experience items
    ProjectsSelection,    // Select projects
    Generating,           // Show loading state
    Success(String),      // Show success with file path
    Error(String),        // Show error message
    Exiting,             // Quit application
}
```

### Data Structures

All user data is stored in `ResumeData`:

```rust
pub struct ResumeData {
    pub profile: Option<Profile>,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
}
```

Each selectable item has an `is_visible` field:

```rust
#[serde(skip, default = "default_true")]
pub is_visible: bool,
```

The `#[serde(skip)]` ensures this UI-only field isn't serialized to JSON.

### JSON Generation

The `to_filtered_json()` method creates a clean JSON payload:

1. Filters each collection to only include `is_visible=true` items
2. Uses `serde_json` to serialize the filtered data
3. Returns a pretty-printed JSON string

This JSON matches the structure expected by the Typst template.

## Typst Template Integration

### How Data Gets to Typst (Direct Approach)

**Original Question:** The `.typ` file expects `json("/payload.json")`. How does this work?

**Answer:** We've simplified this! Instead of writing JSON to disk, we pass data directly to Typst:

```
┌─────────────────────────────────────────────────────────┐
│ Step 1: Filter the data                                │
│                                                         │
│ Rust: data.to_filtered_data()                          │
│   → Returns FilteredResumeData struct                  │
│   → Contains only is_visible=true items                │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ Step 2: Convert to Typst Dict                          │
│                                                         │
│ FilteredResumeData derives IntoDict and IntoValue      │
│   → Automatically implements Into<Dict>                │
│   → No JSON serialization needed!                      │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ Step 3: Pass directly to Typst engine                  │
│                                                         │
│ Rust: engine.compile_with_input(filtered_data)         │
│   → Data passed as Typst Dict in memory                │
│   → Typst receives it as sys.inputs                    │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ Step 4: Template accesses the data                     │
│                                                         │
│ Typst: #import sys: inputs                             │
│        #let resume_data = inputs                       │
│   → Direct access to the Dict we passed                │
│   → No file I/O needed!                                │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ Step 5: Template uses the data                         │
│                                                         │
│ Typst: resume_data.profile.name                        │
│        resume_data.experience                          │
│   → Accesses the data structure directly               │
└─────────────────────────────────────────────────────────┘
```

### Key Benefits

**No File I/O:** Data is passed directly in memory using Typst's `sys.inputs` mechanism.

**Type Safety:** The `derive_typst_intoval` crate automatically converts our Rust structs to Typst-compatible types.

**Cleaner Output:** The `output/` directory only contains `resume.pdf` - no intermediate JSON or template files.

### Template Structure

The template (`.github/templates/headless_head_hunter.typ`):

1. **Loads data from Rust:** `#import sys: inputs` and `#let resume_data = inputs`
   - Accesses the data passed via `compile_with_input()`
   - No file reading needed - data is already in memory
2. **Defines helper functions** for formatting sections
   - `resume_header()`, `section_title()`, `work_item()`, etc.
3. **Renders each section** using the loaded data
   - Iterates over arrays: `for job in resume_data.experience`
   - Accesses nested properties: `job.role`, `job.company`
4. **Uses Typst's layout and styling** features to create the PDF
   - Grid layouts, text styling, spacing, etc.

### Data Access in Template

The template accesses data like:
- `resume_data.profile.name`
- `resume_data.education` (array)
- `resume_data.experience` (array)
- `resume_data.projects` (array)

This matches the `FilteredResumeData` struct structure that we pass via `compile_with_input()`.

### The derive_typst_intoval Magic

The `derive_typst_intoval` crate provides derive macros that automatically implement the necessary traits:

```rust
#[derive(DeriveIntoValue, IntoDict)]
pub struct FilteredResumeData {
    profile: Profile,
    education: Vec<Education>,
    experience: Vec<Experience>,
    projects: Vec<Project>,
}
```

This generates code that converts our Rust structs into Typst's native data types (Dict, Array, String, etc.), making them directly usable in Typst templates without any manual serialization.

## Error Handling

Errors are handled at multiple levels:

1. **Data Loading**: Missing files are logged as warnings, not errors
2. **PDF Generation**: Returns `Result<String>` with detailed error messages
3. **UI**: Shows error screen with the error message
4. **Color-eyre**: Provides beautiful error reports with context

## Future Improvements

Potential enhancements:
- [ ] Multiple template support (let user choose template)
- [ ] Save selection presets (e.g., "Backend Role", "Frontend Role")
- [ ] Edit data within the TUI
- [ ] Preview PDF without exiting
- [ ] Export to multiple formats (HTML, Markdown)
- [ ] Skills and certifications sections
- [ ] Custom ordering of sections