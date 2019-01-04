extern crate ordered_float;

use ordered_float::OrderedFloat;

#[cfg(feature = "immutable")]
extern crate im;

#[cfg(feature = "immutable")]
use immutable::{Map, Set, Vec};
#[cfg(not(feature = "immutable"))]
use standard::{Map, Set, Vec};

#[cfg(feature = "immutable")]
use im::{HashMap, HashSet, Vector};
#[cfg(not(feature = "immutable"))]
use std::collections::{BTreeSet,BTreeMap};

#[cfg(feature = "immutable")]
use std::hash::Hash;

#[cfg(not(feature = "immutable"))]
mod standard;
#[cfg(feature = "immutable")]
mod immutable;

use std::fmt;

pub mod parser;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

// TODO.
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug not implemented for Value")
    }
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

#[cfg(not(feature = "immutable"))]
impl<A> From<Vec<A>> for Value
    where
        Value: From<A>,
{
    fn from(s: Vec<A>) -> Self {
        Value::Vector(s.into_iter().map(Value::from).collect())
    }
}

#[cfg(feature = "immutable")]
impl<A> From<Vector<A>> for Value
    where
        A: Clone + Hash + Eq,
        Value: From<A>,
{
    fn from(s: Vector<A>) -> Self {
        Value::Vector(s.iter().map(|a| Value::from(a.clone())).collect())
    }
}


#[cfg(not(feature = "immutable"))]
impl<K, V> From<BTreeMap<K, V>> for Value
    where
        Value: From<K>,
        Value: From<V>,
{
    fn from(s: Map<K, V>) -> Self {
        let mut map = Map::new();
        for (k, v) in s {
            map.insert(Value::from(k), Value::from(v));
        }
        Value::Map(map)
    }
}


#[cfg(feature = "immutable")]
impl<K, V> From<HashMap<K, V>> for Value
    where
        K: Clone + Hash + Eq,
        V: Clone + Hash + Eq,
        Value: From<K>,
        Value: From<V>,
{
    fn from(s: HashMap<K, V>) -> Self {
        Value::Map(
            s.iter()
                .map(|(k, v)|
                    (Value::from(k.clone()), Value::from(v.clone()))).collect())
    }
}

#[cfg(not(feature = "immutable"))]
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


#[cfg(feature = "immutable")]
impl<A> From<HashSet<A>> for Value
    where
        A: Clone + Hash + Eq,
        Value: From<A>,
{
    fn from(s: HashSet<A>) -> Self {
        Value::Set(s.iter().map(|v| Value::from(v.clone())).collect())
    }
}
