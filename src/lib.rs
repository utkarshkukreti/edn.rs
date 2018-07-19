extern crate ordered_float;

use std::collections::{BTreeMap, BTreeSet};

use ordered_float::OrderedFloat;

pub mod parser;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Nil,
    Boolean(bool),
    String(String),
    Char(char),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    Float(OrderedFloat<f64>),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>),
    Tagged(String, Box<Value>),
}

impl From<bool> for Value {
    fn from(s: bool) -> Self {
        Value::Boolean(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<char> for Value {
    fn from(s: char) -> Self {
        Value::Char(s)
    }
}

impl From<i64> for Value {
    fn from(s: i64) -> Self {
        Value::Integer(s)
    }
}
impl From<f64> for Value {
    fn from(s: f64) -> Self {
        Value::Float(OrderedFloat(s))
    }
}

impl From<OrderedFloat<f64>> for Value {
    fn from(s: OrderedFloat<f64>) -> Self {
        Value::Float(s)
    }
}

impl<A> From<Vec<A>> for Value
where
    Value: From<A>,
{
    fn from(s: Vec<A>) -> Self {
        Value::Vector(s.into_iter().map(Value::from).collect())
    }
}

impl<K, V> From<BTreeMap<K, V>> for Value
where
    Value: From<K>,
    Value: From<V>,
{
    fn from(s: BTreeMap<K, V>) -> Self {
        let mut map = BTreeMap::new();
        for (k, v) in s {
            map.insert(Value::from(k), Value::from(v));
        }
        Value::Map(map)
    }
}

impl<A> From<BTreeSet<A>> for Value
where
    Value: From<A>,
{
    fn from(s: BTreeSet<A>) -> Self {
        let mut set = BTreeSet::new();
        s.into_iter().for_each(|a| {
            set.insert(Value::from(a));
        });
        Value::Set(set)
    }
}
