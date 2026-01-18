use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target URL to process
    #[arg(
        short = 'u',
        long,
        value_name = "URL",
        help = "The URL of album you want to scrape"
    )]
    url: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}
