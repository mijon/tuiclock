use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // Time format
    #[arg(short, long, default_value = "%H:%M:%S")]
    pub format: String,

    #[arg(short, long, default_value_t = 250)]
    pub tick_rate: u64,
}
