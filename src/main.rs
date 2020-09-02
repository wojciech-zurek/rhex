use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::error::Error;
use std::{result, process};
use colored::Colorize;
use std::path::Path;
use std::io::{BufReader, Read, Write, ErrorKind};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const DEFAULT_BYTES: &str = "20";

type Result = result::Result<(), Box<dyn Error>>;

fn main() -> Result {
    parse(&cli())
}

fn cli() -> ArgMatches {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about("Another command-line hex viewer")
        .arg(Arg::with_name("file")
            .about("File path")
            .required(false)
            .index(1)
            .takes_value(true))
        .arg(Arg::with_name("bytes")
            .about("Bytes per line")
            .short('b')
            .long("bytes")
            .default_value(DEFAULT_BYTES)
            .required(false)
            .takes_value(true))
        .get_matches()
}

fn parse(matches: &ArgMatches) -> Result {
    let bytes = matches.value_of("bytes").unwrap_or(DEFAULT_BYTES).parse::<usize>().unwrap();
    match matches.value_of("file") {
        Some(file_name) => {
            let path = Path::new(file_name);
            if !path.exists() || path.is_dir() {
                Err(Box::from("Invalid file path"))
            } else {
                read(BufReader::new(File::open(&file_name)?), bytes)
            }
        }
        None => read(BufReader::new(std::io::stdin()), bytes)
    }
}

fn read<T: Read>(mut reader: BufReader<T>, bytes: usize) -> Result {
    let mut buffer = Vec::new();
    buffer.resize(bytes, 0);

    let mut index = 0;
    let mut stdout = std::io::stdout();

    while let Ok(r) = reader.read(&mut buffer) {
        if r == 0 { break; }

        let idx = format!("{:0>8X} ", index).green().bold();

        let hex = buffer
            .iter()
            .map(|c| format!("{:<3}", format!("{:0>2X}", c)))
            .collect::<String>();

        let data = buffer
            .iter()
            .map(|c| String::from_utf8(escape(*c)).unwrap())
            .collect::<String>();

        if let Err(e) = writeln!(stdout, "{}", format!("{} {} {}", idx, hex, data)) {
            if e.kind() != ErrorKind::BrokenPipe {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        buffer.clear();
        buffer.resize(bytes, 0);
        index += r;
    }

    Ok(())
}

fn escape(c: u8) -> Vec<u8> {
    match c {
        0 => vec![],
        b'\t' => vec![b'\\', b't'],
        b'\r' => vec![b'\\', b'r'],
        b'\n' => vec![b'\\', b'n'],
        b'\\' => vec![b'\\', b'\\'],
        b'\'' => vec![b'\\', b'\''],
        // b'"' => vec![b'\\', b'"'],
        b'\x20'..=b'\x7e' => vec![c],
        _ => vec![b'.']
    }
}
