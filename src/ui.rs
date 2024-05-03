use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

// pub fn render(app: &mut App, f: &mut Frame) {
//     f.render_widget(
//         Block::default()
//             .title(format!(" Game of Life (Epoch: {}) ", app.epoch))
//             .title_alignment(Alignment::Center)
//             .borders(Borders::ALL)
//             .border_type(BorderType::Thick)
//             .style(Style::default().fg(Color::Yellow)),
//         f.size(),
//     )
// }

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(app.board.to_string())
            .block(
                Block::default()
                    .title(format!("Game of Life (Epoch: {})", app.epoch))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        f.size(),
    )
}
