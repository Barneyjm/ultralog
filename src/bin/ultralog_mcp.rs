//! UltraLog MCP Server Binary
//!
//! This is the MCP (Model Context Protocol) server for UltraLog. It allows
//! LLMs like Claude to interact with the UltraLog application through the
//! standardized MCP protocol.
//!
//! # Usage
//!
//! 1. Start the UltraLog GUI application (it will start the IPC server)
//! 2. Configure Claude Desktop or Claude Code to use this MCP server
//! 3. Claude can now control UltraLog through the MCP tools
//!
//! # Configuration
//!
//! Add to your Claude Desktop config (`~/.config/claude-desktop/config.json`):
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "ultralog": {
//!       "command": "/path/to/ultralog-mcp",
//!       "args": []
//!     }
//!   }
//! }
//! ```

use ultralog::mcp::UltraLogMcpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize simple logging to stderr (MCP uses stdio for protocol)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting UltraLog MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Check for port argument
    let port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(ultralog::ipc::DEFAULT_IPC_PORT);

    let server = UltraLogMcpServer::with_port(port);

    tracing::info!("Connecting to UltraLog GUI on port {}", port);

    server.run_stdio().await?;

    Ok(())
}
