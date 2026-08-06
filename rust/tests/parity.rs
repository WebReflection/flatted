use std::cell::RefCell;
use std::rc::Rc;

use flatted::{
    from_json, parse, parse_simple, stringify, stringify_simple, to_json, Object, Value,
};
use serde_json::json;

fn obj(entries: &[(&str, Value)]) -> Value {
    let mut map = Object::new();
    for (k, v) in entries {
        map.insert((*k).to_owned(), v.clone());
    }
    Value::Object(Rc::new(RefCell::new(map)))
}

fn arr(items: Vec<Value>) -> Value {
    Value::Array(Rc::new(RefCell::new(items)))
}

#[test]
fn multiple_nulls() {
    let a = arr(vec![Value::Null, Value::Null]);
    assert_eq!(stringify_simple(&a).unwrap(), "[[null,null]]");
}

#[test]
fn empty_collections() {
    assert_eq!(stringify_simple(&arr(vec![])).unwrap(), "[[]]");
    assert_eq!(stringify_simple(&obj(&[])).unwrap(), "[{}]");
}

#[test]
fn recursive_array() {
    let a = arr(vec![]);
    a.as_array().unwrap().borrow_mut().push(a.clone());
    assert_eq!(stringify_simple(&a).unwrap(), r#"[["0"]]"#);

    let back = parse_simple(r#"[["0"]]"#).unwrap();
    let first = back.as_array().unwrap().borrow()[0].clone();
    assert!(back.ptr_eq(&first));
}

#[test]
fn recursive_object() {
    let o = obj(&[]);
    o.as_object()
        .unwrap()
        .borrow_mut()
        .insert("o".into(), o.clone());
    assert_eq!(stringify_simple(&o).unwrap(), r#"[{"o":"0"}]"#);

    let back = parse_simple(r#"[{"o":"0"}]"#).unwrap();
    let inner = back
        .as_object()
        .unwrap()
        .borrow()
        .get("o")
        .cloned()
        .unwrap();
    assert!(back.ptr_eq(&inner));
}

#[test]
fn values_in_array_and_object() {
    let a = arr(vec![]);
    {
        let mut slot = a.as_array().unwrap().borrow_mut();
        slot.push(a.clone());
        slot.push(Value::from(1_i64));
        slot.push(Value::from("two"));
        slot.push(Value::from(true));
    }
    assert_eq!(stringify_simple(&a).unwrap(), r#"[["0",1,"1",true],"two"]"#);

    let o = obj(&[]);
    {
        let mut slot = o.as_object().unwrap().borrow_mut();
        slot.insert("o".into(), o.clone());
        slot.insert("one".into(), Value::from(1_i64));
        slot.insert("two".into(), Value::from("two"));
        slot.insert("three".into(), Value::from(true));
    }
    assert_eq!(
        stringify_simple(&o).unwrap(),
        r#"[{"o":"0","one":1,"two":"1","three":true},"two"]"#
    );
}

#[test]
fn object_in_array_and_array_in_object() {
    let a = arr(vec![]);
    let o = obj(&[]);

    {
        let mut slot = a.as_array().unwrap().borrow_mut();
        slot.push(a.clone());
        slot.push(Value::from(1_i64));
        slot.push(Value::from("two"));
        slot.push(Value::from(true));
        slot.push(o.clone());
    }
    {
        let mut slot = o.as_object().unwrap().borrow_mut();
        slot.insert("o".into(), o.clone());
        slot.insert("one".into(), Value::from(1_i64));
        slot.insert("two".into(), Value::from("two"));
        slot.insert("three".into(), Value::from(true));
        slot.insert("a".into(), a.clone());
    }

    assert_eq!(
        stringify_simple(&a).unwrap(),
        r#"[["0",1,"1",true,"2"],"two",{"o":"2","one":1,"two":"1","three":true,"a":"0"}]"#
    );
    assert_eq!(
        stringify_simple(&o).unwrap(),
        r#"[{"o":"0","one":1,"two":"1","three":true,"a":"2"},"two",["2",1,"1",true,"0"]]"#
    );
}

#[test]
fn objects_in_array_js_identity() {
    // JS keeps equal-but-distinct objects separate (unlike Python equality dedup).
    let a = arr(vec![]);
    let o = obj(&[]);

    {
        let mut slot = a.as_array().unwrap().borrow_mut();
        slot.extend([
            a.clone(),
            Value::from(1_i64),
            Value::from("two"),
            Value::from(true),
            o.clone(),
        ]);
    }
    {
        let mut slot = o.as_object().unwrap().borrow_mut();
        slot.insert("o".into(), o.clone());
        slot.insert("one".into(), Value::from(1_i64));
        slot.insert("two".into(), Value::from("two"));
        slot.insert("three".into(), Value::from(true));
        slot.insert("a".into(), a.clone());
    }

    let test_a = obj(&[("test", Value::from("OK"))]);
    let array_a = arr(vec![
        Value::from(1_i64),
        Value::from(2_i64),
        Value::from(3_i64),
    ]);
    let test_o = obj(&[("test", Value::from("OK"))]);
    let array_o = arr(vec![
        Value::from(1_i64),
        Value::from(2_i64),
        Value::from(3_i64),
    ]);

    a.as_array().unwrap().borrow_mut().push(test_a);
    a.as_array().unwrap().borrow_mut().push(array_a);
    o.as_object()
        .unwrap()
        .borrow_mut()
        .insert("test".into(), test_o);
    o.as_object()
        .unwrap()
        .borrow_mut()
        .insert("array".into(), array_o);

    assert_eq!(
        stringify_simple(&a).unwrap(),
        r#"[["0",1,"1",true,"2","3","4"],"two",{"o":"2","one":1,"two":"1","three":true,"a":"0","test":"5","array":"6"},{"test":"7"},[1,2,3],{"test":"7"},[1,2,3],"OK"]"#
    );
    assert_eq!(
        stringify_simple(&o).unwrap(),
        r#"[{"o":"0","one":1,"two":"1","three":true,"a":"2","test":"3","array":"4"},"two",["2",1,"1",true,"0","5","6"],{"test":"7"},[1,2,3],{"test":"7"},[1,2,3],"OK"]"#
    );
}

#[test]
fn roundtrip_primitives() {
    for value in [
        Value::from(1_i64),
        Value::from(false),
        Value::Null,
        Value::from("test"),
    ] {
        let text = stringify_simple(&value).unwrap();
        let back = parse_simple(&text).unwrap();
        assert_eq!(back, value);
    }
}

#[test]
fn readme_circular_example() {
    // const a = [{}]; a[0].a = a; a.push(a);
    // stringify(a) => [["1","0"],{"a":"0"}]
    let a = arr(vec![obj(&[])]);
    let first = a.as_array().unwrap().borrow()[0].clone();
    first
        .as_object()
        .unwrap()
        .borrow_mut()
        .insert("a".into(), a.clone());
    a.as_array().unwrap().borrow_mut().push(a.clone());
    assert_eq!(stringify_simple(&a).unwrap(), r#"[["1","0"],{"a":"0"}]"#);
}

#[test]
fn special_strings() {
    let special = r"\x7e";
    let o = obj(&[("a", Value::from(special))]);
    let back = parse_simple(&stringify_simple(&o).unwrap()).unwrap();
    assert_eq!(
        back.as_object().unwrap().borrow().get("a"),
        Some(&Value::from(special))
    );
}

#[test]
fn replacer_whitelist() {
    let o = obj(&[("a", Value::from(1_i64)), ("b", Value::from(2_i64))]);
    let s = stringify(&o, Some(&["b"]), None).unwrap();
    assert_eq!(s, r#"[{"b":2}]"#);
}

#[test]
fn indentation_js_style() {
    let o = obj(&[("a", Value::from(1_i64))]);
    // JS: '[' + JSON.stringify({a:1}, null, 2) + ']' => '[{\n  "a": 1\n}]'
    assert_eq!(
        stringify(&o, None, Some("  ")).unwrap(),
        "[{\n  \"a\": 1\n}]"
    );
}

#[test]
fn to_from_json() {
    let a = arr(vec![obj(&[])]);
    let first = a.as_array().unwrap().borrow()[0].clone();
    first
        .as_object()
        .unwrap()
        .borrow_mut()
        .insert("a".into(), a.clone());

    let flat = to_json(&a).unwrap();
    let back = from_json(&flat).unwrap();
    let m = back.as_array().unwrap().borrow()[0].clone();
    let self_ref = m.as_object().unwrap().borrow().get("a").cloned().unwrap();
    assert!(back.ptr_eq(&self_ref));
}

#[test]
fn complex_shared_structure() {
    let unique = obj(&[("a", Value::from("sup"))]);
    let nested = obj(&[
        ("prop", obj(&[("value", Value::from(123_i64))])),
        (
            "a",
            arr(vec![
                obj(&[]),
                obj(&[(
                    "b",
                    arr(vec![obj(&[
                        ("a", Value::from(1_i64)),
                        ("d", Value::from(2_i64)),
                        ("c", unique.clone()),
                        (
                            "z",
                            obj(&[
                                ("g", Value::from(2_i64)),
                                ("a", unique.clone()),
                                (
                                    "b",
                                    obj(&[
                                        ("r", Value::from(4_i64)),
                                        ("u", unique.clone()),
                                        ("c", Value::from(5_i64)),
                                    ]),
                                ),
                                ("f", Value::from(6_i64)),
                            ]),
                        ),
                        ("h", Value::from(1_i64)),
                    ])]),
                )]),
            ]),
        ),
        (
            "b",
            obj(&[
                ("e", Value::from("f")),
                ("t", unique.clone()),
                ("p", Value::from(4_i64)),
            ]),
        ),
    ]);

    let text = stringify_simple(&nested).unwrap();
    let p = parse_simple(&text).unwrap();

    let res_b_t = p
        .as_object()
        .unwrap()
        .borrow()
        .get("b")
        .unwrap()
        .as_object()
        .unwrap()
        .borrow()
        .get("t")
        .cloned()
        .unwrap();

    let res_a_c = {
        let a = p.as_object().unwrap().borrow().get("a").cloned().unwrap();
        let second = a.as_array().unwrap().borrow()[1].clone();
        let b = second
            .as_object()
            .unwrap()
            .borrow()
            .get("b")
            .cloned()
            .unwrap();
        let first = b.as_array().unwrap().borrow()[0].clone();
        let c = first
            .as_object()
            .unwrap()
            .borrow()
            .get("c")
            .cloned()
            .unwrap();
        c
    };

    assert!(res_b_t.ptr_eq(&res_a_c));
}

#[test]
fn empty_keys_circular() {
    let inner = obj(&[("d", Value::from(1_i64))]);
    let empty_key_map = obj(&[("c", inner)]);
    let b_map = obj(&[("", empty_key_map.clone())]);
    let a = obj(&[("b", b_map), ("_circular", empty_key_map)]);

    let back = parse_simple(&stringify_simple(&a).unwrap()).unwrap();
    let circular = back
        .as_object()
        .unwrap()
        .borrow()
        .get("_circular")
        .cloned()
        .unwrap();
    let via_empty = back
        .as_object()
        .unwrap()
        .borrow()
        .get("b")
        .unwrap()
        .as_object()
        .unwrap()
        .borrow()
        .get("")
        .cloned()
        .unwrap();
    assert!(circular.ptr_eq(&via_empty));
}

#[test]
fn deep_chain() {
    let amount = 1500;
    let mut chain = arr(vec![Value::from("leaf")]);
    for _ in 0..amount {
        chain = arr(vec![chain]);
    }
    let text = stringify_simple(&chain).unwrap();
    let mut parsed = parse_simple(&text).unwrap();
    for _ in 0..amount {
        let next = parsed.as_array().unwrap().borrow()[0].clone();
        parsed = next;
    }
    // Innermost value is the original `["leaf"]` array.
    assert_eq!(parsed.as_array().unwrap().borrow()[0], Value::from("leaf"));
}

#[test]
fn specs_examples() {
    assert_eq!(stringify_simple(&Value::from("a")).unwrap(), r#"["a"]"#);
    assert_eq!(
        stringify_simple(&arr(vec![Value::from("a")])).unwrap(),
        r#"[["1"],"a"]"#
    );
    assert_eq!(
        stringify_simple(&arr(vec![
            Value::from("a"),
            Value::from(1_i64),
            Value::from("b")
        ]))
        .unwrap(),
        r#"[["1",1,"2"],"a","b"]"#
    );
    assert_eq!(
        stringify_simple(&obj(&[("a", Value::from("a"))])).unwrap(),
        r#"[{"a":"1"},"a"]"#
    );
    assert_eq!(
        stringify_simple(&obj(&[
            ("a", Value::from("a")),
            ("n", Value::from(1_i64)),
            ("b", Value::from("b")),
        ]))
        .unwrap(),
        r#"[{"a":"1","n":1,"b":"2"},"a","b"]"#
    );
}

#[test]
fn large_fixtures() {
    for name in ["65515.json", "65518.json"] {
        let path = format!("../test/{name}");
        let Ok(data) = std::fs::read_to_string(&path) else {
            continue;
        };
        let raw: serde_json::Value = serde_json::from_str(&data).unwrap();
        let tool_data = raw.get("toolData").expect("toolData");
        let text = serde_json::to_string(tool_data).unwrap();
        let parsed = parse_simple(&text).unwrap();
        assert!(!matches!(parsed, Value::Null));
    }
}

#[test]
fn reviver_transforms_root() {
    let text = stringify_simple(&Value::from(1_i64)).unwrap();
    let back = parse(
        &text,
        Some(|_k, v| match v {
            Value::Number(n) if n.as_i64() == Some(1) => Value::from(42_i64),
            other => other,
        }),
    )
    .unwrap();
    assert_eq!(back, Value::from(42_i64));
}

#[test]
fn from_json_value_conversion() {
    let v = Value::from(json!({"a": [1, true, null]}));
    let text = stringify_simple(&v).unwrap();
    let back = parse_simple(&text).unwrap();
    assert_eq!(
        back.as_object()
            .unwrap()
            .borrow()
            .get("a")
            .unwrap()
            .as_array()
            .unwrap()
            .borrow()
            .len(),
        3
    );
}
