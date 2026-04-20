mod args;
mod display;
mod entry;
mod git;

use args::{Args, Format, SortBy};
use clap::Parser;
use entry::Entry;
use git::get_git_statuses;
use ignore::{Walk, WalkBuilder};
use lscolors::LsColors;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args = Args::parse();
    let lscolors = LsColors::from_env().unwrap_or_default();
    let git_statuses = get_git_statuses(&args.path);

    let mut entries = collect_entries(&args, &git_statuses);
    sort_entries(&mut entries, &args);
    output_entries(&entries, &args, &lscolors);
}

fn collect_entries(args: &Args, git_statuses: &HashMap<PathBuf, String>) -> Vec<Entry> {
    let mut entries = Vec::new();
    let walker = create_walker(args);

    for result in walker {
        if let Ok(dir_entry) = result {
            if let Some(entry) = process_dir_entry(&dir_entry, args, git_statuses) {
                entries.push(entry);
            }
        }
    }
    entries
}

fn create_walker(args: &Args) -> Walk {
    let mut builder = WalkBuilder::new(&args.path);
    builder.hidden(!args.all && !args.whole_all);
    builder.git_ignore(!args.whole_all);
    builder.max_depth(if args.recursive { None } else { Some(1) });
    builder.build()
}

fn process_dir_entry(
    dir_entry: &ignore::DirEntry,
    args: &Args,
    git_statuses: &HashMap<PathBuf, String>,
) -> Option<Entry> {
    let path = dir_entry.path();
    let name = get_display_name(path, args, dir_entry)?;

    let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let git_status = git_statuses
        .get(&abs_path)
        .cloned()
        .unwrap_or_else(|| " ".to_string());

    Entry::new(path, name, git_status)
}

fn get_display_name(path: &Path, args: &Args, dir_entry: &ignore::DirEntry) -> Option<String> {
    if args.recursive {
        match path.strip_prefix(&args.path) {
            Ok(p) => {
                let s = p.to_string_lossy().into_owned();
                Some(if s.is_empty() {
                    args.path.to_string_lossy().into_owned()
                } else {
                    s
                })
            }
            Err(_) => Some(dir_entry.file_name().to_string_lossy().into_owned()),
        }
    } else {
        if path == args.path {
            return None;
        }
        Some(dir_entry.file_name().to_string_lossy().into_owned())
    }
}

fn sort_entries(entries: &mut [Entry], args: &Args) {
    entries.sort_by(|a, b| {
        let cmp = match args.sort {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Time => a.modified.cmp(&b.modified),
            SortBy::Size => a.size.cmp(&b.size),
            SortBy::Extension => a.extension.cmp(&b.extension),
        };
        if args.reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

fn output_entries(entries: &[Entry], args: &Args, lscolors: &LsColors) {
    match args.format {
        Format::Plain => {
            if args.long {
                display::print_long(entries, lscolors, args.icon);
            } else {
                display::print_plain(entries, lscolors, args.icon);
            }
        }
        Format::Csv => print_csv(entries),
        Format::Json => println!("{}", serde_json::to_string_pretty(entries).unwrap()),
        Format::Yaml => println!("{}", serde_yaml::to_string(entries).unwrap()),
    }
}

fn print_csv(entries: &[Entry]) {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for entry in entries {
        wtr.serialize(entry).unwrap();
    }
    wtr.flush().unwrap();
}
