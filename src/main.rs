use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

mod app;
mod models;
mod pdf;
mod typst_backend;
mod ui;

use app::{App, CurrentScreen};
use ui::render_ui;

// 3. ENTRY POINT
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

// 4. APPLICATION LOOP
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    while app.current_screen != CurrentScreen::Exiting {
        terminal.draw(|frame| render_ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key_event(key.code);
            }
        }
    }
    Ok(())
}
