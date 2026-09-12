use color_eyre::Result;
use std::fs;
use typst::foundations::{Dict, IntoValue};
use typst_pdf::PdfOptions;

use crate::models::ResumeData;
use crate::typst_backend::ResumeWorld;

// function to get the current year from the system to be used in the output file name
fn get_current_year() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let days = duration.as_secs() / 86400;

    let mut year = 1970;
    let mut days_left = days;

    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { 366 } else { 365 };

        if days_left >= days_in_year {
            days_left -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }

    year
}

use std::path::{Path, PathBuf};

// PDF GENERATION
pub fn generate_pdf(data: &ResumeData) -> Result<String> {
    generate_pdf_with_export(data, None)
}

pub fn generate_pdf_with_export(data: &ResumeData, export_target: Option<&str>) -> Result<String> {
    let current_dir = std::env::current_dir()?;

    let template_path = current_dir
        .join("data")
        .join("templates")
        .join("default_resume_template.typ");

    if !template_path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Template file not found at: {:?}.\nPlease ensure the 'data' folder containing your templates is in the same directory as the executable.",
            template_path
        ));
    }

    let template_content = fs::read_to_string(&template_path)?;

    // Convert Data
    let filtered_data = data.to_filtered_data();
    let inputs: Dict = filtered_data.into();

    // Create World
    let world = ResumeWorld::new(template_content, inputs);

    // Compile
    let document = typst::compile(&world)
        .output
        .map_err(|err| color_eyre::eyre::eyre!("Typst Compile Errors: {:?}", err))?;

    // It takes 2 arguments: the document and the options.
    let options = PdfOptions::default();

    // The ? operator unwraps the Ok(Vec<u8>) or returns the Err.
    let pdf_data = typst_pdf::pdf(&document, &options)
        .map_err(|e| color_eyre::eyre::eyre!("PDF Export Error: {:?}", e))?;

    // --- Generate dynamic filename ---
    // 1. First and Last Name
    let user_name = data
        .profile
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_default();

    let name_parts: Vec<&str> = user_name.split_whitespace().collect();
    let name_str = if !name_parts.is_empty() {
        let first_name = name_parts[0];
        let last_name = *name_parts.last().unwrap_or(&"");
        if first_name == last_name {
            first_name.to_string()
        } else {
            format!("{} {}", first_name, last_name)
        }
    } else {
        String::new()
    };

    // 2. Company Name (from target_company or cover_letter)
    let company_opt = data
        .target_company
        .as_ref()
        .or_else(|| data.cover_letter.as_ref().map(|cl| &cl.company))
        .map(|c| c.trim().replace('/', "-").replace('\\', "-"))
        .filter(|c| !c.is_empty());

    // 3. Job Title
    let title_str = if let Some(ref title) = data.job_title {
        let trimmed = title.trim();
        if !trimmed.is_empty() && trimmed != "N/A" {
            Some(trimmed.replace('/', "-").replace('\\', "-"))
        } else {
            None
        }
    } else {
        None
    };

    // 4. Current Year
    let current_year = get_current_year();

    let default_filename = if let Some(company) = company_opt {
        // When company is present: "<First Last> - <Company> - <Job Title> <Year>.pdf"
        let role = title_str.unwrap_or_else(|| "Resume".to_string());
        if !name_str.is_empty() {
            format!("{} - {} - {} {}.pdf", name_str, company, role, current_year)
        } else {
            format!("{} - {} {}.pdf", company, role, current_year)
        }
    } else {
        // Fallback if no company is specified: "<First Last> <Job Title> <Year>.pdf"
        let mut parts = Vec::new();
        if !name_str.is_empty() {
            parts.push(name_str);
        }
        if let Some(title) = title_str {
            parts.push(title);
        }
        parts.push(current_year.to_string());

        let mut base_filename = parts.join(" ");
        if base_filename.trim().is_empty() || parts.len() == 1 {
            base_filename = "resume".to_string();
        }
        let safe_filename = base_filename.replace('/', "-").replace('\\', "-");
        format!("{}.pdf", safe_filename)
    };

    let final_output_path: PathBuf = if let Some(target) = export_target {
        let path = Path::new(target);
        let is_pdf_file = target.to_lowercase().ends_with(".pdf");
        let is_directory = target.ends_with('/')
            || target.ends_with('\\')
            || path.is_dir()
            || (!is_pdf_file && path.extension().is_none());

        if is_directory {
            if !path.exists() {
                fs::create_dir_all(path)?;
            }
            path.join(default_filename)
        } else {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            path.to_path_buf()
        }
    } else {
        let output_dir = current_dir.join("data").join("output");
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }
        output_dir.join(default_filename)
    };

    fs::write(&final_output_path, pdf_data)?;

    Ok(final_output_path.to_string_lossy().to_string())
}

