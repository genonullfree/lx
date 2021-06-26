use clap::{App, AppSettings, Arg};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::str;

fn main() {
    let matches = App::new("lx")
        .version("0.1.0")
        .author("geno")
        .about("List files with hex formatted output")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from_usage("<cmd>... 'files to list'"))
        .get_matches();

    let trail: Vec<&str> = matches.values_of("cmd").unwrap().collect();
    for i in trail {
        print_meta(i);
    }
}

fn print_meta(file: &str) {
    let meta = fs::metadata(file).unwrap();
    let ftype: char;
    if meta.file_type().is_symlink() {
        ftype = 's';
    } else if meta.file_type().is_dir() {
        ftype = 'd';
    } else if (meta.permissions().mode() & 0o060000) == 0o060000 {
        ftype = 'b';
    } else if (meta.permissions().mode() & 0o020000) == 0o020000 {
        ftype = 'c';
    } else {
        ftype = '-';
    };
    let perms: &str = &print_permissions(meta.permissions().mode())
        .iter()
        .collect::<String>();
    println!(
        "{}{}\t{:06o}\t0x{:x}\t{}",
        ftype,
        perms,
        meta.permissions().mode(),
        meta.len(),
        file
    );
}

fn print_permissions(mode: u32) -> Vec<char> {
    let mut d: Vec<char> = Vec::<char>::new();
    for i in 0..3 {
        let tmp = (mode >> (2 - i) * 3) & 0x7;
        if (tmp & 4) == 4 {
            d.push('r');
        } else {
            d.push('-')
        };
        if (tmp & 2) == 2 {
            d.push('w');
        } else {
            d.push('-')
        };
        if (tmp & 1) == 1 {
            d.push('x');
        } else {
            d.push('-')
        };
    }
    d
}
