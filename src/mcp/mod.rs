//! MCP (Model Context Protocol) server module for UltraLog
//!
//! This module implements an MCP server that allows LLMs like Claude to
//! interact with the UltraLog application, controlling channel visualization,
//! computing derived channels, and analyzing ECU log data.

pub mod client;
pub mod server;

pub use server::UltraLogMcpServer;
