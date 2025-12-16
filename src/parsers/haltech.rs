use regex::Regex;
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use strum::{AsRefStr, EnumString};

use super::types::{Channel, Log, Meta, Parseable, Value};

/// Haltech channel types - comprehensive list from actual log files
#[derive(AsRefStr, Clone, Debug, EnumString, Serialize, Default)]
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
    DrivenDistance,
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
    #[default]
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

/// Haltech log file metadata
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

/// Haltech channel definition
#[derive(Clone, Debug, Default, Serialize)]
pub struct HaltechChannel {
    pub name: String,
    pub id: String,
    pub r#type: ChannelType,
    pub display_min: Option<f64>,
    pub display_max: Option<f64>,
}

/// Haltech log file parser
pub struct Haltech;

impl Haltech {
    /// Parse timestamp from HH:MM:SS.mmm format to seconds
    fn parse_timestamp(timestamp: &str) -> Option<f64> {
        // Format: "HH:MM:SS.mmm" e.g., "14:15:46.000"
        let parts: Vec<&str> = timestamp.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let hours: f64 = parts[0].parse().ok()?;
        let minutes: f64 = parts[1].parse().ok()?;

        // Seconds may include milliseconds
        let seconds: f64 = parts[2].parse().ok()?;

        Some(hours * 3600.0 + minutes * 60.0 + seconds)
    }

    /// Check if a line looks like a data row (starts with timestamp)
    fn is_data_row(line: &str) -> bool {
        // Data rows start with HH:MM:SS pattern
        let timestamp_regex = Regex::new(r"^\d{1,2}:\d{2}:\d{2}").unwrap();
        timestamp_regex.is_match(line)
    }
}

