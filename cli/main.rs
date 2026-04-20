mod args;
mod display;

use args::{Args, Format};
use clap::Parser;
use lis::entry::{sort_entries, Entry};
use lis::Lis;
use lscolors::LsColors;

fn main() {
    let args = Args::parse();
    let lscolors = LsColors::from_env().unwrap_or_default();

    let mut entries = Lis::new(args.path.clone())
        .recursive(args.recursive)
        .all(args.all)
        .whole_all(args.whole_all)
        .list();

    sort_entries(&mut entries, args.sort.clone(), args.reverse);
    output_entries(&entries, &args, &lscolors);
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
