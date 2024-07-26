use clap::{Parser, ValueEnum};
use std::path::PathBuf;



#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Anki,
    Json,
    Csv,
    Txt
}

#[derive(Debug, Parser)]
#[clap(name = "duocards-downloader", about = "A downloader for DuoCards")]
pub struct Config {
    /// API token (copy from Authorization header, without 'Bearer' word)
    #[clap(long)]
    pub token: String,
    /// URL to download from
    #[clap(long, default_value = "https://api.duocards.com/graphql?CardsRefetchQuery")]
    pub url: String,
    /// Directory to save files
    #[clap(long, default_value=".", value_parser = parse_directory)]
    pub directory: PathBuf,
    /// Format to save
    #[clap(long, default_value = "txt")]
    pub format: OutputFormat,
    /// DeckID (copy from request, or use empty to use first deck)
    #[clap(long, default_value = "")]
    pub deckid: String
}


fn parse_directory(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("'{}' is not an existing directory", s))
    }
}

pub fn init() -> Config {
    let config = Config::parse();

    return config
}
