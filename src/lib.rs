use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    String(String),
    Char(char),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    Float(f64),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>)
}
