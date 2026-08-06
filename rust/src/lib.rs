//! A super light and fast circular JSON parser.
//!
//! Port of [flatted](https://github.com/WebReflection/flatted) for Rust.
//!
//! The wire format is a JSON array where index `0` is the root. Objects, arrays,
//! and strings are stored once and replaced elsewhere by their index as a
//! decimal string. Only round-trip through `parse(stringify(value))` — do not
//! mix with plain `serde_json` encode/decode of the same payload.
//!
//! Arrays and objects use `Rc<RefCell<_>>` so circular and shared references
//! keep their identity after parsing.

mod value;

use std::collections::HashMap;

use serde_json::{Map as JsonMap, Value as JsonValue};

pub use value::{Array, Object, ObjectRef, Value};

/// Error returned by flatted operations.
#[derive(Debug)]
pub enum Error {
    /// Invalid JSON / flatted text.
    Json(serde_json::Error),
    /// Flatted payload is not a top-level array.
    ExpectedArray,
    /// String index did not point at a valid slot.
    InvalidIndex(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json(e) => write!(f, "{e}"),
            Error::ExpectedArray => write!(f, "flatted input must be a JSON array"),
            Error::InvalidIndex(s) => write!(f, "invalid flatted index: {s}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}

/// Optional key whitelist passed to [`stringify`], mirroring `JSON.stringify`.
pub type Replacer<'a> = Option<&'a [&'a str]>;

/// Converts a value into a specialized flatted JSON string.
///
/// `replacer`, when set, is a key whitelist (like `JSON.stringify`'s array form).
/// `space`, when set, pretty-prints each node with that indent string (JS style).
pub fn stringify(
    value: &Value,
    replacer: Replacer<'_>,
    space: Option<&str>,
) -> Result<String, Error> {
    let mut known_objects: HashMap<*const (), String> = HashMap::new();
    let mut known_strings: HashMap<String, String> = HashMap::new();
    let mut input: Vec<Value> = Vec::new();

    push_known(
        value.clone(),
        &mut known_objects,
        &mut known_strings,
        &mut input,
    );

    let mut output: Vec<JsonValue> = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let current = input[i].clone();
        let transformed = transform(
            &current,
            replacer,
            &mut known_objects,
            &mut known_strings,
            &mut input,
        );
        output.push(transformed);
        i += 1;
    }

    if let Some(indent) = space {
        Ok(join_pretty(&output, indent)?)
    } else {
        Ok(serde_json::to_string(&JsonValue::Array(output))?)
    }
}

fn push_known(
    v: Value,
    known_objects: &mut HashMap<*const (), String>,
    known_strings: &mut HashMap<String, String>,
    input: &mut Vec<Value>,
) -> String {
    let idx = input.len().to_string();
    if let Some(ptr) = v.identity_ptr() {
        known_objects.insert(ptr, idx.clone());
    } else if let Value::String(ref s) = v {
        known_strings.insert(s.clone(), idx.clone());
    }
    input.push(v);
    idx
}

fn relate(
    v: &Value,
    known_objects: &mut HashMap<*const (), String>,
    known_strings: &mut HashMap<String, String>,
    input: &mut Vec<Value>,
) -> JsonValue {
    match v {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Number(n) => JsonValue::Number(n.clone()),
        Value::String(s) => {
            if let Some(idx) = known_strings.get(s) {
                return JsonValue::String(idx.clone());
            }
            JsonValue::String(push_known(
                Value::String(s.clone()),
                known_objects,
                known_strings,
                input,
            ))
        }
        Value::Array(_) | Value::Object(_) => {
            let ptr = v.identity_ptr().expect("container has identity");
            if let Some(idx) = known_objects.get(&ptr) {
                return JsonValue::String(idx.clone());
            }
            JsonValue::String(push_known(v.clone(), known_objects, known_strings, input))
        }
    }
}

fn transform(
    v: &Value,
    replacer: Replacer<'_>,
    known_objects: &mut HashMap<*const (), String>,
    known_strings: &mut HashMap<String, String>,
    input: &mut Vec<Value>,
) -> JsonValue {
    match v {
        Value::Array(arr) => {
            let items: Vec<JsonValue> = arr
                .borrow()
                .iter()
                .map(|item| relate(item, known_objects, known_strings, input))
                .collect();
            JsonValue::Array(items)
        }
        Value::Object(obj) => {
            let mut map = JsonMap::new();
            for (key, item) in obj.borrow().iter() {
                if let Some(whitelist) = replacer {
                    if !whitelist.iter().any(|k| *k == key) {
                        continue;
                    }
                }
                map.insert(
                    key.clone(),
                    relate(item, known_objects, known_strings, input),
                );
            }
            JsonValue::Object(map)
        }
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Number(n) => JsonValue::Number(n.clone()),
        Value::String(s) => JsonValue::String(s.clone()),
    }
}

/// Match JS: `'[' + output.map(v => JSON.stringify(v, null, space)).join(',') + ']'`.
fn join_pretty(items: &[JsonValue], indent: &str) -> Result<String, Error> {
    let mut parts = Vec::with_capacity(items.len());
    for item in items {
        let pretty = pretty_with_indent(item, indent)?;
        parts.push(pretty);
    }
    let mut out = String::from('[');
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str(part);
    }
    out.push(']');
    Ok(out)
}

fn pretty_with_indent(value: &JsonValue, indent: &str) -> Result<String, Error> {
    // serde_json always pretty-prints with two spaces; remap when needed.
    let two_space = serde_json::to_string_pretty(value)?;
    if indent == "  " {
        Ok(two_space)
    } else {
        Ok(remap_indent(&two_space, "  ", indent))
    }
}

fn remap_indent(text: &str, from: &str, to: &str) -> String {
    text.lines()
        .map(|line| {
            let mut depth = 0;
            let mut rest = line;
            while rest.starts_with(from) {
                depth += 1;
                rest = &rest[from.len()..];
            }
            format!("{}{}", to.repeat(depth), rest)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Converts a specialized flatted string into a value graph.
///
/// `reviver`, when set, is called as `reviver(key, value)` for each property,
/// including the root with key `""`, matching `JSON.parse`.
pub fn parse<F>(text: &str, reviver: Option<F>) -> Result<Value, Error>
where
    F: FnMut(String, Value) -> Value,
{
    let json: JsonValue = serde_json::from_str(text)?;
    let JsonValue::Array(flat) = json else {
        return Err(Error::ExpectedArray);
    };

    // Phase 1: allocate shells so circular links can share identity.
    let input: Vec<Value> = flat
        .iter()
        .map(|v| match v {
            JsonValue::Array(_) => Value::empty_array(),
            JsonValue::Object(_) => Value::empty_object(),
            JsonValue::Null => Value::Null,
            JsonValue::Bool(b) => Value::Bool(*b),
            JsonValue::Number(n) => Value::Number(n.clone()),
            JsonValue::String(s) => Value::String(s.clone()),
        })
        .collect();

    // Phase 2: fill arrays and objects; nested strings are indexes into `input`.
    for (i, v) in flat.iter().enumerate() {
        match v {
            JsonValue::Array(items) => {
                let Value::Array(target) = &input[i] else {
                    unreachable!("shell type mismatch");
                };
                let mut slot = target.borrow_mut();
                slot.reserve(items.len());
                for item in items {
                    slot.push(resolve_ref(item, &input)?);
                }
            }
            JsonValue::Object(map) => {
                let Value::Object(target) = &input[i] else {
                    unreachable!("shell type mismatch");
                };
                let mut slot = target.borrow_mut();
                for (key, item) in map {
                    slot.insert(key.clone(), resolve_ref(item, &input)?);
                }
            }
            _ => {}
        }
    }

    let root = input.into_iter().next().unwrap_or(Value::Null);

    if let Some(mut revive) = reviver {
        Ok(apply_reviver(&mut revive, String::new(), root))
    } else {
        Ok(root)
    }
}

fn resolve_ref(item: &JsonValue, input: &[Value]) -> Result<Value, Error> {
    match item {
        JsonValue::String(s) => {
            let idx: usize = s.parse().map_err(|_| Error::InvalidIndex(s.clone()))?;
            input
                .get(idx)
                .cloned()
                .ok_or_else(|| Error::InvalidIndex(s.clone()))
        }
        JsonValue::Null => Ok(Value::Null),
        JsonValue::Bool(b) => Ok(Value::Bool(*b)),
        JsonValue::Number(n) => Ok(Value::Number(n.clone())),
        JsonValue::Array(_) | JsonValue::Object(_) => {
            Err(Error::InvalidIndex("nested container without index".into()))
        }
    }
}

fn apply_reviver<F>(revive: &mut F, key: String, value: Value) -> Value
where
    F: FnMut(String, Value) -> Value,
{
    match &value {
        Value::Array(arr) => {
            let len = arr.borrow().len();
            for i in 0..len {
                let child = arr.borrow()[i].clone();
                let revived = apply_reviver(revive, i.to_string(), child);
                arr.borrow_mut()[i] = revived;
            }
        }
        Value::Object(obj) => {
            let keys: Vec<String> = obj.borrow().keys().cloned().collect();
            for k in keys {
                let child = obj.borrow().get(&k).cloned().unwrap();
                let revived = apply_reviver(revive, k.clone(), child);
                obj.borrow_mut().insert(k, revived);
            }
        }
        _ => {}
    }
    revive(key, value)
}

/// Converts a value into a JSON-serializable flatted array without losing recursion.
pub fn to_json(value: &Value) -> Result<JsonValue, Error> {
    let text = stringify(value, None, None)?;
    Ok(serde_json::from_str(&text)?)
}

/// Converts a previously flattened JSON array back into a recursive value.
pub fn from_json(value: &JsonValue) -> Result<Value, Error> {
    let text = serde_json::to_string(value)?;
    parse_simple(&text)
}

/// Convenience: stringify with no replacer or space.
pub fn stringify_simple(value: &Value) -> Result<String, Error> {
    stringify(value, None, None)
}

/// Convenience: parse with no reviver.
pub fn parse_simple(text: &str) -> Result<Value, Error> {
    parse(text, None::<fn(String, Value) -> Value>)
}
