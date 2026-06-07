use humansize::{DECIMAL, format_size};
use lis::entry::Entry;
use lscolors::LsColors;
use terminal_size::{Width, terminal_size};
use unicode_width::UnicodeWidthStr;

pub fn print_plain(entries: &[Entry], lscolors: &LsColors, show_icon: bool, quote: bool) {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        print_one_column(entries, quote);
        return;
    }

    let names = style_entries(entries, lscolors, show_icon);
    if names.is_empty() {
        return;
    }

    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let max_width = get_max_width(entries, show_icon);

    print_columns(&names, max_width, term_width, quote);
}

fn escape_double_quote(name: &str) -> String {
    name.replace("\"", "\\\"")
}

fn print_one_column(entries: &[Entry], quote: bool) {
    for entry in entries {
        if quote {
            println!("\"{}\"", escape_double_quote(&entry.name));
        } else {
            println!("{}", entry.name);
        }
    }
}

fn style_entries(entries: &[Entry], lscolors: &LsColors, show_icon: bool) -> Vec<String> {
    entries
        .iter()
        .map(|e| {
            let s = if show_icon {
                format!("{} {}", get_icon(&e.name, e.is_dir, &e.extension), e.name)
            } else {
                e.name.clone()
            };
            lscolors
                .style_for_path(&e.path)
                .map(|style| style.to_nu_ansi_term_style().paint(&s).to_string())
                .unwrap_or(s)
        })
        .collect()
}

fn get_max_width(entries: &[Entry], show_icon: bool) -> usize {
    entries
        .iter()
        .map(|e| {
            let icon_width = if show_icon { 3 } else { 0 };
            UnicodeWidthStr::width(e.name.as_str()) + icon_width + 3
        })
        .max()
        .unwrap_or(0)
        + 2
}

fn print_columns(names: &[String], max_width: usize, term_width: usize, quote: bool) {
    let cols = (term_width / max_width).max(1);
    let rows = (names.len() as f64 / cols as f64).ceil() as usize;

    for r in 0..rows {
        for c in 0..cols {
            let i = c * rows + r;
            if i < names.len() {
                let pad = max_width.saturating_sub(visible_width(&names[i]));
                if quote {
                    print!("\"{}\"{}", escape_double_quote(&names[i]), " ".repeat(pad));
                } else {
                    print!("{}{}", names[i], " ".repeat(pad));
                }
            }
        }
        println!();
    }
}

fn visible_width(s: &str) -> usize {
    let mut plain = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for c in chars.by_ref() {
                if ('@'..='~').contains(&c) {
                    break;
                }
            }
            continue;
        }
        plain.push(ch);
    }

    UnicodeWidthStr::width(plain.as_str())
}

pub fn print_long(entries: &[Entry], lscolors: &LsColors, show_icon: bool, quote: bool) {
    for e in entries {
        let size_str = format_size(e.size, DECIMAL);
        let name_str = style_entry_name(e, lscolors, show_icon);
        let name_str = if quote {
            format!("\"{}\"", escape_double_quote(&name_str))
        } else {
            name_str
        };

        println!(
            "{} {:>3} {:<8} {:<8} {:>8} {} {} {}",
            e.mode, e.nlink, e.owner, e.group, size_str, e.modified, e.git_status, name_str
        );
    }
}

fn style_entry_name(e: &Entry, lscolors: &LsColors, show_icon: bool) -> String {
    let s = if show_icon {
        format!("{} {}", get_icon(&e.name, e.is_dir, &e.extension), e.name)
    } else {
        e.name.clone()
    };
    lscolors
        .style_for_path(&e.path)
        .map(|style| style.to_nu_ansi_term_style().paint(&s).to_string())
        .unwrap_or(s)
}

