//! Flatted bench mirroring `test/bench.js` timing (1s wall-clock ops/sec).
//!
//! ```bash
//! cargo run --example bench --release
//! cargo run --example bench --release -- --json
//! ```

use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use flatted::{parse_simple, stringify_simple, Value};
use serde_json::Value as JsonValue;

const WINDOW: Duration = Duration::from_secs(1);

fn repo_test_dir() -> PathBuf {
    // examples/ -> rust/ -> repo root -> test/
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("test")
}

fn bench_stringify(value: &Value) -> f64 {
    let start = Instant::now();
    let mut i = 0u64;
    let mut last = String::new();
    while start.elapsed() < WINDOW {
        last = stringify_simple(value).expect("stringify");
        i += 1;
    }
    // keep last result live so the optimizer cannot drop the work
    std::hint::black_box(&last);
    i as f64 / WINDOW.as_secs_f64()
}

fn bench_parse(text: &str) -> f64 {
    let start = Instant::now();
    let mut i = 0u64;
    let mut last = Value::Null;
    while start.elapsed() < WINDOW {
        last = parse_simple(text).expect("parse");
        i += 1;
    }
    std::hint::black_box(&last);
    i as f64 / WINDOW.as_secs_f64()
}

fn load_data_json() -> Value {
    let path = repo_test_dir().join("data.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let json: JsonValue = serde_json::from_str(&raw).expect("data.json");
    Value::from_json(&json)
}

fn slice_array(value: &Value, len: usize) -> Value {
    let items: Vec<Value> = value
        .as_array()
        .expect("array")
        .borrow()
        .iter()
        .take(len)
        .cloned()
        .collect();
    Value::Array(Rc::new(RefCell::new(items)))
}

fn concat_shared(parts: &[&Value]) -> Value {
    let mut out = Vec::new();
    for part in parts {
        out.extend(part.as_array().expect("array").borrow().iter().cloned());
    }
    Value::Array(Rc::new(RefCell::new(out)))
}

fn make_circular_object() -> Value {
    let o = Value::empty_object();
    o.as_object()
        .unwrap()
        .borrow_mut()
        .insert("b".into(), o.clone());
    o
}

fn circular_list(n: usize) -> Value {
    let items: Vec<Value> = (0..n).map(|_| make_circular_object()).collect();
    Value::Array(Rc::new(RefCell::new(items)))
}

struct ResultRow {
    case: &'static str,
    op: &'static str,
    label: String,
    ops: f64,
}

fn emit(row: &ResultRow, json: bool) {
    if json {
        println!(
            r#"{{"engine":"rust","case":"{}","op":"{}","label":"{}","ops":{:.2}}}"#,
            row.case,
            row.op,
            row.label.replace('"', "\\\""),
            row.ops
        );
    } else {
        println!(
            "rust / flatted {} {} parsed {:.2} times per second",
            row.op, row.label, row.ops
        );
    }
}

fn pair(case: &'static str, label: String, value: &Value) -> (ResultRow, ResultRow) {
    let stringify_ops = bench_stringify(value);
    let text = stringify_simple(value).unwrap();
    let parse_ops = bench_parse(&text);
    (
        ResultRow {
            case,
            op: "stringify",
            label: label.clone(),
            ops: stringify_ops,
        },
        ResultRow {
            case,
            op: "parse",
            label,
            ops: parse_ops,
        },
    )
}

fn main() {
    let json = env::args().any(|a| a == "--json");
    let data = load_data_json();
    let keys = data.as_array().unwrap().borrow()[0]
        .as_object()
        .unwrap()
        .borrow()
        .len();

    if !json {
        println!("-----------------------------------");
        println!("Object with {keys} keys each");
        println!("-----------------------------------");
    }

    let dummy100 = data.clone();
    let dummy50 = slice_array(&data, 50);
    let dummy10 = slice_array(&data, 10);

    for (case, label, value) in [
        ("objects_100", "100 objects".to_string(), &dummy100),
        ("objects_50", "50 objects".to_string(), &dummy50),
        ("objects_10", "10 objects".to_string(), &dummy10),
    ] {
        let (s, p) = pair(case, label, value);
        emit(&s, json);
        emit(&p, json);
    }

    if !json {
        println!("-----------------------------------");
        println!("50% same objects");
        println!("-----------------------------------");
    }
    let shared50 = concat_shared(&[&dummy50, &dummy50]);
    let (s, p) = pair("shared_50", "100 objects".into(), &shared50);
    emit(&s, json);
    emit(&p, json);

    if !json {
        println!("-----------------------------------");
        println!("90% same objects");
        println!("-----------------------------------");
    }
    let shared90 = concat_shared(&[
        &dummy10, &dummy10, &dummy10, &dummy10, &dummy10, &dummy10, &dummy10, &dummy10, &dummy10,
        &dummy10,
    ]);
    let (s, p) = pair("shared_90", "100 objects".into(), &shared90);
    emit(&s, json);
    emit(&p, json);

    if !json {
        println!("-----------------------------------");
        println!("with circular");
        println!("-----------------------------------");
    }
    let circ = circular_list(100);
    let (s, p) = pair("circular_100", "100 objects".into(), &circ);
    emit(&s, json);
    emit(&p, json);

    if !json {
        println!("-----------------------------------");
        println!("with circular 90% same");
        println!("-----------------------------------");
    }
    let circ10 = circular_list(10);
    let circ90 = concat_shared(&[
        &circ10, &circ10, &circ10, &circ10, &circ10, &circ10, &circ10, &circ10, &circ10, &circ10,
    ]);
    let (s, p) = pair("circular_shared_90", "100 objects".into(), &circ90);
    emit(&s, json);
    emit(&p, json);

    // Big real-world: prefer preconverted flatted payload (from compare harness).
    let flatted_path = repo_test_dir().join(".bench-circular.flatted.json");
    if flatted_path.is_file() {
        if !json {
            println!("-----------------------------------");
            println!("Big real-world circular data");
            println!("-----------------------------------");
        }
        let text = fs::read_to_string(&flatted_path).expect("circular flatted");
        let value = parse_simple(&text).expect("parse circular flatted");
        let label = format!("{} chars", text.len());
        let stringify_ops = bench_stringify(&value);
        let parse_ops = bench_parse(&text);
        emit(
            &ResultRow {
                case: "big_circular",
                op: "stringify",
                label: label.clone(),
                ops: stringify_ops,
            },
            json,
        );
        emit(
            &ResultRow {
                case: "big_circular",
                op: "parse",
                label,
                ops: parse_ops,
            },
            json,
        );
    } else if !json {
        println!("-----------------------------------");
        println!(
            "Big real-world circular data (skipped: missing {})",
            flatted_path.display()
        );
        println!("Run: node test/bench-js-vs-rs.mjs   # prepares the fixture");
        println!("-----------------------------------");
    }
}
