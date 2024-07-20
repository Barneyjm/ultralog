use std::error::Error;

use serde::Serialize;

use super::haltech::{HaltechChannel, HaltechMeta};

#[derive(Clone, Debug, Serialize)]
pub enum Meta {
  Haltech(HaltechMeta),
}

#[derive(Clone, Debug, Serialize)]
pub enum Channel {
  Haltech(HaltechChannel),
}

#[derive(Clone, Debug, Serialize)]
pub enum ChannelValue {
  _Bool(bool),
  _Float(f64),
  Int(i64),
  String(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct Log {
  pub meta: Meta,
  pub channels: Vec<Channel>,
  pub times: Vec<String>,
  pub data: Vec<Vec<ChannelValue>>,
}

pub trait Parseable {
  fn parse(&self, data: &str) -> Result<Log, Box<dyn Error>>;
  fn get_channel(&self, channel_name: String) -> Result<Vec<ChannelValue>, Box<dyn Error>>;
}