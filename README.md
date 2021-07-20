# lx

lx is a utility to list contents of a directory or a list of files with output similar to `ls` but with the file size value printed in hexadecimal.

## Usage
```
lx 0.1.0
geno
List files with hex formatted output

USAGE:
    lx [file]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>...    files to list
```

## Sample output
```
$ cargo run -- *
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/lx Cargo.lock Cargo.toml README.md src target`
-rw-r--r--      100644  0xbd9   Cargo.lock
-rw-r--r--      100644  0x106   Cargo.toml
-rw-r--r--      100644  0x19a   README.md
drwxr-xr-x      040755  0xe     src
drwxr-xr-x      040755  0x50    target
```

```
Each line of output is:
`[permissions] [permissions in octal] [file size in hex] [file name]`
