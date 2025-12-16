use std::fs;

// Import from the library
use ultralog::parsers::{Haltech, Parseable};

fn main() {
    let path = "exampleLogs/haltech/2025-07-18_0215pm_Log1118.csv";

    println!("Reading file: {}", path);
    let contents = fs::read_to_string(path).expect("Failed to read file");
    println!("File size: {} bytes", contents.len());

    println!("\nParsing Haltech log...");
    let parser = Haltech;
    match parser.parse(&contents) {
        Ok(log) => {
            println!("\n=== Parse Results ===");
            println!("Channels: {}", log.channels.len());
            println!("Data points: {}", log.data.len());
            println!(
                "Time range: {} to {} seconds",
                log.times.first().unwrap_or(&"N/A".to_string()),
                log.times.last().unwrap_or(&"N/A".to_string())
            );

            println!("\n=== First 10 Channels ===");
            for (i, channel) in log.channels.iter().take(10).enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    channel.name(),
                    channel.type_name()
                );
            }

            if log.channels.len() > 10 {
                println!("  ... and {} more channels", log.channels.len() - 10);
            }

            println!("\n=== Sample Data (first 5 rows) ===");
            for (i, (time, row)) in log.times.iter().zip(log.data.iter()).take(5).enumerate() {
                let values: Vec<String> = row
                    .iter()
                    .take(5)
                    .map(|v| format!("{:.0}", v.as_f64()))
                    .collect();
                println!(
                    "  {}: t={:.3}s -> [{}...]",
                    i,
                    time.parse::<f64>().unwrap_or(0.0),
                    values.join(", ")
                );
            }

            println!("\n=== Success! Parser working correctly ===");
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
