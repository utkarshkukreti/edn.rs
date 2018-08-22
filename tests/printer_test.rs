extern crate edn;
extern crate ordered_float;

use edn::printer::{print};

use edn::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[test]
fn test_read_empty() {
    assert_eq!(print(Value::Integer(0)), "0");
    assert_eq!(print(Value::Nil), "nil");
    assert_eq!(print(Value::Boolean(true)), "true");
    assert_eq!(print(Value::String("foo".to_string())), "\"foo\"");
    assert_eq!(print(Value::Char('a')), "\\a");
    assert_eq!(print(Value::Symbol("foo".to_string())), "foo");
    assert_eq!(print(Value::Keyword("bar".to_string())), ":bar");
    // assert_eq!(print(Value::Float(OrderedFloat(0_f64))), "0.0");
    assert_eq!(print(Value::List(vec![])), "()");
    assert_eq!(print(Value::Vector(vec![])), "[]");
    assert_eq!(
        print(Value::Vector(vec![Value::Integer(1), Value::Integer(0), Value::List(vec![Value::Nil])])),
        "[1, 0, (nil)]"
    );
    assert_eq!(print(Value::Set(BTreeSet::new())), "#{}");
    let mut s = BTreeSet::new();
    s.insert(Value::Integer(1));
    assert_eq!(print(Value::Set(s)), "#{1}");
    assert_eq!(print(Value::Map(BTreeMap::new())), "{}");
    let mut m = BTreeMap::new();
    m.insert(Value::Integer(1), Value::Integer(2));
    m.insert(Value::Keyword("foo".to_string()), Value::String("bar".to_string()));
    assert_eq!(
        print(Value::Map(m)),
        "{:foo \"bar\", 1 2}"
    );
    assert_eq!(print(Value::Tagged(
        "foo/bar".into(),
        Box::new(Value::Keyword("baz".into())))),
               "#foo/bar :baz"
    )
}
