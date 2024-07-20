// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

mod parsers;

use parsers::types::{Channel, ChannelValue, Log, Parseable};

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![add_file, get_channel_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default)]
struct UserData {
  pub logs: Vec<Log>,
}

#[tauri::command]
fn add_file(file_path: String) -> Result<Vec<Channel>, String> {
  let contents = match fs::read_to_string(&file_path) {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Error reading file: {}", e);
      return Err("Error reading file".to_string());
    }
  };

  // Parse with haltech
  let haltech = parsers::haltech::Haltech {};
  match haltech.parse(&contents) {
    Ok(log) => {
      let mut user_data = UserData::default();
      user_data.logs.push(log.clone());
      Ok(log.channels)
    },
    Err(e) => {
      eprintln!("Error parsing Haltech file: {}", e);
      Err("Error parsing file".to_string())
    }
  }
}

#[tauri::command]
fn get_channel_data(_channel_name: String) -> Result<Vec<ChannelValue>, String> {
  Ok(vec![])
}