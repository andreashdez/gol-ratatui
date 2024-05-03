use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use crate::app::App;
use crate::tui;

pub fn key_listener(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('j') | KeyCode::Char('J') => app.increment_epoch(),
        _ => {}
    };
}

pub fn mouse_listener(app: &mut App, mouse_event: MouseEvent) {
    if let Some(coords) = tui::get_coords(mouse_event.column, mouse_event.row) {
        app.populate_board(coords.0, coords.1);
    }
}

pub fn resize_listener(app: &mut App, _c: u16, _r: u16) {
    let window_size = tui::get_size().unwrap_or((0, 0));
    app.resize_board(window_size.0, window_size.1);
}
