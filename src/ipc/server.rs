//! TCP server for receiving IPC commands from the MCP server
//!
//! This server runs in a background thread and communicates with the GUI
//! via channels, allowing the main eframe event loop to process commands.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use super::commands::{IpcCommand, IpcResponse};
use super::DEFAULT_IPC_PORT;

/// IPC Server that listens for commands from the MCP server
pub struct IpcServer {
    /// Receiver for incoming commands (polled by the GUI)
    command_rx: Receiver<(IpcCommand, Sender<IpcResponse>)>,
    /// Port the server is listening on
    port: u16,
    /// Whether the server is running
    is_running: bool,
}

impl IpcServer {
    /// Start a new IPC server on the default port
    pub fn start() -> Result<Self, String> {
        Self::start_on_port(DEFAULT_IPC_PORT)
    }

    /// Start a new IPC server on a specific port
    pub fn start_on_port(port: u16) -> Result<Self, String> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .map_err(|e| format!("Failed to bind to port {}: {}", port, e))?;

        // Set non-blocking so we can check for shutdown
        listener
            .set_nonblocking(true)
            .map_err(|e| format!("Failed to set non-blocking: {}", e))?;

        let (command_tx, command_rx) = mpsc::channel();

        // Spawn the listener thread
        thread::spawn(move || {
            Self::listener_loop(listener, command_tx);
        });

        tracing::info!("IPC server started on port {}", port);

        Ok(Self {
            command_rx,
            port,
            is_running: true,
        })
    }

    /// Get the port the server is listening on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Check if there's a pending command and return it
    pub fn poll_command(&self) -> Option<(IpcCommand, Sender<IpcResponse>)> {
        self.command_rx.try_recv().ok()
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Main listener loop (runs in background thread)
    fn listener_loop(listener: TcpListener, command_tx: Sender<(IpcCommand, Sender<IpcResponse>)>) {
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    tracing::info!("MCP client connected from {}", addr);
                    let tx = command_tx.clone();
                    thread::spawn(move || {
                        Self::handle_connection(stream, tx);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No connection available, sleep briefly
                    thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => {
                    tracing::error!("Error accepting connection: {}", e);
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }

    /// Handle a single client connection
    fn handle_connection(
        mut stream: TcpStream,
        command_tx: Sender<(IpcCommand, Sender<IpcResponse>)>,
    ) {
        let peer_addr = stream.peer_addr().ok();

        // Set timeouts
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(30)));
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(10)));

        let reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    tracing::debug!("Connection closed: {}", e);
                    break;
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            // Parse the command
            let command: IpcCommand = match serde_json::from_str(&line) {
                Ok(cmd) => cmd,
                Err(e) => {
                    let response = IpcResponse::error(format!("Invalid command JSON: {}", e));
                    let _ = Self::send_response(&mut stream, &response);
                    continue;
                }
            };

            tracing::debug!("Received command: {:?}", command);

            // Create a channel for the response
            let (response_tx, response_rx) = mpsc::channel();

            // Send the command to the GUI thread
            if command_tx.send((command, response_tx)).is_err() {
                let response = IpcResponse::error("GUI is not responding");
                let _ = Self::send_response(&mut stream, &response);
                break;
            }

            // Wait for the response from the GUI
            let response = match response_rx.recv_timeout(std::time::Duration::from_secs(30)) {
                Ok(resp) => resp,
                Err(_) => IpcResponse::error("Timeout waiting for GUI response"),
            };

            if Self::send_response(&mut stream, &response).is_err() {
                break;
            }
        }

        if let Some(addr) = peer_addr {
            tracing::info!("MCP client disconnected: {}", addr);
        }
    }

    /// Send a response to the client
    fn send_response(stream: &mut TcpStream, response: &IpcResponse) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(response).unwrap_or_else(|_| {
            r#"{"status":"Error","data":{"message":"Failed to serialize response"}}"#.to_string()
        });
        writeln!(stream, "{}", json)?;
        stream.flush()?;
        Ok(())
    }
}