pub fn format_current_date() -> String {
    let now = time::OffsetDateTime::now_utc();
    let month_name = match now.month() {
        time::Month::January => "January",
        time::Month::February => "February",
        time::Month::March => "March",
        time::Month::April => "April",
        time::Month::May => "May",
        time::Month::June => "June",
        time::Month::July => "July",
        time::Month::August => "August",
        time::Month::September => "September",
        time::Month::October => "October",
        time::Month::November => "November",
        time::Month::December => "December",
    };
    format!("{} {}, {}", month_name, now.day(), now.year())
}

pub fn generate_cover_letter_pdf(
    data: &ResumeData,
    cover_letter: &crate::models::types::CoverLetterPreset,
    export_target: Option<&str>,
) -> Result<String> {
    use typst::foundations::IntoValue;

    let current_dir = std::env::current_dir()?;

    let template_path = current_dir
        .join("data")
        .join("templates")
        .join("default_cover_letter_template.typ");

    if !template_path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Cover letter template not found at: {:?}.\nPlease ensure 'default_cover_letter_template.typ' exists in data/templates/.",
            template_path
        ));
    }

    let template_content = fs::read_to_string(&template_path)?;

    let mut letter = cover_letter.clone();
    if letter.date.as_deref().unwrap_or("").is_empty() || letter.date.as_deref() == Some("auto") {
        letter.date = Some(format_current_date());
    }

    let mut inputs = Dict::new();
    let filtered_profile = data.to_filtered_data().profile;
    inputs.insert("profile".into(), filtered_profile.into_value());
    inputs.insert(
        "job_title".into(),
        data.job_title
            .clone()
            .unwrap_or_else(|| "Software Engineer".to_string())
            .into_value(),
    );
    inputs.insert("cover_letter".into(), letter.into_value());

    let world = ResumeWorld::new(template_content, inputs);

    let document = typst::compile(&world)
        .output
        .map_err(|err| color_eyre::eyre::eyre!("Typst Compile Errors in cover letter: {:?}", err))?;

    let pdf_data = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| color_eyre::eyre::eyre!("Cover Letter PDF Export Error: {:?}", e))?;

    let user_name = data
        .profile
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_default();

    let name_parts: Vec<&str> = user_name.split_whitespace().collect();
    let name_str = if !name_parts.is_empty() {
        let first_name = name_parts[0];
        let last_name = *name_parts.last().unwrap_or(&"");
        if first_name == last_name {
            first_name.to_string()
        } else {
            format!("{} {}", first_name, last_name)
        }
    } else {
        String::new()
    };

    let company_sanitized = cover_letter
        .company
        .trim()
        .replace('/', "-")
        .replace('\\', "-");

    let current_year = get_current_year();

    let default_filename = if !company_sanitized.is_empty() {
        if !name_str.is_empty() {
            format!("{} - {} - Cover Letter {}.pdf", name_str, company_sanitized, current_year)
        } else {
            format!("{} - Cover Letter {}.pdf", company_sanitized, current_year)
        }
    } else {
        if !name_str.is_empty() {
            format!("{} - Cover Letter {}.pdf", name_str, current_year)
        } else {
            format!("Cover Letter {}.pdf", current_year)
        }
    };

    let final_output_path: PathBuf = if let Some(target) = export_target {
        let path = Path::new(target);
        let is_pdf_file = target.to_lowercase().ends_with(".pdf");
        let is_directory = target.ends_with('/')
            || target.ends_with('\\')
            || path.is_dir()
            || (!is_pdf_file && path.extension().is_none());

        if is_directory {
            if !path.exists() {
                fs::create_dir_all(path)?;
            }
            path.join(default_filename)
        } else {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("cover_letter");
            let new_file_name = if stem.to_lowercase().contains("cover") {
                format!("{}.pdf", stem)
            } else {
                format!("{}_cover_letter.pdf", stem)
            };
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
                parent.join(new_file_name)
            } else {
                PathBuf::from(new_file_name)
            }
        }
    } else {
        let output_dir = current_dir.join("data").join("output");
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }
        output_dir.join(default_filename)
    };

    fs::write(&final_output_path, pdf_data)?;

    Ok(final_output_path.to_string_lossy().to_string())
}

