use std::error::Error;

use serde::Serialize;

use super::haltech::{HaltechChannel, HaltechMeta};

#[derive(Clone, Debug, Serialize)]
pub enum Meta {
  Haltech(HaltechMeta),
  Empty,
}

impl Default for Meta {
  fn default() -> Self {
    Meta::Empty
  }
}

#[derive(Clone, Debug)]
pub enum Channel {
  Haltech(HaltechChannel),
}

// Implements Serialize for Channel manually to "unwrap" the value inside the
// enum. Without it, a channel is serialized as an object with a single key
// e.g. { "Haltech": { ... } }
impl Serialize for Channel {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      Channel::Haltech(h) => h.serialize(serializer),
    }
  }
}

impl Channel {
  pub fn name(&self) -> String {
    match self {
      Channel::Haltech(h) => h.name.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub enum Value {
  _Bool(bool),
  _Float(f64),
  Int(i64),
  _String(String),
}

// Implements Serialize for Value manually to "unwrap" the value inside
// the enum. Without it, a Value is serialized as an object with a single key
// e.g. { "Int": 5 } rather than just the value e.g. 5
impl Serialize for Value {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      Value::_Bool(b) => serializer.serialize_bool(*b),
      Value::_Float(f) => serializer.serialize_f64(*f),
      Value::Int(i) => serializer.serialize_i64(*i),
      Value::_String(s) => serializer.serialize_str(s),
    }
  }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Log {
  pub meta: Meta,
  pub channels: Vec<Channel>,
  pub times: Vec<String>,
  pub data: Vec<Vec<Value>>,
}

pub trait Parseable {
  fn parse(&self, data: &str) -> Result<Log, Box<dyn Error>>;
}