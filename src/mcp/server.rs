//! MCP Server implementation for UltraLog
//!
//! This module implements the MCP protocol server that allows Claude to
//! interact with UltraLog through the Model Context Protocol.
//!
//! The server runs as an HTTP service on a configurable port (default 52385)
//! and Claude Desktop can connect to it at `http://localhost:52385/mcp`

use axum::Router;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, ErrorCode, ErrorData as McpError, Implementation, ProtocolVersion,
    ServerCapabilities, ServerInfo,
};
use rmcp::schemars::JsonSchema;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::StreamableHttpService;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::sync::oneshot;

use super::client::GuiClient;
use crate::ipc::commands::{IpcCommand, IpcResponse, ResponseData};
use crate::ipc::DEFAULT_IPC_PORT;

/// Default port for the MCP HTTP server
/// Port 52453 = 5-2-4-5-3, a nod to the 1-2-4-5-3 firing order of legendary inline-5 engines
/// (Audi Quattro, RS3, Volvo 5-cylinder, etc.) with a leading 5 to stay in the dynamic port range
pub const DEFAULT_MCP_PORT: u16 = 52453;

/// Handle to control the running MCP server
pub struct McpServerHandle {
    shutdown_tx: Option<oneshot::Sender<()>>,
    port: u16,
}

impl McpServerHandle {
    /// Get the port the server is running on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the URL for Claude Desktop configuration
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/mcp", self.port)
    }

    /// Signal the server to shut down
    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for McpServerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Start the MCP HTTP server in a background thread
///
/// Returns a handle that can be used to get the server URL and shut it down.
pub fn start_mcp_server(mcp_port: u16, ipc_port: u16) -> Result<McpServerHandle, String> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");

        rt.block_on(async move {
            if let Err(e) = run_mcp_http_server(mcp_port, ipc_port, shutdown_rx).await {
                tracing::error!("MCP server error: {}", e);
            }
        });
    });

    Ok(McpServerHandle {
        shutdown_tx: Some(shutdown_tx),
        port: mcp_port,
    })
}

/// Run the MCP HTTP server
async fn run_mcp_http_server(
    mcp_port: u16,
    ipc_port: u16,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create the MCP service that creates new server instances for each session
    let service = StreamableHttpService::new(
        move || Ok(UltraLogMcpServer::with_ipc_port(ipc_port)),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // Create the router with the MCP service at /mcp
    let router = Router::new().nest_service("/mcp", service);

    // Bind to the port
    let addr = format!("127.0.0.1:{}", mcp_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!(
        "MCP HTTP server started at http://{}/mcp",
        listener.local_addr()?
    );

    // Run the server with graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
            tracing::info!("MCP HTTP server shutting down");
        })
        .await?;

    Ok(())
}

/// UltraLog MCP Server
#[derive(Clone)]
pub struct UltraLogMcpServer {
    client: Arc<GuiClient>,
    tool_router: ToolRouter<UltraLogMcpServer>,
}

impl UltraLogMcpServer {
    pub fn new() -> Self {
        Self::with_ipc_port(DEFAULT_IPC_PORT)
    }

    pub fn with_ipc_port(ipc_port: u16) -> Self {
        Self {
            client: Arc::new(GuiClient::with_port(ipc_port)),
            tool_router: Self::tool_router(),
        }
    }

    fn send_command(&self, command: IpcCommand) -> Result<IpcResponse, String> {
        self.client.send_command(command)
    }

    fn mcp_error(message: impl Into<String>) -> McpError {
        McpError {
            code: ErrorCode(-32603),
            message: Cow::Owned(message.into()),
            data: None,
        }
    }
}

