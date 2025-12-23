# Getting Started

This guide will walk you through the basics of using UltraLog in 5 minutes.

## Overview

UltraLog's interface is divided into three main areas:

```
┌─────────────────────────────────────────────────────────────────┐
│  File  Units  View  Help                            [Menu Bar]  │
├──────────┬──────────────────────────────────┬───────────────────┤
│          │                                  │                   │
│  FILES   │                                  │    CHANNELS       │
│          │          CHART AREA              │                   │
│  File 1  │                                  │    [Search...]    │
│  File 2  │                                  │    □ RPM          │
│          │                                  │    □ MAP          │
│          │                                  │    □ AFR          │
│          │                                  │    □ TPS          │
├──────────┴──────────────────────────────────┴───────────────────┤
│  [▶] [⏸] [⏹]  [1.0x ▼]  ═══════○═══════════  00:15.234 / 02:30  │
└─────────────────────────────────────────────────────────────────┘
                          [Timeline Controls]
```

## Step 1: Load a Log File

There are two ways to load a log file:

### Option A: File Dialog

1. Click the **"Select a file"** button in the left sidebar
2. Navigate to your log file
3. Click **Open**

### Option B: Drag and Drop

1. Open your file manager
2. Drag a log file onto the UltraLog window
3. Drop it anywhere on the application

**Supported file extensions:** `.csv`, `.log`, `.txt`, `.mlg`

UltraLog automatically detects the ECU format based on file contents.

## Step 2: Select Channels to Display

Once a file is loaded:

1. Look at the **Channels** panel on the right
2. Use the search box to filter channels (e.g., type "rpm" or "afr")
3. Click on a channel name to add it to the chart
4. The channel turns **blue** when selected
5. Click again to remove it from the chart

**Tip:** You can display up to 10 channels simultaneously.

## Step 3: Navigate Through Data

### Using the Timeline

- **Click** anywhere on the timeline scrubber to jump to that time
- **Drag** the scrubber handle to seek through the data

### Using the Chart

- **Click** directly on the chart to move the cursor to that position
- **Scroll** up/down to zoom in/out on the time axis
- **Click and drag** to pan left/right

### Using Playback Controls

| Button | Action |
|--------|--------|
| ▶ Play | Start playing through the data |
| ⏸ Pause | Pause playback |
| ⏹ Stop | Stop and return to beginning |

**Speed selector:** Choose from 0.25x, 0.5x, 1x, 2x, 4x, or 8x playback speed.

## Step 4: Read the Data

### Understanding the Chart

- All channels are **normalized to 0-1 range** for easy comparison
- This means channels with different scales (RPM: 0-8000, AFR: 10-18) can be compared visually

### Reading the Legend

The legend shows three pieces of information for each channel:

```
■ Engine RPM          Min: 850    Max: 7200    Current: 3450 rpm
```

| Element | Description |
|---------|-------------|
| Color square | Identifies the line on the chart |
| Channel name | The data channel |
| Min/Max | Peak values across the entire log |
| Current | Value at the cursor position with units |

## Step 5: Customize Your View

### Change Units

1. Click **Units** in the menu bar
2. Select a category (Temperature, Pressure, Speed, etc.)
3. Choose your preferred unit

### Enable Cursor Tracking

Keep the cursor centered while scrubbing:

1. Click **View** in the menu bar
2. Select **Cursor Tracking**

### Enable Colorblind Mode

For better color distinction:

1. Click **View** in the menu bar
2. Select **Colorblind Mode**

---

## Quick Reference

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + O` | Open file |
| `Ctrl/Cmd + W` | Close current tab |
| `Space` | Play/Pause |
| `Escape` | Stop playback |

### Mouse Controls

| Action | Result |
|--------|--------|
| Click on chart | Move cursor to that position |
| Scroll on chart | Zoom in/out |
| Drag on chart | Pan view |
| Double-click chart | Reset zoom |

---

## Working with Multiple Files

UltraLog supports opening multiple log files:

1. Load additional files using any method above
2. Each file opens in its own **tab**
3. Click tabs to switch between files
4. Each tab maintains its own channel selections

**Note:** The same file cannot be loaded twice.

---

## Example Workflow

Here's a typical workflow for analyzing a dyno run:

1. **Load the log file** from your dyno session
2. **Select key channels:** RPM, AFR, MAP, Ignition Timing
3. **Use playback** at 1x speed to watch the run
4. **Pause** when you see something interesting
5. **Zoom in** on the chart to examine details
6. **Check the legend** for exact values at that point
7. **Export** the chart as PNG for your records

---

## Next Steps

Now that you know the basics:

- [[User-Guide]] - Complete reference for all features
- [[Supported-ECU-Formats]] - Details on each ECU system
- [[Unit-Conversion]] - All available unit options
- [[Field-Normalization]] - Understanding channel name mapping