pub fn get_icon(_name: &str, is_dir: bool, ext: &str) -> &'static str {
    if is_dir {
        return "\u{f115}";
    }
    match ext {
        "rs" => "\u{e7a8}",
        "md" => "\u{f48a}",
        "toml" => "\u{f013}",
        "json" => "\u{f1c0}",
        "yaml" | "yml" => "\u{f1c0}",
        "png" | "jpg" | "jpeg" | "gif" => "\u{f1c5}",
        "txt" => "\u{f15c}",
        _ => "\u{f15b}",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lis::entry::Entry;
    use std::path::PathBuf;

    #[test]
    fn test_print_one_column() {
        // Since print_one_column uses println!, we can't easily capture it without redirecting stdout.
        // But we can at least call it to ensure it doesn't panic.
        let entries = vec![
            Entry {
                name: "file1".to_string(),
                path: PathBuf::from("file1"),
                is_dir: false,
                mode: "-".to_string(),
                nlink: 1,
                owner: "u".to_string(),
                group: "g".to_string(),
                size: 0,
                modified: "m".to_string(),
                git_status: " ".to_string(),
                extension: "".to_string(),
            }
        ];
        print_one_column(&entries, false);
    }

    #[test]
    fn test_print_long() {
        let lscolors = LsColors::from_env().unwrap_or_default();
        let entries = vec![
            Entry {
                name: "file1".to_string(),
                path: PathBuf::from("file1"),
                is_dir: false,
                mode: "-".to_string(),
                nlink: 1,
                owner: "u".to_string(),
                group: "g".to_string(),
                size: 100,
                modified: "2023-01-01 10:00".to_string(),
                git_status: " ".to_string(),
                extension: "".to_string(),
            }
        ];
        print_long(&entries, &lscolors, false, false);
    }

    #[test]
    fn test_style_entries() {
        let lscolors = LsColors::from_env().unwrap_or_default();
        let entries = vec![
            Entry {
                name: "file1".to_string(),
                path: PathBuf::from("file1"),
                is_dir: false,
                mode: "-".to_string(),
                nlink: 1,
                owner: "u".to_string(),
                group: "g".to_string(),
                size: 0,
                modified: "m".to_string(),
                git_status: " ".to_string(),
                extension: "".to_string(),
            }
        ];
        let styled = style_entries(&entries, &lscolors, false);
        assert_eq!(styled.len(), 1);
        assert!(styled[0].contains("file1"));
    }

    #[test]
    fn test_print_columns() {
        let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        print_columns(&names, 10, 80, false);
        print_columns(&names, 10, 20, false); // Test narrow terminal
    }

    #[test]
    fn test_visible_width() {
        let plain = "abc";
        let colored = "\u{1b}[31mabc\u{1b}[0m";
        assert_eq!(visible_width(plain), visible_width(colored));
    }

    #[test]
    fn test_get_icon() {
        assert_eq!(get_icon("main.rs", false, "rs"), "\u{e7a8}");
        assert_eq!(get_icon("README.md", false, "md"), "\u{f48a}");
        assert_eq!(get_icon("Cargo.toml", false, "toml"), "\u{f013}");
        assert_eq!(get_icon("src", true, ""), "\u{f115}");
        assert_eq!(get_icon("unknown", false, "unknown"), "\u{f15b}");
    }

    #[test]
    fn test_get_max_width() {
        let entries = vec![
            Entry {
                name: "file1.txt".to_string(),
                path: PathBuf::from("file1.txt"),
                is_dir: false,
                mode: "-rw-r--r--".to_string(),
                nlink: 1,
                owner: "user".to_string(),
                group: "group".to_string(),
                size: 100,
                modified: "2023-01-01 10:00".to_string(),
                git_status: " ".to_string(),
                extension: "txt".to_string(),
            },
            Entry {
                name: "long_filename_directory".to_string(),
                path: PathBuf::from("long_filename_directory"),
                is_dir: true,
                mode: "drwxr-xr-x".to_string(),
                nlink: 2,
                owner: "user".to_string(),
                group: "group".to_string(),
                size: 4096,
                modified: "2023-01-01 11:00".to_string(),
                git_status: " ".to_string(),
                extension: "".to_string(),
            },
        ];

        // Without icon: max("file1.txt".len(), "long_filename_directory".len()) + 2
        // "long_filename_directory".len() = 23. 23 + 2 = 25.
        assert_eq!(get_max_width(&entries, false), 25);

        // With icon: max("file1.txt".len() + 3, "long_filename_directory".len() + 3) + 2
        // 23 + 3 + 2 = 28.
        assert_eq!(get_max_width(&entries, true), 28);
    }

    #[test]
    fn test_style_entry_name() {
        let lscolors = LsColors::from_env().unwrap_or_default();
        let entry = Entry {
            name: "test.rs".to_string(),
            path: PathBuf::from("test.rs"),
            is_dir: false,
            mode: "-rw-r--r--".to_string(),
            nlink: 1,
            owner: "user".to_string(),
            group: "group".to_string(),
            size: 100,
            modified: "2023-01-01 10:00".to_string(),
            git_status: " ".to_string(),
            extension: "rs".to_string(),
        };

        let styled = style_entry_name(&entry, &lscolors, false);
        assert!(styled.contains("test.rs"));

        let styled_icon = style_entry_name(&entry, &lscolors, true);
        assert!(styled_icon.contains("\u{e7a8}"));
        assert!(styled_icon.contains("test.rs"));
    }
}