pub fn inspect_layout(
    data: &ResumeData,
    cli_max_pages: Option<usize>,
) -> Result<crate::models::preset::LayoutTelemetry> {
    let current_dir = std::env::current_dir()?;
    let template_path = current_dir
        .join("data")
        .join("templates")
        .join("default_resume_template.typ");

    if !template_path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Template file not found at: {:?}.\nPlease ensure 'default_resume_template.typ' exists in data/templates/.",
            template_path
        ));
    }

    let template_content = fs::read_to_string(&template_path)?;
    let filtered_data = data.to_filtered_data();
    let inputs: Dict = filtered_data.into();
    let world = ResumeWorld::new(template_content, inputs);

    let document = typst::compile(&world)
        .output
        .map_err(|err| color_eyre::eyre::eyre!("Typst Compile Errors in resume: {:?}", err))?;

    let resume_pages = document.pages.len();

    let mut cover_letter_pages = None;
    if let Some(ref cl) = data.cover_letter {
        let cl_template_path = current_dir
            .join("data")
            .join("templates")
            .join("default_cover_letter_template.typ");

        if cl_template_path.exists() {
            let cl_template_content = fs::read_to_string(&cl_template_path)?;
            let mut letter = cl.clone();
            if letter.date.as_deref().unwrap_or("").is_empty() || letter.date.as_deref() == Some("auto") {
                letter.date = Some(format_current_date());
            }

            let mut cl_inputs = Dict::new();
            let filtered_profile = data.to_filtered_data().profile;
            cl_inputs.insert("profile".into(), filtered_profile.into_value());
            cl_inputs.insert(
                "job_title".into(),
                data.job_title
                    .clone()
                    .unwrap_or_else(|| "Software Engineer".to_string())
                    .into_value(),
            );
            cl_inputs.insert("cover_letter".into(), letter.into_value());

            let cl_world = ResumeWorld::new(cl_template_content, cl_inputs);
            if let Ok(cl_doc) = typst::compile(&cl_world).output {
                cover_letter_pages = Some(cl_doc.pages.len());
            }
        }
    }

    let max_resume_pages = cli_max_pages
        .or_else(|| data.layout.as_ref().and_then(|l| l.resolved_max_resume_pages()));
    let max_cover_letter_pages = data
        .layout
        .as_ref()
        .and_then(|l| l.resolved_max_cover_letter_pages());

    let resume_overflow = max_resume_pages.map_or(false, |limit| resume_pages > limit);
    let cover_letter_overflow = match (cover_letter_pages, max_cover_letter_pages) {
        (Some(actual), Some(limit)) => actual > limit,
        _ => false,
    };

    Ok(crate::models::preset::LayoutTelemetry {
        resume_pages,
        cover_letter_pages,
        max_resume_pages,
        max_cover_letter_pages,
        resume_overflow,
        cover_letter_overflow,
    })
}
