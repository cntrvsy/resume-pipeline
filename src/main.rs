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

    let is_list = cli.list_items
        || cli.command.as_deref() == Some("list")
        || cli.command.as_deref() == Some("dump-data");

    // 1. Schema Dump Mode
    if cli.dump_schema {
        if cli.json {
            println!(
                "{}",
                serde_json::json!({
                    "status": "success",
                    "schema": dump_preset_schema()
                })
            );
        } else {
            println!("{}", dump_preset_schema());
        }
        return Ok(());
    }

    // 2. Data Item Listing Mode (--list-items / list / dump-data)
    if is_list {
        let app = App::new();
        if cli.json {
            println!("{}", serde_json::to_string_pretty(&app.data.list_items_json())?);
        } else {
            println!("{}", app.data.list_items_text());
        }
        return Ok(());
    }

    // 3. Load Master Data & Apply Presets
    let mut app = App::new();
    let mut report_opt = None;

    if let Some(ref preset_path) = cli.preset {
        let preset_str = fs::read_to_string(preset_path).map_err(|e| {
            color_eyre::eyre::eyre!("Could not read preset file at {:?}: {}", preset_path, e)
        })?;
        let preset: SelectionPreset = serde_yaml::from_str(&preset_str).map_err(|e| {
            color_eyre::eyre::eyre!("YAML Parsing error in preset {:?}: {}", preset_path, e)
        })?;

        let report = app.data.apply_preset(&preset);
        report_opt = Some(report);
    }

    if let Some(ref job_title) = cli.job {
        app.data.job_title = Some(job_title.clone());
    }

    // 4. Dry-Run / Validation Flag (--validate / --check)
    if cli.validate {
        let report = report_opt.unwrap_or_default();
        let status = if report.has_unmatched() {
            "failed"
        } else {
            "success"
        };

        if cli.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&report.to_json_value(status, None))?
            );
        } else {
            report.print_summary();
        }

        if report.has_unmatched() {
            std::process::exit(1);
        }
        return Ok(());
    }

    // 5. Dump TUI Screen Text Mode
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

    // 6. Non-Interactive Export Mode
    if let Some(ref export_path_arg) = cli.export {
        let pdf_path = pdf::generate_pdf_with_export(&app.data, Some(export_path_arg))?;
        let report = report_opt.unwrap_or_default();
        let status = if report.has_unmatched() {
            "warning"
        } else {
            "success"
        };

        if cli.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&report.to_json_value(status, Some(&pdf_path)))?
            );
        } else {
            if cli.preset.is_some() {
                report.print_summary();
            }
            println!("Successfully generated resume PDF at: {}", pdf_path);
        }
        return Ok(());
    }

    // If preset was applied in TUI mode
    if !cli.json {
        if let Some(ref report) = report_opt {
            report.print_summary();
        }
    }

    // 7. Interactive TUI Mode
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
