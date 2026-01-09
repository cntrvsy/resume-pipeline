use color_eyre::Result;
use std::fs;
use std::path::Path;
use typst::foundations::Dict;
use typst_pdf::PdfOptions;

use crate::models::ResumeData;
use crate::typst_backend::ResumeWorld;

// 2. PDF GENERATION
pub fn generate_pdf(data: &ResumeData) -> Result<String> {
    let output_dir = Path::new("data/output");
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    // Load template
    // Note: Assuming you are using include_str! for the embedded solution
    // If you are reading from disk for dev, ensure the path is correct
    let template_path = Path::new("data/templates/headless_head_hunter.typ");
    let template_content = fs::read_to_string(template_path).expect("Could not read template file");

    // Convert Data
    let filtered_data = data.to_filtered_data();
    let inputs: Dict = filtered_data.into();

    // Create World
    let world = ResumeWorld::new(template_content, inputs);

    // Compile
    let document = typst::compile(&world)
        .output
        .map_err(|err| color_eyre::eyre::eyre!("Typst Compile Errors: {:?}", err))?;

    // FIX: Updated to match typst-pdf 0.12 signature
    // It takes 2 arguments: the document and the options.
    let options = PdfOptions::default();

    // FIX: Handle the Result returned by pdf().
    // The ? operator unwraps the Ok(Vec<u8>) or returns the Err.
    let pdf_data = typst_pdf::pdf(&document, &options)
        .map_err(|e| color_eyre::eyre::eyre!("PDF Export Error: {:?}", e))?;

    let output_path = output_dir.join("resume.pdf");
    fs::write(&output_path, pdf_data)?;

    Ok(output_path.to_string_lossy().to_string())
}
