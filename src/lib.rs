extern crate ordered_float;

#[cfg(feature = "immutable")]
extern crate im;

use ordered_float::OrderedFloat;

#[cfg(not(feature = "immutable"))]
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "immutable")]
use im::{HashMap, HashSet, Vector};

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
    Map(Map<Value, Value>),
    Set(Set<Value>),
    Tagged(String, Box<Value>),
}

// use these if immutable feature flag set
#[cfg(feature = "immutable")]
type Map<K, V> = HashMap<K, V>;
#[cfg(feature = "immutable")]
type Vec<T> = Vector<T>;
#[cfg(feature = "immutable")]
type Set<T> = HashSet<T>;

// else use defaults
#[cfg(not(feature = "immutable"))]
type Map<K, V> = BTreeMap<K, V>;
#[cfg(not(feature = "immutable"))]
type Vec<T> = std::vec::Vec<T>;
#[cfg(not(feature = "immutable"))]
type Set<T> = BTreeSet<T>;

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
