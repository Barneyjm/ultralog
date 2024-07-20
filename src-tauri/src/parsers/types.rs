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

#[derive(Clone, Debug, Serialize)]
pub enum Channel {
  Haltech(HaltechChannel),
}

impl Channel {
  pub fn name(&self) -> String {
    match self {
      Channel::Haltech(h) => h.name.clone(),
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub enum ChannelValue {
  _Bool(bool),
  _Float(f64),
  Int(i64),
  _String(String),
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Log {
  pub meta: Meta,
  pub channels: Vec<Channel>,
  pub times: Vec<String>,
  pub data: Vec<Vec<ChannelValue>>,
}

pub trait Parseable {
  fn parse(&self, data: &str) -> Result<Log, Box<dyn Error>>;
}