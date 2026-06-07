use clap::{Parser, ValueEnum};
use lis::entry::SortBy;
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

    /// Quote the entries.
    #[arg(long)]
    pub quote: bool,

    /// Display icons
    #[arg(long)]
    pub icon: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Plain)]
    pub format: Format,

    /// Recursive listing
    #[arg(short = 'R', long)]
    pub recursive: bool,

    #[cfg(debug_assertions)]
    /// Generate the completion script files for several shells.
    #[arg(long)]
    pub completions: bool
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Plain,
    Csv,
    Json,
    Yaml,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default() {
        let args = Args::try_parse_from(["lis"]).unwrap();
        assert_eq!(args.path, PathBuf::from("."));
        assert_eq!(args.sort, SortBy::Name);
        assert!(!args.reverse);
        assert!(!args.long);
        assert!(!args.all);
        assert!(!args.whole_all);
        assert_eq!(args.format, Format::Plain);
    }

    #[test]
    fn test_args_custom() {
        let args = Args::try_parse_from([
            "lis", "src", "--sort", "size", "-r", "-l", "-a", "--icon", "--format", "json", "-R",
        ])
        .unwrap();
        assert_eq!(args.path, PathBuf::from("src"));
        assert_eq!(args.sort, SortBy::Size);
        assert!(args.reverse);
        assert!(args.long);
        assert!(args.all);
        assert!(args.icon);
        assert_eq!(args.format, Format::Json);
        assert!(args.recursive);
    }
}