impl Default for UltraLogMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tool Input Types
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadFileRequest {
    #[schemars(description = "Path to the ECU log file to load")]
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FileIdRequest {
    #[schemars(description = "ID of the loaded file (use get_state to see loaded files)")]
    pub file_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChannelRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "Name of the channel")]
    pub channel_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChannelDataRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "Name of the channel")]
    pub channel_name: String,
    #[schemars(description = "Optional start time in seconds")]
    #[serde(default)]
    pub start_time: Option<f64>,
    #[schemars(description = "Optional end time in seconds")]
    #[serde(default)]
    pub end_time: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateComputedChannelRequest {
    #[schemars(description = "Name for the computed channel")]
    pub name: String,
    #[schemars(
        description = "Mathematical formula (e.g., 'RPM * 0.5 + Boost'). Use channel names as variables."
    )]
    pub formula: String,
    #[schemars(description = "Unit for the computed channel (e.g., 'kPa', 'RPM', 'deg')")]
    pub unit: String,
    #[schemars(description = "Optional description")]
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EvaluateFormulaRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "Mathematical formula to evaluate")]
    pub formula: String,
    #[schemars(description = "Optional start time in seconds")]
    #[serde(default)]
    pub start_time: Option<f64>,
    #[schemars(description = "Optional end time in seconds")]
    #[serde(default)]
    pub end_time: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimeRangeRequest {
    #[schemars(description = "Start time in seconds")]
    pub start: f64,
    #[schemars(description = "End time in seconds")]
    pub end: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCursorRequest {
    #[schemars(description = "Cursor position in seconds")]
    pub time: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PlayRequest {
    #[schemars(description = "Playback speed multiplier (0.25 to 8.0, default 1.0)")]
    #[serde(default)]
    pub speed: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindPeaksRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "Name of the channel")]
    pub channel_name: String,
    #[schemars(description = "Minimum prominence for peak detection (default 0.1)")]
    #[serde(default)]
    pub min_prominence: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CorrelateChannelsRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "First channel name")]
    pub channel_a: String,
    #[schemars(description = "Second channel name")]
    pub channel_b: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShowScatterPlotRequest {
    #[schemars(description = "ID of the loaded file")]
    pub file_id: String,
    #[schemars(description = "Channel for X axis")]
    pub x_channel: String,
    #[schemars(description = "Channel for Y axis")]
    pub y_channel: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteComputedChannelRequest {
    #[schemars(description = "Name of the computed channel to delete")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyRequest {}

// ============================================================================
// Tool Implementations
// ============================================================================

#[tool_router]
impl UltraLogMcpServer {
    #[tool(
        description = "Get the current state of UltraLog including loaded files, selected channels, cursor position, and view mode."
    )]
    async fn get_state(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::GetState) {
            Ok(IpcResponse::Ok(Some(ResponseData::State(state)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&state).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Load an ECU log file. Supports Haltech CSV, ECUMaster CSV, RomRaider CSV, Speeduino/rusEFI MLG, AiM XRK/DRK, and Link LLG formats."
    )]
    async fn load_file(
        &self,
        Parameters(req): Parameters<LoadFileRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::LoadFile { path: req.path }) {
            Ok(IpcResponse::Ok(Some(ResponseData::FileLoaded(info)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&info).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Ok(Some(ResponseData::Ack))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    "File is being loaded. Use get_state to check when ready.",
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(description = "Close a loaded file.")]
    async fn close_file(
        &self,
        Parameters(req): Parameters<FileIdRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::CloseFile {
            file_id: req.file_id,
        }) {
            Ok(IpcResponse::Ok(_)) => {
                Ok(CallToolResult::success(vec![Content::text("File closed")]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(
        description = "List all available channels in a loaded file, including computed channels."
    )]
    async fn list_channels(
        &self,
        Parameters(req): Parameters<FileIdRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::ListChannels {
            file_id: req.file_id,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::Channels(channels)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&channels).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Get time series data for a specific channel. Optionally filter by time range."
    )]
    async fn get_channel_data(
        &self,
        Parameters(req): Parameters<ChannelDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let time_range = match (req.start_time, req.end_time) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        };

        match self.send_command(IpcCommand::GetChannelData {
            file_id: req.file_id,
            channel_name: req.channel_name,
            time_range,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::ChannelData { times, values }))) => {
                let result = serde_json::json!({
                    "sample_count": times.len(),
                    "times": times,
                    "values": values
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(description = "Get statistics (min, max, mean, std_dev, median) for a channel.")]
    async fn get_channel_stats(
        &self,
        Parameters(req): Parameters<ChannelDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let time_range = match (req.start_time, req.end_time) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        };

        match self.send_command(IpcCommand::GetChannelStats {
            file_id: req.file_id,
            channel_name: req.channel_name,
            time_range,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::Stats(stats)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&stats).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Add a channel to the chart display. The user will see this channel visualized in the UltraLog GUI."
    )]
    async fn select_channel(
        &self,
        Parameters(req): Parameters<ChannelRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::SelectChannel {
            file_id: req.file_id,
            channel_name: req.channel_name,
        }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Channel selected",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Remove a channel from the chart display.")]
    async fn deselect_channel(
        &self,
        Parameters(req): Parameters<ChannelRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::DeselectChannel {
            file_id: req.file_id,
            channel_name: req.channel_name,
        }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Channel deselected",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Remove all channels from the chart display.")]
    async fn deselect_all_channels(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::DeselectAllChannels) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "All channels deselected",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(
        description = "Create a new computed channel from a mathematical formula. Supports: +, -, *, /, ^, sin, cos, tan, sqrt, abs, ln, log, min, max. Time-shifting: RPM[-1] (previous sample), RPM@-0.1s (100ms ago)."
    )]
    async fn create_computed_channel(
        &self,
        Parameters(req): Parameters<CreateComputedChannelRequest>,
    ) -> Result<CallToolResult, McpError> {
        let name = req.name.clone();
        match self.send_command(IpcCommand::CreateComputedChannel {
            name: req.name,
            formula: req.formula,
            unit: req.unit,
            description: req.description,
        }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Computed channel '{}' created",
                name
            ))])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Delete a computed channel.")]
    async fn delete_computed_channel(
        &self,
        Parameters(req): Parameters<DeleteComputedChannelRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::DeleteComputedChannel { name: req.name }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Computed channel deleted",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "List all saved computed channel templates.")]
    async fn list_computed_channels(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::ListComputedChannels) {
            Ok(IpcResponse::Ok(Some(ResponseData::ComputedChannels(channels)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&channels).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Evaluate a mathematical formula against the log data without creating a permanent channel. Returns the computed values and statistics."
    )]
    async fn evaluate_formula(
        &self,
        Parameters(req): Parameters<EvaluateFormulaRequest>,
    ) -> Result<CallToolResult, McpError> {
        let time_range = match (req.start_time, req.end_time) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        };

        match self.send_command(IpcCommand::EvaluateFormula {
            file_id: req.file_id,
            formula: req.formula,
            time_range,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::FormulaResult {
                times,
                values,
                stats,
            }))) => {
                let result = serde_json::json!({
                    "sample_count": times.len(),
                    "stats": stats,
                    "times": times,
                    "values": values
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Set the visible time range on the chart. Use this to zoom into a specific time window."
    )]
    async fn set_time_range(
        &self,
        Parameters(req): Parameters<SetTimeRangeRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::SetTimeRange {
            start: req.start,
            end: req.end,
        }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Time range set",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(
        description = "Set the cursor position on the timeline. The user will see channel values at this time."
    )]
    async fn set_cursor(
        &self,
        Parameters(req): Parameters<SetCursorRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::SetCursor { time: req.time }) {
            Ok(IpcResponse::Ok(_)) => {
                Ok(CallToolResult::success(vec![Content::text("Cursor set")]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Start playback of the log data. The cursor will move through time.")]
    async fn play(
        &self,
        Parameters(req): Parameters<PlayRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::Play { speed: req.speed }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Playback started",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Pause playback.")]
    async fn pause(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::Pause) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Playback paused",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Stop playback and reset cursor to the start.")]
    async fn stop(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::Stop) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Playback stopped",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Get channel values at the current cursor position.")]
    async fn get_cursor_values(
        &self,
        Parameters(req): Parameters<FileIdRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::GetCursorValues {
            file_id: req.file_id,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::CursorValues(values)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&values).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Find peaks (local maxima) in a channel. Useful for finding acceleration events, boost spikes, etc."
    )]
    async fn find_peaks(
        &self,
        Parameters(req): Parameters<FindPeaksRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::FindPeaks {
            file_id: req.file_id,
            channel_name: req.channel_name,
            min_prominence: req.min_prominence,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::Peaks(peaks)))) => {
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&peaks).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Calculate the correlation between two channels. Returns Pearson correlation coefficient and interpretation."
    )]
    async fn correlate_channels(
        &self,
        Parameters(req): Parameters<CorrelateChannelsRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::CorrelateChannels {
            file_id: req.file_id,
            channel_a: req.channel_a,
            channel_b: req.channel_b,
        }) {
            Ok(IpcResponse::Ok(Some(ResponseData::Correlation {
                coefficient,
                interpretation,
            }))) => {
                let result = serde_json::json!({
                    "coefficient": coefficient,
                    "interpretation": interpretation
                });
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap_or_default(),
                )]))
            }
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
            _ => Err(Self::mcp_error("Unexpected response")),
        }
    }

    #[tool(
        description = "Switch to scatter plot view to visualize correlation between two channels."
    )]
    async fn show_scatter_plot(
        &self,
        Parameters(req): Parameters<ShowScatterPlotRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::ShowScatterPlot {
            file_id: req.file_id,
            x_channel: req.x_channel,
            y_channel: req.y_channel,
        }) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Scatter plot displayed",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }

    #[tool(description = "Switch back to time series chart view.")]
    async fn show_chart(
        &self,
        Parameters(_): Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.send_command(IpcCommand::ShowChart) {
            Ok(IpcResponse::Ok(_)) => Ok(CallToolResult::success(vec![Content::text(
                "Chart view displayed",
            )])),
            Ok(IpcResponse::Error { message }) => Err(Self::mcp_error(message)),
            Err(e) => Err(Self::mcp_error(e)),
        }
    }
}

#[tool_handler]
impl ServerHandler for UltraLogMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "ultralog".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "UltraLog MCP Server - Control the UltraLog ECU log viewer application. \
                Use get_state to see loaded files and current view. \
                Load files, select channels to display, create computed channels, \
                and analyze ECU telemetry data."
                    .to_string(),
            ),
        }
    }
}
