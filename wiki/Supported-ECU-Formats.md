# Supported ECU Formats

UltraLog supports multiple ECU systems. This page details each format's capabilities and requirements.

## Quick Compatibility Reference

| ECU System | Status | File Type | Export Required |
|------------|--------|-----------|-----------------|
| Haltech | Full Support | `.csv` | Yes (from NSP) |
| ECUMaster EMU Pro | Full Support | `.csv` | Yes (from EMU Pro) |
| Speeduino | Full Support | `.mlg` | No (native format) |
| rusEFI | Full Support | `.mlg` | No (native format) |
| MegaSquirt | Coming Soon | - | - |
| AEM | Coming Soon | - | - |
| MaxxECU | Coming Soon | - | - |
| MoTeC | Coming Soon | - | - |
| Link ECU | Coming Soon | - | - |

---

## Haltech

### Overview

Full support for Haltech ECU log files exported from NSP (Nexus Software Platform).

### Supported File Format

- **Type:** CSV text file
- **Extension:** `.csv`
- **Source:** Exported from Haltech NSP software
- **Identifier:** File begins with `%DataLog%` header

### How to Export from NSP

1. Open your datalog in Haltech NSP
2. Go to **File → Export → CSV**
3. Select all channels you want to include
4. Choose save location and filename
5. Click Export

### Supported Channel Types

UltraLog supports 50+ Haltech channel types with automatic unit conversion:

**Engine Parameters**
- Engine Speed (RPM)
- Manifold Pressure (MAP)
- Throttle Position (TPS)
- Engine Load

**Fuel System**
- Air/Fuel Ratio (AFR)
- Lambda
- Fuel Pressure
- Injector Duty Cycle
- Fuel Trim (Short-term, Long-term)

**Ignition System**
- Ignition Timing
- Knock Levels
- Dwell Time

**Temperatures**
- Coolant Temperature
- Intake Air Temperature
- Oil Temperature
- Exhaust Gas Temperature (EGT)

**Pressures**
- Boost Pressure
- Oil Pressure
- Fuel Pressure
- Barometric Pressure

**Vehicle Data**
- Vehicle Speed
- Gear Position
- Battery Voltage

**And many more...**

### Unit Handling

Haltech logs store data in specific units based on the CAN protocol. UltraLog:
- Automatically detects the channel type
- Applies appropriate conversions
- Displays in your preferred units via Unit Preferences

### Min/Max Values

Haltech logs include min/max metadata for each channel, which UltraLog uses for the legend display.

---

## ECUMaster EMU Pro

### Overview

Full support for ECUMaster EMU Pro log files exported as CSV.

### Supported File Format

- **Type:** CSV text file (semicolon or tab-delimited)
- **Extension:** `.csv`
- **Source:** Exported from EMU Pro software
- **Note:** Native `.emuprolog` binary format is NOT supported

### How to Export from EMU Pro

1. Open your log in EMU Pro software
2. Go to **File → Export → CSV**
3. Select channels to export
4. Choose delimiter (semicolon or tab both work)
5. Save the file

### Channel Path Structure

ECUMaster uses hierarchical channel paths:

```
engine/rpm
engine/map
engine/coolant_temp
fuel/lambda
fuel/injector_pw
ignition/timing
```

UltraLog parses these paths and can normalize them to standard names.

### Automatic Unit Inference

UltraLog infers units from channel naming patterns:

| Pattern | Inferred Unit |
|---------|---------------|
| `*temp*` | °C |
| `*rpm*` | RPM |
| `*map*`, `*press*` | kPa |
| `*lambda*` | Lambda |
| `*percent*`, `*duty*` | % |

### Supported Channels

**Engine**
- RPM
- MAP
- TPS
- Load

**Fuel**
- Lambda
- AFR
- Injector Pulse Width
- Fuel Pressure

**Ignition**
- Timing
- Dwell

**Temperatures**
- Coolant
- Intake Air
- Oil

**And more depending on your EMU Pro configuration**

### Limitations

- Native `.emuprolog` binary format is not currently supported
- Always export to CSV from EMU Pro software

---

## Speeduino / rusEFI

### Overview

Full support for Speeduino and rusEFI logs in MegaLogViewer (MLG) binary format.

### Supported File Format

- **Type:** Binary file
- **Extension:** `.mlg`
- **Source:** Logged directly by Speeduino/rusEFI
- **Identifier:** File begins with `MLVLG` header

### No Export Required

Unlike Haltech and ECUMaster, Speeduino and rusEFI log directly to the MLG format. Simply load the `.mlg` file directly into UltraLog.

### Binary Field Types

UltraLog supports all MLG field types:

| Type | Description |
|------|-------------|
| U08 | Unsigned 8-bit integer |
| S08 | Signed 8-bit integer |
| U16 | Unsigned 16-bit integer |
| S16 | Signed 16-bit integer |
| U32 | Unsigned 32-bit integer |
| S32 | Signed 32-bit integer |
| S64 | Signed 64-bit integer |
| F32 | 32-bit floating point |

### Supported Channels

Depends on your Speeduino/rusEFI configuration. Common channels include:

**Engine**
- RPM
- MAP
- TPS
- VE (Volumetric Efficiency)

**Fuel**
- AFR / Lambda
- Pulse Width
- Fuel Pressure

**Ignition**
- Advance
- Dwell

**Temperatures**
- Coolant
- IAT

**Sensors**
- O2 Sensor
- Battery Voltage
- Various analog inputs

### Timestamp Handling

MLG files include precise timestamps for each record, which UltraLog uses for:
- Time axis display
- Playback synchronization
- Cursor position tracking

---

## Format Auto-Detection

UltraLog automatically detects the ECU format based on file contents:

1. **Haltech:** Looks for `%DataLog%` header
2. **ECUMaster:** Identifies semicolon/tab-delimited CSV with hierarchical paths
3. **Speeduino/rusEFI:** Identifies `MLVLG` binary header

You don't need to specify the format - just load the file.

---

## Coming Soon

The following ECU formats are planned for future releases:

### MegaSquirt

- MS1, MS2, MS3 support
- MSQ and MLG formats

### AEM

- AEM Infinity
- AEM EMS

### MaxxECU

- MaxxECU log files

### MoTeC

- MoTeC M1 series
- MoTeC i2 exports

### Link ECU

- Link G4/G4+ series
- PCLink exports

---

## Requesting New Format Support

If your ECU system isn't supported:

1. Open an issue on [GitHub](https://github.com/SomethingNew71/UltraLog/issues)
2. Include:
   - ECU system name and model
   - Software version used for logging/export
   - A sample log file (if possible)
   - Documentation links (if available)

---

## Next Steps

- [[Getting-Started]] - Start using UltraLog
- [[User-Guide]] - Complete feature reference
- [[Unit-Conversion]] - Configure display units
