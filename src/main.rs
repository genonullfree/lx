use clap::{App, AppSettings, Arg};
use std::fs;

fn main() {
    let matches = App::new("lx")
        .version("0.1.0")
        .author("geno")
        .about("List files with hex formatted output")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::from_usage("<cmd>... 'files to list'"),
        )
        .get_matches();

    let trail: Vec<&str> = matches.values_of("cmd").unwrap().collect();
    for i in trail {
        print_meta(i);
    }
}

fn print_meta(file Vec<&str>) {
    let meta = fs::metadata(file).unwrap();
    println!("0x{:x}\t{}", meta.len(), i);
}