impl Parseable for Haltech {
    fn parse(&self, file_contents: &str) -> Result<Log, Box<dyn Error>> {
        let mut meta = HaltechMeta::default();
        let mut channels: Vec<Channel> = vec![];
        let mut times: Vec<String> = vec![];
        let mut data: Vec<Vec<Value>> = vec![];

        // Regex for key-value pairs like "Key : Value"
        let kv_regex =
            Regex::new(r"^(?<name>[^:]+?)\s*:\s*(?<value>.+)$").expect("Failed to compile regex");

        let mut current_channel = HaltechChannel::default();
        let mut in_data_section = false;
        let mut first_timestamp: Option<f64> = None;

        for line in file_contents.lines() {
            let line = line.trim();

            // Skip empty lines and header marker
            if line.is_empty() || line == "%DataLog%" {
                continue;
            }

            // Check if this is a data row
            if Self::is_data_row(line) {
                in_data_section = true;

                // Push any pending channel before processing data
                if !current_channel.name.is_empty() {
                    channels.push(Channel::Haltech(current_channel));
                    current_channel = HaltechChannel::default();
                }

                // Parse CSV data row
                let parts: Vec<&str> = line.split(',').collect();
                if parts.is_empty() {
                    continue;
                }

                // First column is timestamp
                let timestamp_str = parts[0].trim();
                if let Some(timestamp_secs) = Self::parse_timestamp(timestamp_str) {
                    // Store relative time from first timestamp
                    let relative_time = if let Some(first) = first_timestamp {
                        timestamp_secs - first
                    } else {
                        first_timestamp = Some(timestamp_secs);
                        0.0
                    };
                    times.push(format!("{:.3}", relative_time));

                    // Parse remaining values
                    let values: Vec<Value> = parts[1..]
                        .iter()
                        .filter_map(|v| {
                            let v = v.trim();
                            // Try parsing as i64 first, then f64
                            if let Ok(i) = v.parse::<i64>() {
                                Some(Value::Int(i))
                            } else if let Ok(f) = v.parse::<f64>() {
                                Some(Value::Float(f))
                            } else {
                                None
                            }
                        })
                        .collect();

                    // Only add if we have values matching channel count
                    if !values.is_empty() {
                        data.push(values);
                    }
                }
                continue;
            }

            // Not in data section yet - parse metadata and channel definitions
            if !in_data_section {
                if let Some(captures) = kv_regex.captures(line) {
                    let name = captures["name"].trim();
                    let value = captures["value"].trim().to_string();

                    match name {
                        "DataLogVersion" => meta.data_log_version = value,
                        "Software" => meta.software = value,
                        "SoftwareVersion" => meta.software_version = value,
                        "DownloadDateTime" | "DownloadDate/Time" => {
                            meta.download_date_time = value
                        }
                        "Log Source" => meta.log_source = value,
                        "Log Number" => meta.log_number = value,
                        "Log" => meta.log_date_time = value,
                        // "Channel" key indicates start of a new channel definition
                        "Channel" => {
                            if !current_channel.name.is_empty() {
                                channels.push(Channel::Haltech(current_channel));
                            }
                            current_channel = HaltechChannel::default();
                            current_channel.name = value;
                        }
                        "ID" => current_channel.id = value,
                        "Type" => {
                            if let Ok(channel_type) = ChannelType::from_str(&value) {
                                current_channel.r#type = channel_type;
                            } else {
                                tracing::warn!("Unknown channel type: {}", value);
                                current_channel.r#type = ChannelType::Raw;
                            }
                        }
                        "DisplayMaxMin" => {
                            let values: Vec<&str> = value.split(',').collect();
                            if values.len() >= 2 {
                                current_channel.display_max = values[0].trim().parse().ok();
                                current_channel.display_min = values[1].trim().parse().ok();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Verify data integrity
        let channel_count = channels.len();
        if channel_count > 0 {
            // Filter out data rows that don't match channel count
            data.retain(|row| row.len() >= channel_count);
        }

        tracing::info!(
            "Parsed Haltech log: {} channels, {} data points",
            channels.len(),
            data.len()
        );

        Ok(Log {
            meta: Meta::Haltech(meta),
            channels,
            times,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(Haltech::parse_timestamp("00:00:00.000"), Some(0.0));
        assert_eq!(Haltech::parse_timestamp("00:01:00.000"), Some(60.0));
        assert_eq!(Haltech::parse_timestamp("01:00:00.000"), Some(3600.0));
        assert_eq!(Haltech::parse_timestamp("14:15:46.000"), Some(51346.0));
        assert_eq!(Haltech::parse_timestamp("14:15:46.500"), Some(51346.5));
    }

    #[test]
    fn test_parse_haltech_log() {
        let sample = r#"%DataLog%
DataLogVersion : 1.1
Software : Haltech NSP
SoftwareVersion : 999.999.999.999
DownloadDateTime : 20250718 04:09:48
Channel : RPM
ID : 384
Type : EngineSpeed
DisplayMaxMin : 20000,0
Channel : Manifold Pressure
ID : 224
Type : Pressure
DisplayMaxMin : 4013,13
Log Source : 20
Log Number : 1118
Log : 20250718 02:15:46
14:15:46.000,5000,1013
14:15:46.020,5100,1020
14:15:46.040,5200,1030
"#;

        let parser = Haltech;
        let log = parser.parse(sample).unwrap();

        assert_eq!(log.channels.len(), 2);
        assert_eq!(log.channels[0].name(), "RPM");
        assert_eq!(log.channels[1].name(), "Manifold Pressure");
        assert_eq!(log.times.len(), 3);
        assert_eq!(log.data.len(), 3);

        // Check relative timestamps
        assert_eq!(log.times[0], "0.000");
        assert_eq!(log.times[1], "0.020");
        assert_eq!(log.times[2], "0.040");
    }

    #[test]
    fn test_is_data_row() {
        assert!(Haltech::is_data_row("14:15:46.000,5000,1013"));
        assert!(Haltech::is_data_row("0:00:00.000,100,200"));
        assert!(!Haltech::is_data_row("Channel : RPM"));
        assert!(!Haltech::is_data_row("ID : 384"));
        assert!(!Haltech::is_data_row("%DataLog%"));
    }
}
