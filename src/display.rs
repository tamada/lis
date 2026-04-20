use crate::entry::Entry;
use lscolors::LsColors;
use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;
use humansize::{format_size, DECIMAL};

pub fn print_plain(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
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

pub fn print_long(entries: &[Entry], lscolors: &LsColors, show_icon: bool) {
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

pub fn get_icon(_name: &str, is_dir: bool, ext: &str) -> &'static str {
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
