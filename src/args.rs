use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Sort by
    #[arg(long, value_enum, default_value_t = SortBy::Name)]
    pub sort: SortBy,

    /// Reverse sort order
    #[arg(short, long)]
    pub reverse: bool,

    /// Long format
    #[arg(short, long)]
    pub long: bool,

    /// All entries, respecting .gitignore
    #[arg(short, long)]
    pub all: bool,

    /// All entries, ignoring .gitignore
    #[arg(short = 'A', long)]
    pub whole_all: bool,

    /// Display icons
    #[arg(long)]
    pub icon: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Plain)]
    pub format: Format,

    /// Recursive listing
    #[arg(short = 'R', long)]
    pub recursive: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Name,
    Time,
    Size,
    Extension,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Plain,
    Csv,
    Json,
    Yaml,
}
