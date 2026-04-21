use humansize::{DECIMAL, format_size};
use lis::entry::Entry;
use lscolors::LsColors;
use terminal_size::{Width, terminal_size};
use unicode_width::UnicodeWidthStr;

pub fn print_plain(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        print_one_column(entries);
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

    print_columns(&names, max_width, term_width);
}

fn print_one_column(entries: &[Entry]) {
    for entry in entries {
        println!("{}", entry.name);
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
            UnicodeWidthStr::width(e.name.as_str()) + icon_width
        })
        .max()
        .unwrap_or(0)
        + 2
}

fn print_columns(names: &[String], max_width: usize, term_width: usize) {
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

pub fn print_long(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
    for e in entries {
        let size_str = format_size(e.size, DECIMAL);
        let name_str = style_entry_name(e, lscolors, show_icon);

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
