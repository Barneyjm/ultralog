# Development

This guide covers setting up a development environment, building from source, and contributing to UltraLog.

## Table of Contents

- [Development Setup](#development-setup)
- [Building](#building)
- [Project Structure](#project-structure)
- [Architecture Overview](#architecture-overview)
- [Adding New ECU Support](#adding-new-ecu-support)
- [Code Style](#code-style)
- [Testing](#testing)
- [CI/CD](#cicd)
- [Contributing](#contributing)

---

## Development Setup

### Prerequisites

- **Rust** - Latest stable version
- **Git** - For version control
- **Platform build tools** - See below

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Platform-Specific Dependencies

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    libgtk-3-dev \
    libglib2.0-dev \
    libatk1.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libgdk-pixbuf2.0-dev
```

**Linux (Fedora):**
```bash
sudo dnf install -y \
    gcc \
    libxcb-devel \
    libxkbcommon-devel \
    openssl-devel \
    gtk3-devel \
    glib2-devel \
    atk-devel \
    cairo-devel \
    pango-devel \
    gdk-pixbuf2-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Select "Desktop development with C++"

### Clone the Repository

```bash
git clone https://github.com/SomethingNew71/UltraLog.git
cd UltraLog
```

---

## Building

### Development Build

Faster compile time, slower runtime, includes debug symbols:

```bash
cargo build
```

### Release Build

Slower compile time, optimized runtime:

```bash
cargo build --release
```

### Run the Application

```bash
# Debug mode
cargo run

# Release mode
cargo run --release
```

### Run the Parser Test Utility

```bash
cargo run --bin test_parser -- path/to/logfile.csv
```

---

## Project Structure

```
UltraLog/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library module exports
│   ├── app.rs               # Main UltraLogApp struct (934 lines)
│   │                        # - Application state management
│   │                        # - File loading pipeline
│   │                        # - eframe::App implementation
│   ├── state.rs             # Core data types (206 lines)
│   │                        # - LoadedFile, SelectedChannel
│   │                        # - Color palettes
│   │                        # - Loading states
│   ├── units.rs             # Unit conversion system (278 lines)
│   ├── normalize.rs         # Field name normalization (500 lines)
│   ├── parsers/
│   │   ├── mod.rs           # Parser module exports
│   │   ├── types.rs         # Core parser traits & enums
│   │   │                    # - Log, Channel, Value, Meta
│   │   │                    # - Parseable trait
│   │   │                    # - EcuType enumeration
│   │   ├── haltech.rs       # Haltech CSV parser (547 lines)
│   │   ├── ecumaster.rs     # ECUMaster CSV parser (366 lines)
│   │   └── speeduino.rs     # Speeduino MLG parser (542 lines)
│   ├── ui/
│   │   ├── mod.rs           # UI module exports
│   │   ├── sidebar.rs       # File list & view options
│   │   ├── channels.rs      # Channel selection panel
│   │   ├── chart.rs         # Main chart & LTTB algorithm
│   │   ├── timeline.rs      # Playback controls
│   │   ├── menu.rs          # Menu bar & units UI
│   │   ├── tab_bar.rs       # Chrome-style tabs
│   │   ├── toast.rs         # Notification system
│   │   ├── icons.rs         # Icon drawing utilities
│   │   ├── export.rs        # PNG/PDF export
│   │   ├── normalization_editor.rs  # Custom field mapping UI
│   │   ├── scatter_plot.rs  # XY scatter visualization
│   │   └── tool_switcher.rs # Tool selection UI
│   └── bin/
│       └── test_parser.rs   # CLI parser testing utility
├── assets/
│   ├── icons/               # Platform-specific app icons
│   ├── Outfit-Regular.ttf   # Custom font
│   └── Outfit-Bold.ttf      # Custom font (bold)
├── exampleLogs/             # Test data
├── .github/
│   └── workflows/
│       ├── ci.yml           # CI pipeline
│       └── release.yml      # Release automation
├── build.rs                 # Windows icon embedding
├── Cargo.toml               # Project manifest
└── README.md                # Documentation
```

### Lines of Code

- **Core app:** 934 lines
- **UI modules:** ~3,148 lines
- **Parsers:** ~1,667 lines
- **Supporting:** ~1,108 lines
- **Total:** ~6,857 lines

---

## Architecture Overview

### Data Flow

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Log File   │───►│    Parser    │───►│  Log Struct │
└─────────────┘    └──────────────┘    └─────────────┘
                                              │
                                              ▼
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   Display   │◄───│  LTTB Down-  │◄───│  Selected   │
│   (egui)    │    │   sample     │    │  Channels   │
└─────────────┘    └──────────────┘    └─────────────┘
```

### Key Components

**UltraLogApp** (`app.rs`)
- Main application state
- Implements `eframe::App` trait
- Manages file loading, tabs, playback

**Parsers** (`parsers/`)
- Each ECU format has its own parser
- All implement the `Parseable` trait
- Convert ECU-specific data to common `Log` struct

**UI Modules** (`ui/`)
- Each UI section is a separate module
- Uses egui's immediate mode paradigm
- Renders from application state each frame

### State Management

```rust
pub struct UltraLogApp {
    // Files and tabs
    files: Vec<LoadedFile>,
    tabs: Vec<Tab>,
    active_tab: Option<usize>,

    // Playback
    cursor_time: Option<f64>,
    is_playing: bool,
    playback_speed: f64,

    // Settings
    unit_preferences: UnitPreferences,
    color_blind_mode: bool,
    field_normalization: bool,

    // UI state
    loading_state: LoadingState,
    downsample_cache: HashMap<...>,
}
```

---

## Adding New ECU Support

### Step 1: Add ECU Type

In `src/parsers/types.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum EcuType {
    Haltech,
    EcuMaster,
    Speeduino,
    NewEcu,  // Add your ECU type
}
```

### Step 2: Create Parser Module

Create `src/parsers/newecu.rs`:

```rust
use super::types::{Channel, Log, Meta, Parseable, Value};
use anyhow::Result;

pub struct NewEcuParser;

impl Parseable for NewEcuParser {
    fn parse(&self, data: &str) -> Result<Log> {
        // Parse your ECU format here

        let channels = vec![
            Channel {
                name: "RPM".to_string(),
                unit: Some("rpm".to_string()),
                ..Default::default()
            },
            // ... more channels
        ];

        let data = vec![
            vec![Value::Float(3500.0), /* ... */],
            // ... more records
        ];

        let times = vec!["0.000".to_string(), /* ... */];

        Ok(Log {
            meta: Meta::default(),
            channels,
            times,
            data,
        })
    }
}
```

### Step 3: Register Parser

In `src/parsers/mod.rs`:

```rust
pub mod newecu;
pub use newecu::NewEcuParser;
```

### Step 4: Add Format Detection

In `src/app.rs`, add detection logic in the file loading function:

```rust
fn detect_format(content: &str) -> Option<EcuType> {
    if content.starts_with("NewECU Header") {
        return Some(EcuType::NewEcu);
    }
    // ... existing detection
}
```

### Step 5: Add to Format Handler

```rust
match ecu_type {
    EcuType::NewEcu => NewEcuParser.parse(&content),
    // ... existing handlers
}
```

### Step 6: Add Field Normalizations

In `src/normalize.rs`, add mappings for your ECU's channel names:

```rust
("newecu_rpm", "Engine RPM"),
("newecu_map", "Manifold Pressure"),
// ... more mappings
```

---

## Code Style

### Formatting

Use rustfmt for consistent formatting:

```bash
cargo fmt
```

### Linting

Use clippy with warnings as errors:

```bash
cargo clippy -- -D warnings
```

### Naming Conventions

- `render_*` - UI rendering methods
- `start_*` - Async operations
- `get_*/set_*` - Accessors
- `SCREAMING_SNAKE_CASE` - Constants
- `PascalCase` - Types and traits
- `snake_case` - Functions and variables

### Error Handling

- Use `anyhow` for parser errors
- Use `thiserror` for custom error types
- Show user-friendly errors via toast notifications

---

## Testing

### Run Tests

```bash
cargo test
```

### Test Parser

```bash
cargo run --bin test_parser -- path/to/logfile.csv
```

### Example Log Files

The `exampleLogs/` directory contains sample files:

- `haltech/` - Haltech CSV samples
- `ecumaster/` - ECUMaster CSV samples
- `speeduino/` - Speeduino MLG samples
- `rusefi/` - rusEFI MLG samples

---

## CI/CD

### CI Pipeline (ci.yml)

Runs on every push and pull request:

1. **Check** - Compile check on all platforms
2. **Test** - Run test suite
3. **Clippy** - Lint for issues
4. **Format** - Verify code formatting

### Release Pipeline (release.yml)

Triggered by version tags (`v*`):

1. **Build** - Create binaries for all platforms
2. **Release** - Create GitHub release with binaries

### Creating a Release

```bash
# Update version in Cargo.toml
# Commit changes
git add .
git commit -m "Release v1.0.0"

# Create and push tag
git tag v1.0.0
git push origin v1.0.0
```

---

## Contributing

### How to Contribute

1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Submit** a pull request

### Pull Request Guidelines

- Keep PRs focused on a single change
- Update documentation if needed
- Add tests for new functionality
- Ensure CI passes
- Write clear commit messages

### Commit Message Format

```
type: short description

Longer description if needed.

Fixes #123
```

Types:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `refactor` - Code refactoring
- `test` - Tests
- `chore` - Maintenance

### Areas for Contribution

- **New ECU formats** - Add support for more ECUs
- **Features** - Implement roadmap items
- **Bug fixes** - Fix reported issues
- **Documentation** - Improve wiki and comments
- **Performance** - Optimize parsing or rendering
- **Accessibility** - Improve accessibility features

---

## Roadmap

### Planned Features

- [ ] MegaSquirt format support
- [ ] AEM format support
- [ ] MaxxECU format support
- [ ] MoTeC format support
- [ ] Link ECU format support
- [ ] Data export to CSV/Excel
- [ ] Advanced statistical analysis
- [ ] Data filtering and smoothing
- [ ] Custom formula channels
- [ ] Persistent settings storage
- [ ] Multi-log comparison tools

### Feature Requests

Open an issue on GitHub to request features.

---

## Resources

### Dependencies Documentation

- [egui](https://docs.rs/egui) - GUI framework
- [eframe](https://docs.rs/eframe) - egui framework
- [egui_plot](https://docs.rs/egui_plot) - Plotting
- [serde](https://docs.rs/serde) - Serialization
- [anyhow](https://docs.rs/anyhow) - Error handling

### Related Links

- [GitHub Repository](https://github.com/SomethingNew71/UltraLog)
- [GitHub Issues](https://github.com/SomethingNew71/UltraLog/issues)
- [Releases](https://github.com/SomethingNew71/UltraLog/releases)

---

## Next Steps

- [[Getting-Started]] - User introduction
- [[Supported-ECU-Formats]] - Current ECU support
- [[Troubleshooting]] - Common issues
