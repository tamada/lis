mod args;
mod display;
mod entry;
mod git;

use args::{Args, Format, SortBy};
use clap::Parser;
use entry::Entry;
use git::get_git_statuses;
use ignore::WalkBuilder;
use lscolors::LsColors;
use std::fs;

fn main() {
    let args = Args::parse();
    let lscolors = LsColors::from_env().unwrap_or_default();

    let mut entries = Vec::new();
    let git_statuses = get_git_statuses(&args.path);

    let mut builder = WalkBuilder::new(&args.path);
    builder.hidden(!args.all && !args.whole_all);
    builder.git_ignore(!args.whole_all);
    builder.max_depth(if args.recursive { None } else { Some(1) });

    let walker = builder.build();

    for result in walker {
        match result {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                // For name display:
                // If not recursive, use file_name.
                // If recursive, use path relative to args.path.
                let name = if args.recursive {
                    match path.strip_prefix(&args.path) {
                        Ok(p) => {
                            let s = p.to_string_lossy().into_owned();
                            if s.is_empty() {
                                args.path.to_string_lossy().into_owned()
                            } else {
                                s
                            }
                        }
                        Err(_) => dir_entry.file_name().to_string_lossy().into_owned(),
                    }
                } else {
                    if path == args.path {
                        continue;
                    }
                    dir_entry.file_name().to_string_lossy().into_owned()
                };

                let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
                let git_status = git_statuses
                    .get(&abs_path)
                    .cloned()
                    .unwrap_or_else(|| " ".to_string());

                if let Some(entry) = Entry::new(path, name, git_status) {
                    entries.push(entry);
                }
            }
            Err(_) => {}
        }
    }

    // Sorting
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

    match args.format {
        Format::Plain => {
            if args.long {
                display::print_long(&entries, &lscolors, args.icon);
            } else {
                display::print_plain(&entries, &lscolors, args.icon);
            }
        }
        Format::Csv => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            for entry in entries {
                wtr.serialize(entry).unwrap();
            }
            wtr.flush().unwrap();
        }
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        }
        Format::Yaml => {
            println!("{}", serde_yaml::to_string(&entries).unwrap());
        }
    }
}
