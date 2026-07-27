use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

use indexmap::IndexMap;
use serde_json::{Map as JsonMap, Number, Value as JsonValue};

/// Ordered object map (insertion order, matching JS).
pub type Object = IndexMap<String, Value>;

/// Shared mutable array handle (enables circular / shared references).
pub type Array = Rc<RefCell<Vec<Value>>>;

/// Shared mutable object handle (enables circular / shared references).
pub type ObjectRef = Rc<RefCell<Object>>;

/// JSON-compatible value with shared ownership for arrays and objects.
///
/// Unlike `serde_json::Value`, arrays and objects sit behind `Rc<RefCell<_>>`,
/// so a graph can point back into itself or share a node across positions.
#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(ObjectRef),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Value::Number(n) => f.debug_tuple("Number").field(n).finish(),
            Value::String(s) => f.debug_tuple("String").field(s).finish(),
            Value::Array(a) => write!(f, "Array(len={})", a.borrow().len()),
            Value::Object(o) => write!(f, "Object(len={})", o.borrow().len()),
        }
    }
}

impl Value {
    /// Returns `true` when both values refer to the same array or object allocation.
    pub fn ptr_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b),
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }

    /// Returns a reference to the shared array, if this value is an array.
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the shared object, if this value is an object.
    pub fn as_object(&self) -> Option<&ObjectRef> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Pointer identity for known-map deduplication during stringify.
    pub(crate) fn identity_ptr(&self) -> Option<*const ()> {
        match self {
            Value::Array(a) => Some(Rc::as_ptr(a) as *const ()),
            Value::Object(o) => Some(Rc::as_ptr(o) as *const ()),
            _ => None,
        }
    }

    /// Creates an empty shared array.
    pub fn empty_array() -> Self {
        Value::Array(Rc::new(RefCell::new(Vec::new())))
    }

    /// Creates an empty shared object.
    pub fn empty_object() -> Self {
        Value::Object(Rc::new(RefCell::new(Object::new())))
    }

    /// Converts a non-circular `serde_json::Value` tree into a flatted `Value`.
    pub fn from_json(value: &JsonValue) -> Self {
        match value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool(b) => Value::Bool(*b),
            JsonValue::Number(n) => Value::Number(n.clone()),
            JsonValue::String(s) => Value::String(s.clone()),
            JsonValue::Array(items) => {
                let arr = items.iter().map(Value::from_json).collect();
                Value::Array(Rc::new(RefCell::new(arr)))
            }
            JsonValue::Object(map) => {
                let mut obj = Object::new();
                for (k, v) in map {
                    obj.insert(k.clone(), Value::from_json(v));
                }
                Value::Object(Rc::new(RefCell::new(obj)))
            }
        }
    }

    /// Converts into a `serde_json::Value` tree.
    ///
    /// Shared references are cloned as distinct subtrees. Circular references
    /// are replaced with `Null` after the first visit (lossy). Prefer
    /// [`crate::to_json`] when recursion must be preserved.
    pub fn to_serde_json(&self) -> JsonValue {
        let mut visiting = HashSet::new();
        self.to_serde_json_inner(&mut visiting)
    }

    fn to_serde_json_inner(&self, visiting: &mut HashSet<*const ()>) -> JsonValue {
        match self {
            Value::Null => JsonValue::Null,
            Value::Bool(b) => JsonValue::Bool(*b),
            Value::Number(n) => JsonValue::Number(n.clone()),
            Value::String(s) => JsonValue::String(s.clone()),
            Value::Array(a) => {
                let ptr = Rc::as_ptr(a) as *const ();
                if !visiting.insert(ptr) {
                    return JsonValue::Null;
                }
                let items: Vec<JsonValue> = a
                    .borrow()
                    .iter()
                    .map(|v| v.to_serde_json_inner(visiting))
                    .collect();
                visiting.remove(&ptr);
                JsonValue::Array(items)
            }
            Value::Object(o) => {
                let ptr = Rc::as_ptr(o) as *const ();
                if !visiting.insert(ptr) {
                    return JsonValue::Null;
                }
                let mut map = JsonMap::new();
                for (k, v) in o.borrow().iter() {
                    map.insert(k.clone(), v.to_serde_json_inner(visiting));
                }
                visiting.remove(&ptr);
                JsonValue::Object(map)
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            // Containers compare by identity; use stringify for structural checks.
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b),
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n.into())
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Number(n.into())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Number::from_f64(n)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<Vec<Value>> for Value {
    fn from(items: Vec<Value>) -> Self {
        Value::Array(Rc::new(RefCell::new(items)))
    }
}

impl From<Object> for Value {
    fn from(map: Object) -> Self {
        Value::Object(Rc::new(RefCell::new(map)))
    }
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        Value::from_json(&value)
    }
}
