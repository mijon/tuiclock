use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_big_text::BigText;

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        big_time(app.render_time()),
        f.size()
    )
}

pub fn big_time(time_string: String) -> BigText<'static> {
    let lines = vec![time_string.into()];
    tui_big_text::BigTextBuilder::default().lines(lines).build().unwrap()
}
