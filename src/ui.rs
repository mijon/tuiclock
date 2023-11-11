use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(
            format!("{}", app.render_time())
    )
    .block(
        Block::default()
            .title("Time")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Center),
        f.size()
    )
}

