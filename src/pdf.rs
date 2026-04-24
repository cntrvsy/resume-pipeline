use color_eyre::Result;
use std::fs;
use typst::foundations::Dict;
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

// PDF GENERATION
pub fn generate_pdf(data: &ResumeData) -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let output_dir = current_dir.join("data").join("output");

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

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
    let mut filename_parts = Vec::new();

    // 1. First and Last Name
    let user_name = data
        .profile
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_default();

    let name_parts: Vec<&str> = user_name.split_whitespace().collect();
    if !name_parts.is_empty() {
        let first_name = name_parts[0];
        let last_name = *name_parts.last().unwrap_or(&"");
        if first_name == last_name {
            filename_parts.push(first_name.to_string());
        } else {
            filename_parts.push(format!("{} {}", first_name, last_name));
        }
    }

    // 2. Job Title
    if let Some(ref title) = data.job_title {
        let trimmed_title = title.trim();
        if !trimmed_title.is_empty() && trimmed_title != "N/A" {
            // Sanitize title to remove slashes which break file paths
            let sanitized_title = trimmed_title.replace('/', "-").replace('\\', "-");
            filename_parts.push(sanitized_title);
        }
    }

    // 3. Current Year
    let current_year = get_current_year();
    filename_parts.push(current_year.to_string());

    let mut base_filename = filename_parts.join(" ");

    // Fallback if empty or something went wrong (only contains year)
    if base_filename.trim().is_empty() || filename_parts.len() == 1 {
        base_filename = "resume".to_string();
    }

    // Final sanitization of the whole base filename just in case
    let safe_filename = base_filename.replace('/', "-").replace('\\', "-");
    let filename = format!("{}.pdf", safe_filename);
    let output_path = output_dir.join(&filename);
    fs::write(&output_path, pdf_data)?;

    Ok(output_path.to_string_lossy().to_string())
}
