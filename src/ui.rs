use ratatui::{
    prelude::{Constraint, Frame, Layout, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_big_text::BigText;

use crate::app::App;


pub fn render(app: &mut App, f: &mut Frame) {
    let vert_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(f.size());

    let horiz_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(vert_layout[1]);

    f.render_widget(
        big_time(app.render_time()),
        horiz_layout[1]
    )
}

pub fn big_time(time_string: String) -> BigText<'static> {
    let lines = vec![time_string.into()];
    tui_big_text::BigTextBuilder::default().lines(lines).build().unwrap()
}
