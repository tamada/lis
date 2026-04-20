use chrono::{DateTime, Local};
use clap::{Parser, ValueEnum};
use git2::{Repository, StatusOptions};
use humansize::{format_size, DECIMAL};
use ignore::WalkBuilder;
use lscolors::LsColors;
use nix::sys::stat::Mode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;
use uzers::{get_group_by_gid, get_user_by_uid};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Sort by
    #[arg(long, value_enum, default_value_t = SortBy::Name)]
    sort: SortBy,

    /// Reverse sort order
    #[arg(short, long)]
    reverse: bool,

    /// Long format
    #[arg(short, long)]
    long: bool,

    /// All entries, respecting .gitignore
    #[arg(short, long)]
    all: bool,

    /// All entries, ignoring .gitignore
    #[arg(short = 'A', long)]
    whole_all: bool,

    /// Display icons
    #[arg(long)]
    icon: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Plain)]
    format: Format,

    /// Recursive listing
    #[arg(short = 'R', long)]
    recursive: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SortBy {
    Name,
    Time,
    Size,
    Extension,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum Format {
    Plain,
    Csv,
    Json,
    Yaml,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    mode: String,
    nlink: u64,
    owner: String,
    group: String,
    size: u64,
    modified: String,
    git_status: String,
    extension: String,
}

fn get_git_statuses(path: &Path) -> HashMap<PathBuf, String> {
    let mut statuses = HashMap::new();
    if let Ok(repo) = Repository::discover(path) {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        if let Ok(repo_statuses) = repo.statuses(Some(&mut opts)) {
            if let Some(workdir) = repo.workdir() {
                let workdir = fs::canonicalize(workdir).unwrap_or_else(|_| workdir.to_path_buf());
                for entry in repo_statuses.iter() {
                    if let Some(path_str) = entry.path() {
                        let full_path = workdir.join(path_str);
                        let status = entry.status();
                        let status_str = if status.is_index_new() || status.is_wt_new() {
                            "A"
                        } else if status.is_index_modified() || status.is_wt_modified() {
                            "M"
                        } else if status.is_index_deleted() || status.is_wt_deleted() {
                            "D"
                        } else if status.is_index_renamed() || status.is_wt_renamed() {
                            "R"
                        } else if status.is_index_typechange() || status.is_wt_typechange() {
                            "T"
                        } else {
                            " "
                        };
                        statuses.insert(full_path, status_str.to_string());
                    }
                }
            }
        }
    }
    statuses
}

fn get_mode_string(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut s = String::with_capacity(10);

    if is_dir {
        s.push('d');
    } else if is_symlink {
        s.push('l');
    } else {
        s.push('-');
    }

    #[cfg(unix)]
    {
        let m = Mode::from_bits_truncate(mode as nix::sys::stat::mode_t);
        s.push(if m.contains(Mode::S_IRUSR) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWUSR) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXUSR) { 'x' } else { '-' });
        s.push(if m.contains(Mode::S_IRGRP) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWGRP) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXGRP) { 'x' } else { '-' });
        s.push(if m.contains(Mode::S_IROTH) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWOTH) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXOTH) { 'x' } else { '-' });
    }

    s
}

fn get_icon(_name: &str, is_dir: bool, ext: &str) -> &'static str {
    if is_dir {
        return "\u{f115}"; // 📁
    }
    match ext {
        "rs" => "\u{e7a8}",   // 🦀
        "md" => "\u{f48a}",   // 📝
        "toml" => "\u{f013}", // ⚙️
        "json" => "\u{f1c0}", // 🗄️
        "yaml" | "yml" => "\u{f1c0}",
        "png" | "jpg" | "jpeg" | "gif" => "\u{f1c5}", // 🖼️
        "txt" => "\u{f15c}",                          // 📄
        _ => "\u{f15b}",                              // 📄
    }
}

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

                let metadata = match fs::symlink_metadata(path) {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let is_dir = metadata.is_dir();
                let is_symlink = metadata.file_type().is_symlink();
                let mode = get_mode_string(metadata.permissions().mode(), is_dir, is_symlink);
                let nlink = metadata.nlink();
                let owner = get_user_by_uid(metadata.uid())
                    .map(|u| u.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| metadata.uid().to_string());
                let group = get_group_by_gid(metadata.gid())
                    .map(|g| g.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| metadata.gid().to_string());
                let size = metadata.len();
                let modified: DateTime<Local> = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()).into();
                let modified_str = modified.format("%Y-%m-%d %H:%M").to_string();
                
                let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
                let git_status = git_statuses.get(&abs_path).cloned().unwrap_or_else(|| " ".to_string());
                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();

                entries.push(Entry {
                    name,
                    path: path.to_path_buf(),
                    is_dir,
                    mode,
                    nlink,
                    owner,
                    group,
                    size,
                    modified: modified_str,
                    git_status,
                    extension,
                });
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
                print_long(&entries, &lscolors, args.icon);
            } else {
                print_plain(&entries, &lscolors, args.icon);
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

fn print_plain(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        for entry in entries {
            println!("{}", entry.name);
        }
        return;
    }

    let names: Vec<String> = entries
        .iter()
        .map(|e| {
            let s = if show_icon {
                format!("{} {}", get_icon(&e.name, e.is_dir, &e.extension), e.name)
            } else {
                e.name.clone()
            };
            if let Some(style) = lscolors.style_for_path(&e.path) {
                let colored_s = style.to_nu_ansi_term_style().paint(s);
                format!("{}", colored_s)
            } else {
                s
            }
        })
        .collect();

    if names.is_empty() {
        return;
    }

    let term_width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let max_width = entries
        .iter()
        .map(|e| {
            let icon_width = if show_icon { 3 } else { 0 };
            UnicodeWidthStr::width(e.name.as_str()) + icon_width
        })
        .max()
        .unwrap_or(0)
        + 2;

    let cols = (term_width / max_width).max(1);
    let rows = (names.len() as f64 / cols as f64).ceil() as usize;

    for r in 0..rows {
        for c in 0..cols {
            let i = c * rows + r;
            if i < names.len() {
                print!("{:<width$}", names[i], width = max_width);
            }
        }
        println!();
    }
}

fn print_long(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
    for e in entries {
        let size_str = format_size(e.size, DECIMAL);
        let mut name_str = if show_icon {
            format!("{} {}", get_icon(&e.name, e.is_dir, &e.extension), e.name)
        } else {
            e.name.clone()
        };

        if let Some(style) = lscolors.style_for_path(&e.path) {
            name_str = format!("{}", style.to_nu_ansi_term_style().paint(name_str));
        }

        println!(
            "{} {:>3} {:<8} {:<8} {:>8} {} {} {}",
            e.mode, e.nlink, e.owner, e.group, size_str, e.modified, e.git_status, name_str
        );
    }
}
