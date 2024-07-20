use std::error::Error;
use std::str::FromStr;

use regex::Regex;
use serde::Serialize;
use strum::{AsRefStr, EnumString};

use crate::parsers::types::{Log, Parseable};

use super::types::{Channel, ChannelValue, Meta};

#[derive(AsRefStr, Clone, Debug, EnumString, Serialize)]
pub enum ChannelType {
  AFR,
  AbsPressure,
  Acceleration,
  Angle,
  AngularVelocity,
  BatteryVoltage,
  BoostToFuelFlowRate,
  ByteCount,
  Current,
  #[strum(serialize = "Current_uA_as_mA")]
  CurrentMicroampsAsMilliamps,
  #[strum(serialize = "Current_mA_as_A")]
  CurrentMilliampsAsAmps,
  Decibel,
  Density,
  EngineSpeed,
  EngineVolume,
  Flow,
  Frequency,
  #[strum(serialize = "FuelEcomony")]
  FuelEconomy,
  FuelVolume,
  Gear,
  GearRatio,
  InjFuelVolume,
  MassOverTime,
  #[strum(serialize = "MassPerCyl")]
  MassPerCylinder,
  Mileage,
  PercentPerEngineCycle,
  PercentPerLambda,
  #[strum(serialize = "PercentPerRpm")]
  PercentPerRPM,
  Percentage,
  Pressure,
  PulsesPerLongDistance,
  Ratio,
  Raw,
  Resistance,
  Speed,
  Stoichiometry,
  Temperature,
  #[strum(serialize = "Time_us")]
  TimeMicroseconds,
  #[strum(serialize = "TimeUsAsUs")]
  TimeMicrosecondsAsMicroseconds,
  #[strum(serialize = "Time_ms_as_s")]
  TimeMillisecondsAsSeconds,
  #[strum(serialize = "Time_ms")]
  TimeMilliseconds,
  #[strum(serialize = "Time_s")]
  TimeSeconds,
}

impl Default for ChannelType {
  fn default() -> Self { ChannelType::Raw }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct HaltechMeta {
  pub data_log_version: String,
  pub software: String,
  pub software_version: String,
  pub download_date_time: String,
  pub log_source: String,
  pub log_number: String,
  pub log_date_time: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct HaltechChannel {
  pub name: String,
  pub id: String,
  pub r#type: ChannelType,
  pub display_min: Option<f64>,
  pub display_max: Option<f64>,
}

pub struct Haltech {}

impl Parseable for Haltech {
  fn parse(&self, file_contents: &str) -> Result<Log, Box<dyn Error>> {
    let mut meta = HaltechMeta::default();
    let mut channels = vec![];
    let mut times = vec![];
    let mut data = vec![];

    let regex = Regex::new(r"(?<name>.+) : (?<value>.+)")
      .expect("Failed to compile regex");

    let mut current_channel = HaltechChannel::default();
    for line in file_contents.lines() {
      let line = line.trim();

      // Start by attempting to parse the line as a meta/channel key-value pair
      if let Some(captures) = regex.captures(line) {
        let name = captures["name"].trim();
        let value = captures["value"].trim().to_string();

        match name {
          "DataLogVersion" => meta.data_log_version = value,
          "Software" => meta.software = value,
          "SoftwareVersion" => meta.software_version = value,
          "DownloadDate/Time" => meta.download_date_time = value,
          "Log Source" => meta.log_source = value,
          "Log Number" => meta.log_number = value,
          "Log" => meta.log_date_time = value,
          // The "Channel" key indicates the start of a new channel
          // so this assumes the previous channel is complete and adds it to
          // the list
          "Channel" => {
            if !current_channel.name.is_empty() {
              channels.push(Channel::Haltech(current_channel));
            }

            current_channel = HaltechChannel::default();
            current_channel.name = value;
          }
          "Id" => current_channel.id = value,
          "Type" => {
            if let Ok(channel_type) = ChannelType::from_str(&value) {
              current_channel.r#type = channel_type;
            } else {
              eprintln!("Failed to parse channel type: {}", value);
            }
          }
          "DisplayMaxMin" => {
            let values: Vec<&str> = value.split(",").collect();
            current_channel.display_max = values[0].parse().ok();
            current_channel.display_min = values[1].parse().ok();
          }
          _ => {}
        }
      } else {
        // This is not a key-value pair, so it must be channel data (CSV)
        //
        // If `current_channel` is not empty, add it to the list of channels
        if !current_channel.name.is_empty() {
          channels.push(Channel::Haltech(current_channel));
          current_channel = HaltechChannel::default();
        }

        if !line.is_empty() {
          let values = line
            .split(",")
            .enumerate()
            .filter_map(|(i, v)| {
              // The first value is always a timestamp
              if i == 0 {
                times.push(v.to_string());
                return None;
              }

              let Channel::Haltech(channel) = &channels[i - 1];
              match channel.r#type {
                _ => Some(ChannelValue::Int(v.parse().unwrap())),
              }
            })
            .collect::<Vec<_>>();

          if channels.len() > 0 && values.len() >= channels.len() {
            data.push(values);
          }
        }
      }
    }

    Ok(Log {
      meta: Meta::Haltech(meta),
      channels,
      times,
      data,
    })
  }
}