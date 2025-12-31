//! TCP client for communicating with the UltraLog GUI's IPC server

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::Duration;

use crate::ipc::commands::{IpcCommand, IpcResponse};
use crate::ipc::DEFAULT_IPC_PORT;

/// Client for communicating with the UltraLog GUI
pub struct GuiClient {
    stream: Mutex<Option<TcpStream>>,
    port: u16,
}

impl GuiClient {
    /// Create a new GUI client
    pub fn new() -> Self {
        Self {
            stream: Mutex::new(None),
            port: DEFAULT_IPC_PORT,
        }
    }

    /// Create a new GUI client with a specific port
    pub fn with_port(port: u16) -> Self {
        Self {
            stream: Mutex::new(None),
            port,
        }
    }

    /// Connect to the GUI if not already connected
    fn ensure_connected(&self) -> Result<(), String> {
        let mut stream = self.stream.lock().map_err(|e| format!("Lock error: {}", e))?;

        if stream.is_some() {
            return Ok(());
        }

        let addr = format!("127.0.0.1:{}", self.port);
        let new_stream = TcpStream::connect(&addr)
            .map_err(|e| format!("Failed to connect to UltraLog GUI at {}: {}", addr, e))?;

        new_stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        new_stream
            .set_write_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| format!("Failed to set write timeout: {}", e))?;

        *stream = Some(new_stream);
        Ok(())
    }

    /// Send a command to the GUI and get a response
    pub fn send_command(&self, command: IpcCommand) -> Result<IpcResponse, String> {
        self.ensure_connected()?;

        let mut stream_guard = self.stream.lock().map_err(|e| format!("Lock error: {}", e))?;

        // Take the stream out temporarily
        let mut stream = stream_guard
            .take()
            .ok_or_else(|| "Not connected".to_string())?;

        // Serialize and send the command
        let json = serde_json::to_string(&command)
            .map_err(|e| format!("Failed to serialize command: {}", e))?;

        if let Err(e) = writeln!(stream, "{}", json) {
            // Connection lost, don't put it back
            return Err(format!("Failed to send command: {}", e));
        }

        if let Err(e) = stream.flush() {
            return Err(format!("Failed to flush: {}", e));
        }

        // Read the response
        let mut reader = BufReader::new(stream.try_clone().map_err(|e| format!("Clone error: {}", e))?);
        let mut response_line = String::new();

        if let Err(e) = reader.read_line(&mut response_line) {
            return Err(format!("Failed to read response: {}", e));
        }

        // Put the stream back
        *stream_guard = Some(stream);

        // Parse the response
        serde_json::from_str(&response_line)
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Check if the GUI is running and responsive
    pub fn ping(&self) -> bool {
        match self.send_command(IpcCommand::Ping) {
            Ok(IpcResponse::Ok(_)) => true,
            _ => false,
        }
    }

    /// Disconnect from the GUI
    pub fn disconnect(&self) {
        if let Ok(mut stream) = self.stream.lock() {
            *stream = None;
        }
    }
}

impl Default for GuiClient {
    fn default() -> Self {
        Self::new()
    }
}
