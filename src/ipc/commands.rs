//! IPC command and response types for GUI-MCP communication

use serde::{Deserialize, Serialize};

/// Commands that can be sent from the MCP server to the GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcCommand {
    /// Ping to check if the GUI is running
    Ping,

    /// Get the current state of the application
    GetState,

    /// Load a log file
    LoadFile { path: String },

    /// Close a loaded file
    CloseFile { file_id: String },

    /// List all channels in a loaded file
    ListChannels { file_id: String },

    /// Get data for a specific channel
    GetChannelData {
        file_id: String,
        channel_name: String,
        /// Optional time range (start, end) in seconds
        time_range: Option<(f64, f64)>,
    },

    /// Get statistics for a channel
    GetChannelStats {
        file_id: String,
        channel_name: String,
        /// Optional time range for stats calculation
        time_range: Option<(f64, f64)>,
    },

    /// Select a channel to display on the chart
    SelectChannel {
        file_id: String,
        channel_name: String,
    },

    /// Deselect a channel from the chart
    DeselectChannel {
        file_id: String,
        channel_name: String,
    },

    /// Deselect all channels
    DeselectAllChannels,

    /// Create a computed channel
    CreateComputedChannel {
        name: String,
        formula: String,
        unit: String,
        description: Option<String>,
    },

    /// Delete a computed channel
    DeleteComputedChannel { name: String },

    /// List all computed channel templates
    ListComputedChannels,

    /// Evaluate a formula without creating a permanent channel
    EvaluateFormula {
        file_id: String,
        formula: String,
        /// Optional time range
        time_range: Option<(f64, f64)>,
    },

    /// Set the visible time range on the chart
    SetTimeRange { start: f64, end: f64 },

    /// Set the cursor position
    SetCursor { time: f64 },

    /// Start playback
    Play { speed: Option<f64> },

    /// Pause playback
    Pause,

    /// Stop playback and reset cursor
    Stop,

    /// Get values at the current cursor position
    GetCursorValues { file_id: String },

    /// Find peaks in a channel
    FindPeaks {
        file_id: String,
        channel_name: String,
        /// Minimum prominence for peak detection
        min_prominence: Option<f64>,
    },

    /// Correlate two channels
    CorrelateChannels {
        file_id: String,
        channel_a: String,
        channel_b: String,
    },

    /// Switch to scatter plot view
    ShowScatterPlot {
        file_id: String,
        x_channel: String,
        y_channel: String,
    },

    /// Switch back to time series chart view
    ShowChart,
}

/// Responses from the GUI to the MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum IpcResponse {
    /// Successful response with optional data
    Ok(Option<ResponseData>),

    /// Error response
    Error { message: String },
}

/// Data that can be returned in a successful response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ResponseData {
    /// Simple acknowledgment
    Ack,

    /// Pong response
    Pong,

    /// Application state
    State(AppState),

    /// File was loaded successfully
    FileLoaded(FileInfo),

    /// List of channels
    Channels(Vec<ChannelInfo>),

    /// Channel time series data
    ChannelData {
        times: Vec<f64>,
        values: Vec<f64>,
    },

    /// Channel statistics
    Stats(ChannelStats),

    /// Formula evaluation result
    FormulaResult {
        times: Vec<f64>,
        values: Vec<f64>,
        stats: ChannelStats,
    },

    /// Values at cursor position
    CursorValues(Vec<CursorValue>),

    /// List of computed channel templates
    ComputedChannels(Vec<ComputedChannelInfo>),

    /// Peak detection results
    Peaks(Vec<Peak>),

    /// Correlation result
    Correlation {
        coefficient: f64,
        interpretation: String,
    },
}

/// Current application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// List of loaded files
    pub files: Vec<FileInfo>,
    /// Currently active file ID
    pub active_file: Option<String>,
    /// Currently selected channels
    pub selected_channels: Vec<SelectedChannelInfo>,
    /// Current cursor time
    pub cursor_time: Option<f64>,
    /// Visible time range
    pub visible_time_range: Option<(f64, f64)>,
    /// Whether playback is active
    pub is_playing: bool,
    /// Current view mode
    pub view_mode: String,
}

/// Information about a loaded file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Unique identifier for the file
    pub id: String,
    /// File path
    pub path: String,
    /// File name (for display)
    pub name: String,
    /// ECU type detected
    pub ecu_type: String,
    /// Number of channels
    pub channel_count: usize,
    /// Number of data records
    pub record_count: usize,
    /// Total duration in seconds
    pub duration: f64,
    /// Sample rate (records per second)
    pub sample_rate: f64,
}

/// Information about a channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    /// Channel name
    pub name: String,
    /// Channel unit
    pub unit: String,
    /// Channel type/category
    pub channel_type: String,
    /// Whether this is a computed channel
    pub is_computed: bool,
    /// Min value in the data
    pub min_value: Option<f64>,
    /// Max value in the data
    pub max_value: Option<f64>,
}

/// Information about a selected channel on the chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedChannelInfo {
    /// File ID
    pub file_id: String,
    /// Channel name
    pub channel_name: String,
    /// Display color (hex)
    pub color: String,
}

/// Channel statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub median: f64,
    /// Number of samples
    pub count: usize,
    /// Time of minimum value
    pub min_time: f64,
    /// Time of maximum value
    pub max_time: f64,
}

/// Value at cursor position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorValue {
    pub channel_name: String,
    pub value: f64,
    pub unit: String,
}

/// Information about a computed channel template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedChannelInfo {
    pub id: String,
    pub name: String,
    pub formula: String,
    pub unit: String,
    pub description: String,
}

/// A detected peak in the data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peak {
    pub time: f64,
    pub value: f64,
    pub prominence: f64,
}

impl IpcResponse {
    /// Create a simple OK response
    pub fn ok() -> Self {
        Self::Ok(Some(ResponseData::Ack))
    }

    /// Create an OK response with data
    pub fn ok_with_data(data: ResponseData) -> Self {
        Self::Ok(Some(data))
    }

    /// Create an error response
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
        }
    }
}
