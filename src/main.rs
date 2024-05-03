use std::env;

/// Application.
pub mod app;

/// Playing board.
pub mod board;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

use color_eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let window_size = tui::get_size()?;
    let mut app = app::App::new(window_size.0, window_size.1);

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = event::EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => update::key_listener(&mut app, key_event),
            event::Event::Mouse(mouse_event) => update::mouse_listener(&mut app, mouse_event),
            event::Event::Resize(c, r) => update::resize_listener(&mut app, c, r),
        };
    }

    tui.exit()?;
    Ok(())
}
