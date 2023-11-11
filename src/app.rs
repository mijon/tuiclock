use chrono::Local;

/// Applictation Struct
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub time_format: String,
}


impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn render_time(&self) -> String {
        let time = Local::now();
        time.format(&self.time_format).to_string()
    }
}
