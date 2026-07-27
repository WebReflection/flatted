//! CLI for flattening / unflattening circular JSON (optional `cli` feature).

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process;

use flatted::{parse_simple, stringify_simple, Value};
use serde_json::Value as JsonValue;

fn usage(exe: &str) {
    eprintln!("Usage: {exe} [OPTION]... [FILE]");
    eprintln!("Flatten or unflatten circular JSON structures.");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -d, --decompress, --unflatten   unflatten flatted JSON to plain JSON");
    eprintln!();
    eprintln!("If no FILE is provided, or if FILE is -, read from standard input.");
}

fn read_input(path: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match path {
        None | Some("-") => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        Some(p) => Ok(fs::read_to_string(p)?),
    }
}

fn flatten(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: JsonValue = serde_json::from_str(input)?;
    let value = Value::from_json(&data);
    Ok(stringify_simple(&value)?)
}

fn unflatten(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = parse_simple(input.trim())?;
    // Lossy for cycles: emit structure with cycles broken to null for display.
    let json = parsed.to_serde_json();
    Ok(serde_json::to_string_pretty(&json)?)
}

fn main() {
    let exe = env::args().next().unwrap_or_else(|| "flatted".into());
    let args: Vec<String> = env::args().skip(1).collect();

    let mut decompress = false;
    let mut file: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                usage(&exe);
                process::exit(0);
            }
            "-d" | "--decompress" | "--unflatten" => decompress = true,
            "--" => {
                i += 1;
                if i < args.len() {
                    file = Some(args[i].as_str());
                }
                break;
            }
            s if s.starts_with('-') => {
                eprintln!("{exe}: unknown option {s}");
                usage(&exe);
                process::exit(1);
            }
            s => {
                file = Some(s);
                break;
            }
        }
        i += 1;
    }

    let input = match read_input(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{exe}: {e}");
            process::exit(1);
        }
    };

    let result = if decompress {
        unflatten(&input)
    } else {
        flatten(&input)
    };

    match result {
        Ok(out) => {
            let mut stdout = io::stdout().lock();
            let _ = writeln!(stdout, "{out}");
        }
        Err(e) => {
            eprintln!("{exe}: {e}");
            process::exit(1);
        }
    }
}
