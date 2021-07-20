use clap::{App, AppSettings, Arg};
use std::fs;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::str;

fn main() {
    let matches = App::new("lx")
        .version("0.1.0")
        .author("geno")
        .about("List files with hex formatted output")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::from_usage("[file]... 'files to list'"))
        .get_matches();

    // Parse the command arguments, or default to current directory
    let trail: Vec<&str> = match matches.values_of("file") {
        Some(_) => matches.values_of("file").unwrap().collect(),
        None => vec!["."],
    };

    // Iterate through every file listed as an argument
    for i in trail {
        print_meta(i);
    }
}

fn print_meta(file: &str) {
    let ftype = get_file_type(&file);
    let meta = match fs::metadata(file) {
        Ok(x) => x,
        Err(_) => {
            println!("???\t\t{}", file);
            return;
        }
    };
    let perms: &str = &get_permissions(meta.permissions().mode())
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

// Identify the type of file
fn get_file_type(file: &str) -> char {
    let meta = match fs::metadata(file) {
        Ok(x) => x,
        Err(_) => return '?',
    };
    let ftype: char;
    if meta.file_type().is_symlink() {
        ftype = 's';
    } else if meta.file_type().is_dir() {
        ftype = 'd';
    } else if meta.file_type().is_block_device() {
        ftype = 'b';
    } else if meta.file_type().is_char_device() {
        ftype = 'c';
    } else if meta.file_type().is_fifo() {
        ftype = 'f';
    } else if meta.file_type().is_socket() {
        ftype = 'n';
    } else {
        ftype = '-';
    }
    ftype
}

// Translate the permission values into ascii characters
fn get_permissions(mode: u32) -> Vec<char> {
    let mut d: Vec<char> = Vec::<char>::new();
    for i in 0..3 {
        let tmp = (mode >> (2 - i) * 3) & 0x7;
        if (tmp & 4) == 4 {
            d.push('r');
        } else {
            d.push('-')
        }
        if (tmp & 2) == 2 {
            d.push('w');
        } else {
            d.push('-')
        }
        if (tmp & 1) == 1 {
            d.push('x');
        } else {
            d.push('-')
        }
    }
    d
}
