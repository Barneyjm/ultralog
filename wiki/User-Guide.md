# User Guide

This comprehensive guide covers all features and functionality of UltraLog.

## Table of Contents

- [Interface Overview](#interface-overview)
- [Loading Files](#loading-files)
- [Channel Selection](#channel-selection)
- [Chart Interaction](#chart-interaction)
- [Timeline and Playback](#timeline-and-playback)
- [Unit Preferences](#unit-preferences)
- [Field Normalization](#field-normalization)
- [Exporting](#exporting)
- [Scatter Plot Tool](#scatter-plot-tool)
- [Accessibility Features](#accessibility-features)
- [Keyboard Shortcuts](#keyboard-shortcuts)

---

## Interface Overview

### Main Window Layout

UltraLog's interface consists of these main areas:

| Area | Location | Purpose |
|------|----------|---------|
| Menu Bar | Top | File operations, settings, help |
| Tab Bar | Below menu | Switch between open files |
| Left Sidebar | Left | File list and view options |
| Chart Area | Center | Data visualization |
| Right Sidebar | Right | Channel selection |
| Timeline | Bottom | Playback controls and navigation |

### Menu Bar

**File Menu**
- Open - Load a log file
- Export → PNG - Save chart as image
- Export → PDF - Save chart as PDF document

**Units Menu**
- Temperature, Pressure, Speed, Distance, Fuel Economy, Volume, Flow Rate, Acceleration
- Each opens a submenu with available unit options

**View Menu**
- Colorblind Mode - Toggle accessible color palette
- Cursor Tracking - Keep cursor centered during playback
- Field Normalization - Enable/disable standard channel names
- Normalization Editor - Create custom field mappings

**Help Menu**
- About - Version information
- Documentation - Opens this wiki

---

## Loading Files

### Supported File Types

| Extension | Format |
|-----------|--------|
| `.csv` | CSV log files (Haltech, ECUMaster) |
| `.log` | Standard log files |
| `.txt` | Text-based log files |
| `.mlg` | MegaLogViewer binary (Speeduino/rusEFI) |

### Loading Methods

**Method 1: File Dialog**
1. Click **"Select a file"** in the left sidebar, or
2. Use **File → Open** from the menu, or
3. Press `Ctrl/Cmd + O`

**Method 2: Drag and Drop**
- Drag one or more files from your file manager onto the UltraLog window

### Multi-File Support

- Each file opens in its own tab
- Click tabs to switch between files
- Close tabs with the × button on each tab
- Duplicate files are automatically detected and rejected

### File Information

After loading, the left sidebar shows:
- **File name**
- **ECU type** (Haltech, ECUMaster, Speeduino)
- **Channel count**
- **Delete button** to remove the file

---

## Channel Selection

### The Channels Panel

Located in the right sidebar, this panel shows all available data channels from the loaded file.

### Selecting Channels

1. **Search** - Type in the search box to filter channels
2. **Toggle** - Click a channel name to add/remove from chart
3. **Limit** - Maximum of 10 channels can be displayed

### Visual Indicators

| State | Appearance |
|-------|------------|
| Unselected | Gray text |
| Selected | Blue background with color indicator |

### Channel Colors

Selected channels are assigned colors from a palette:
- **Standard:** Blue, Orange, Green, Red, Purple, Brown, Pink, Gray, Yellow, Cyan
- **Colorblind Mode:** Black, Orange, Sky Blue, Bluish Green, Yellow, Blue, Vermillion, Reddish Purple

---

## Chart Interaction

### The Chart Display

The main chart shows:
- **Y-axis:** Normalized values (0-1 scale)
- **X-axis:** Time in seconds
- **Lines:** One colored line per selected channel
- **Cursor:** Vertical line showing current position
- **Legend:** Channel info with current values

### Normalized Display

All channels are displayed normalized to a 0-1 range:
- **Why?** Allows comparing channels with vastly different scales
- **Example:** RPM (0-8000) and Lambda (0.7-1.3) can be compared visually
- **Original values** are shown in the legend with proper units

### Mouse Controls

| Action | Result |
|--------|--------|
| **Click** | Move cursor to clicked position |
| **Scroll** | Zoom in/out on time axis |
| **Drag** | Pan the view left/right |
| **Double-click** | Reset zoom to fit all data |

### Legend Information

For each selected channel, the legend displays:

```
■ Channel Name    Min: 0.00    Max: 100.00    Current: 45.67 units
```

- **Color indicator** - Matches the line on the chart
- **Channel name** - May be normalized if Field Normalization is enabled
- **Min/Max** - Peak values across the entire log
- **Current** - Value at cursor position with unit symbol

---

## Timeline and Playback

### Timeline Controls

Located at the bottom of the window:

```
[▶] [⏸] [⏹]  [1.0x ▼]  ═══════○═══════════  00:15.234 / 02:30.000
```

### Playback Buttons

| Button | Action |
|--------|--------|
| ▶ Play | Start/resume playback |
| ⏸ Pause | Pause at current position |
| ⏹ Stop | Stop and reset to beginning |

### Playback Speed

Available speeds:
- 0.25x (quarter speed)
- 0.5x (half speed)
- 1.0x (real-time)
- 2.0x (double speed)
- 4.0x (4x speed)
- 8.0x (8x speed)

### Timeline Scrubber

- **Click** anywhere on the timeline to jump to that time
- **Drag** the handle to scrub through the data

### Time Display

Shows `current time / total duration` in seconds

### Manual Time Input

Click on the time display to type a specific time in seconds.

### Cursor Tracking Mode

When enabled (View → Cursor Tracking):
- The chart automatically scrolls to keep the cursor centered
- Useful when scrubbing through long logs
- The view window is approximately 30 seconds

---

## Unit Preferences

Access via the **Units** menu. All conversions are applied at display time only - original data is never modified.

### Available Units

| Category | Options | Default |
|----------|---------|---------|
| Temperature | Kelvin, Celsius, Fahrenheit | Celsius |
| Pressure | kPa, PSI, Bar | kPa |
| Speed | km/h, mph | km/h |
| Distance | km, miles | km |
| Fuel Economy | L/100km, MPG | L/100km |
| Volume | Liters, Gallons | Liters |
| Flow Rate | L/min, GPM | L/min |
| Acceleration | m/s², g | m/s² |

### How Units Are Applied

1. UltraLog reads raw data from the log file
2. ECU-specific parsing converts to base units
3. Your unit preference converts for display
4. Legend shows values with appropriate unit symbols

---

## Field Normalization

Field normalization maps ECU-specific channel names to standardized names.

### Purpose

Different ECU systems use different names for the same data:
- Haltech: "Engine Speed"
- ECUMaster: "engine/rpm"
- Speeduino: "RPM"

Field normalization shows them all as "Engine RPM" for consistency.

### Enable/Disable

**View → Field Normalization** toggles this feature.

### Built-in Mappings

UltraLog includes 100+ built-in mappings. Examples:

| ECU Names | Normalized Name |
|-----------|-----------------|
| Act_AFR, AFR1, Aft, Lambda | AFR |
| MAP, Boost_Press, engine/map | Manifold Pressure |
| RPM, Engine_Speed, engine/rpm | Engine RPM |
| TPS, Throttle_Pos, Throttle | Throttle Position |

### Custom Mappings

Create your own mappings via **View → Normalization Editor**:

1. Open the Normalization Editor
2. Enter the **source name** (ECU-specific)
3. Enter the **target name** (standardized)
4. Click Add
5. Mappings apply immediately

**Note:** Custom mappings are stored in memory and reset when the application closes.

---

## Exporting

### PNG Export

Save the current chart view as a PNG image:

1. **File → Export → PNG** (or `Ctrl/Cmd + E`)
2. Choose save location
3. Enter filename
4. Click Save

The exported image includes:
- Current chart view (with zoom/pan state)
- Legend with channel information
- Time axis labels

### PDF Export

Generate a PDF document of your chart:

1. **File → Export → PDF**
2. Choose save location
3. Enter filename
4. Click Save

---

## Scatter Plot Tool

The scatter plot tool visualizes the relationship between two channels.

### Accessing the Tool

1. Click the tool switcher in the interface
2. Select **"Scatter Plot"**

### Using Scatter Plot

1. Select the **X-axis channel** from the dropdown
2. Select the **Y-axis channel** from the dropdown
3. Data points are plotted showing the correlation

### Use Cases

- **AFR vs MAP** - See how AFR changes with manifold pressure
- **Ignition vs RPM** - Visualize timing curve
- **TPS vs Load** - Check throttle-to-load relationship
- **Coolant Temp vs AFR** - Check cold-start enrichment

### Interpreting Results

- **Linear pattern** - Direct correlation between channels
- **Scattered points** - No strong relationship
- **Clusters** - Distinct operating conditions
- **Outliers** - Potential tuning issues or sensor problems

---

## Accessibility Features

### Colorblind Mode

Enable via **View → Colorblind Mode**

Uses Wong's optimized palette designed for:
- Deuteranopia (red-green colorblindness)
- Protanopia (red colorblindness)
- Tritanopia (blue-yellow colorblindness)

**Colorblind Palette Colors:**
1. Black
2. Orange
3. Sky Blue
4. Bluish Green
5. Yellow
6. Blue
7. Vermillion
8. Reddish Purple

### Custom Font

UltraLog uses the **Outfit** typeface for:
- Clear readability
- Consistent appearance across platforms
- Good distinction between similar characters

### Toast Notifications

Non-intrusive notifications appear for:
- File load success/failure
- Export completion
- Errors and warnings

---

## Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + O` | Open file |
| `Ctrl/Cmd + W` | Close current tab |
| `Ctrl/Cmd + E` | Export PNG |

### Playback Shortcuts

| Shortcut | Action |
|----------|--------|
| `Space` | Play/Pause toggle |
| `Escape` | Stop playback |

---

## Tips and Best Practices

### Performance

- UltraLog handles large files well, but loading may take a few seconds for 50MB+ files
- Use release builds for best performance
- Close unused tabs to free memory

### Analysis Workflow

1. **Start wide** - Load the full log and play through at 1x
2. **Identify areas of interest** - Note any anomalies or events
3. **Zoom in** - Use scroll to zoom on specific areas
4. **Compare channels** - Add related channels to identify correlations
5. **Document** - Export charts for records or sharing

### Channel Selection

- Start with essential channels: RPM, AFR, MAP, TPS
- Add diagnostic channels as needed
- Remove channels you're not currently analyzing
- Use search to quickly find specific channels

---

## Next Steps

- [[Supported-ECU-Formats]] - Detailed ECU format information
- [[Unit-Conversion]] - Complete unit reference
- [[Troubleshooting]] - Common issues and solutions
