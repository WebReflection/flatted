# flatted (Rust)

A super light and fast circular JSON parser.

## Usage

```toml
[dependencies]
flatted = { path = "../rust" } # or publish path / git dependency
```

```rust
use std::cell::RefCell;
use std::rc::Rc;

use flatted::{parse_simple, stringify_simple, Object, Value};

fn main() {
    // const a = [{}]; a[0].a = a; a.push(a);
    let a = Value::Array(Rc::new(RefCell::new(vec![Value::empty_object()])));
    let first = a.as_array().unwrap().borrow()[0].clone();
    first
        .as_object()
        .unwrap()
        .borrow_mut()
        .insert("a".into(), a.clone());
    a.as_array().unwrap().borrow_mut().push(a.clone());

    let s = stringify_simple(&a).unwrap();
    assert_eq!(s, r#"[["1","0"],{"a":"0"}]"#);

    let back = parse_simple(&s).unwrap();
    let again = back.as_array().unwrap().borrow()[1].clone();
    let self_ref = again.as_object().unwrap().borrow().get("a").cloned().unwrap();
    assert!(back.ptr_eq(&self_ref));
}
```

Arrays and objects use `Rc<RefCell<_>>` so circular and shared references keep
their identity after parsing (Rust has no built-in reference-typed JSON value).

## API

| Function | Role |
|----------|------|
| `stringify(value, replacer, space)` | Flatten to flatted JSON text |
| `parse(text, reviver)` | Restore a recursive value graph |
| `to_json(value)` | Flatten to a plain `serde_json::Value` array |
| `from_json(value)` | Restore recursion from that array |
| `stringify_simple` / `parse_simple` | Same without replacer/reviver/space |

## CLI

```bash
cargo build --release --features cli
echo '{"a":"b"}' | ./target/release/flatted
echo '[{"a":"1"},"b"]' | ./target/release/flatted -d
```

## Test

```bash
cargo test
```

## Bench (vs JS)

Same 1s wall-clock methodology as `test/bench.js`:

```bash
# from repo root — compares ESM/JS vs Rust release
npm run bench:js-rs

# or individually
node test/bench-flatted.mjs
cargo run --example bench --release --manifest-path rust/Cargo.toml
```

## Note on crates.io

The name `flatted` is already taken on crates.io by an unofficial port. This
directory is the official WebReflection port in this repository; publish naming
can be decided separately (for example a scoped / renamed crate).
