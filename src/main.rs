use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::TestBackend;
use ratatui::{DefaultTerminal, Terminal};
use std::fs;

use resume_pipeline::app::{App, CurrentScreen};
use resume_pipeline::cli::{dump_preset_schema, Cli};
use resume_pipeline::models::SelectionPreset;
use resume_pipeline::pdf;
use resume_pipeline::ui::render_ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // 1. Schema Dump Mode
    if cli.dump_schema {
        println!("{}", dump_preset_schema());
        return Ok(());
    }

    // 2. Load Master Data & Apply Presets
    let mut app = App::new();

    if let Some(ref preset_path) = cli.preset {
        let preset_str = fs::read_to_string(preset_path).map_err(|e| {
            color_eyre::eyre::eyre!("Could not read preset file at {:?}: {}", preset_path, e)
        })?;
        let preset: SelectionPreset = serde_yaml::from_str(&preset_str).map_err(|e| {
            color_eyre::eyre::eyre!("YAML Parsing error in preset {:?}: {}", preset_path, e)
        })?;

        let report = app.data.apply_preset(&preset);
        report.print_summary();
    }

    if let Some(ref job_title) = cli.job {
        app.data.job_title = Some(job_title.clone());
    }

    // 3. Dump TUI Screen Text Mode
    if cli.dump_screen {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|frame| render_ui(frame, &mut app))?;
        let buffer = terminal.backend().buffer();
        for y in 0..buffer.area.height {
            let mut line = String::new();
            for x in 0..buffer.area.width {
                if let Some(cell) = buffer.cell((x, y)) {
                    line.push_str(cell.symbol());
                }
            }
            println!("{}", line.trim_end());
        }
        return Ok(());
    }

    // 4. Non-Interactive Export Mode
    if cli.export.is_some() {
        let pdf_path = pdf::generate_pdf(&app.data)?;
        println!("Successfully generated resume PDF at: {}", pdf_path);
        return Ok(());
    }

    // 5. Interactive TUI Mode
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal, &mut app);
    ratatui::restore();
    app_result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    while app.current_screen != CurrentScreen::Exiting {
        terminal.draw(|frame| render_ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key_event(key.code);
            }
        }
    }
    Ok(())
}
