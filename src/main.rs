use clap::{App, AppSettings, Arg};
use std::fs;
use std::os::unix::fs::PermissionsExt;

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

fn print_meta(file: &str) {
    let meta = fs::metadata(file).unwrap();
    println!("{:06o}\t0x{:x}\t{}", meta.permissions().mode(), meta.len(), file);
}
