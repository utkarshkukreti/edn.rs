extern crate edn;
extern crate ordered_float;

use std::collections::{BTreeMap,BTreeSet};
use edn::Value;
use edn::parser::{Error, Parser};
use ordered_float::OrderedFloat;

#[test]
fn from_bool() {
    assert_eq!(Value::from(true), Value::Boolean(true));
    assert_eq!(Value::from(false), Value::Boolean(false));
}
#[test]
fn from_str() {
    assert_eq!(Value::from(""), Value::String("".to_string()));
    assert_eq!(Value::from("hello"), Value::String("hello".to_string()));
}

#[test]
fn from_string() {
    assert_eq!(Value::from("".to_string()), Value::String("".to_string()));
    assert_eq!(Value::from("hello".to_string()), Value::String("hello".to_string()));
}

#[test]
fn from_char() {
    assert_eq!(Value::from('c'), Value::Char('c'));
}

#[test]
fn from_num() {
    assert_eq!(Value::from(0_i64), Value::Integer(0));
    assert_eq!(Value::from(0), Value::Integer(0));
    assert_eq!(Value::from(-1), Value::Integer(-1));

    assert_eq!(Value::from(0_f64), Value::Float(OrderedFloat(0_f64)));
    assert_eq!(Value::from(OrderedFloat(0_f64)), Value::Float(OrderedFloat(0_f64)));
}

#[test]
fn from_vec() {
    assert_eq!(Value::from(Vec::<i64>::new()), Value::Vector(vec![]));
    assert_eq!(Value::from(Vec::<Value>::new()), Value::Vector(vec![]));
    assert_eq!(Value::from(Vec::<String>::new()), Value::Vector(vec![]));

    assert_eq!(Value::from( vec![ 1, 2, 3 ]),
        Value::Vector(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3)])
    );
}

#[test]
fn from_map() {

    let mut m = BTreeMap::new();
    m.insert(1, 2);

    let mut n = BTreeMap::new();
    n.insert(Value::Integer(1), Value::Integer(2));
    assert_eq!(Value::from(m), Value::Map(n));
}
#[test]
fn from_set() {

    let mut m = BTreeSet::new();
    m.insert(1);
    m.insert(2);

    let mut n = BTreeSet::new();
    n.insert(Value::Integer(1));
    n.insert(Value::Integer(2));
    assert_eq!(Value::from(m), Value::Set(n));
}
