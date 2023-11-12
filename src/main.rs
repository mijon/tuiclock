pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;
pub mod cli;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use crate::update::update;
use cli::Cli;
use clap::Parser;

// fn get_time2(format: &str) {
//     let date = Local::now();
//     date.format(format)
//     // println!("{}", date.format("%Y-%m-%d][%H:%M:%S"));
// }

// fn main() {
//     let curr_time = get_time2("%H:%M:%S");
//     println!("{}", curr_time);
// }




fn main() -> Result<()> {
    let args = Cli::parse();

    let mut app = App { should_quit: false,
                    time_format: args.format.to_string(), };

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(args.tick_rate);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => update(&mut app, key_event),
        }
    }

    tui.exit()?;
    Ok(())
}




    // let tmp = App { should_quit: false,
    //                 time_format: "%H:%M:%S".to_string(), };
    //
    // println!("{}", tmp.render_time());
