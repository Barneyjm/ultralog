# Troubleshooting

This guide helps resolve common issues with UltraLog.

## Table of Contents

- [Installation Issues](#installation-issues)
- [File Loading Issues](#file-loading-issues)
- [Display Issues](#display-issues)
- [Performance Issues](#performance-issues)
- [Export Issues](#export-issues)
- [Platform-Specific Issues](#platform-specific-issues)

---

## Installation Issues

### Windows: SmartScreen Warning

**Problem:** Windows SmartScreen blocks the application with "Windows protected your PC" message.

**Solution:**
1. Click **"More info"**
2. Click **"Run anyway"**

**Why this happens:** The application is not code-signed with an Extended Validation (EV) certificate.

---

### macOS: "Cannot be opened because the developer cannot be verified"

**Problem:** macOS Gatekeeper blocks the application.

**Solutions:**

**Option 1: Right-click Open**
1. Right-click (or Control-click) the application
2. Select **"Open"**
3. Click **"Open"** in the dialog

**Option 2: System Preferences**
1. Try to open the app (it will be blocked)
2. Go to **System Preferences → Security & Privacy → General**
3. Click **"Open Anyway"**

**Option 3: Remove Quarantine**
```bash
xattr -d com.apple.quarantine /path/to/ultralog
```

---

### macOS: "UltraLog is damaged and can't be opened"

**Problem:** Corrupt download or quarantine issues.

**Solutions:**
1. Re-download the file
2. Remove quarantine attribute:
   ```bash
   xattr -cr /path/to/ultralog
   ```

---

### Linux: "Permission denied" when running

**Problem:** The binary doesn't have execute permission.

**Solution:**
```bash
chmod +x ultralog-linux
./ultralog-linux
```

---

### Linux: Missing Libraries

**Problem:** Error about missing shared libraries (libgtk, libcairo, etc.)

**Solution:** Install required dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get install -y \
    libgtk-3-0 \
    libglib2.0-0 \
    libcairo2 \
    libpango-1.0-0 \
    libgdk-pixbuf2.0-0 \
    libatk1.0-0
```

**Fedora:**
```bash
sudo dnf install -y gtk3 glib2 cairo pango gdk-pixbuf2 atk
```

---

## File Loading Issues

### "File format not recognized"

**Problem:** UltraLog cannot identify the ECU format.

**Possible causes and solutions:**

1. **Unsupported ECU system**
   - Check [[Supported-ECU-Formats]] for supported systems
   - Request support via GitHub Issues

2. **Wrong file format**
   - **ECUMaster:** Export to CSV (not `.emuprolog`)
   - **Haltech:** Export from NSP to CSV
   - **Speeduino:** Use `.mlg` files directly

3. **Corrupted file**
   - Try re-exporting from your ECU software
   - Check file size is not 0 bytes

4. **Wrong encoding**
   - Ensure CSV is UTF-8 encoded
   - Avoid special characters in file path

---

### "File already loaded"

**Problem:** UltraLog prevents loading the same file twice.

**Solution:** This is intentional behavior. If you need to reload:
1. Close the existing tab for that file
2. Load the file again

---

### File loads but shows no channels

**Problem:** File loads successfully but channel list is empty.

**Possible causes:**
1. Log file contains no data records
2. Parser couldn't identify channels
3. File is header-only (no actual data)

**Solution:**
- Verify the log file has actual data
- Check if logging was started in the ECU
- Try opening in a text editor to verify contents

---

### Very slow file loading

**Problem:** Large files take a long time to load.

**Normal behavior:**
- Files up to 50MB: A few seconds
- Files 50-100MB: 5-15 seconds
- Files 100MB+: May take longer

**Tips:**
- Loading happens in a background thread - UI remains responsive
- Consider splitting very large logs in your ECU software
- Use release build for best performance

---

## Display Issues

### Channels show wrong units

**Problem:** Values appear incorrect (e.g., temperature showing 363 instead of 90°C).

**Solutions:**
1. Check **Units** menu and select appropriate units
2. Verify your ECU's native units match expected
3. Some channels may not support unit conversion

**Note:** Some ECUs store data in unusual units. Haltech stores temperature in Kelvin, which UltraLog converts automatically.

---

### Chart shows flat lines

**Problem:** All channels appear as flat horizontal lines.

**Possible causes:**
1. **No variation in data** - The logged values didn't change
2. **Very small variations** - Normalized display makes small changes invisible
3. **Wrong time range** - Zoomed to a static portion

**Solutions:**
- Check the Min/Max values in the legend
- Zoom out to see more data
- Verify the ECU was actually logging changing data

---

### Legend shows "NaN" or unusual values

**Problem:** Legend displays NaN (Not a Number) or obviously wrong values.

**Possible causes:**
1. Data contains invalid values from ECU
2. Sensor errors during logging
3. Parser couldn't interpret certain values

**Solutions:**
- Check the raw log file for unusual entries
- Verify sensor connections in your ECU
- Report the issue with a sample file

---

### Colors hard to distinguish

**Problem:** Chart lines look too similar.

**Solutions:**
1. Enable **Colorblind Mode** (View → Colorblind Mode)
2. Select fewer channels (max 10)
3. Use channels that don't overlap

---

## Performance Issues

### Application is slow/laggy

**Problem:** UI feels sluggish, especially with large files.

**Solutions:**

1. **Check build type**
   - Release builds are much faster than debug builds
   - If building from source: `cargo build --release`

2. **Close unused tabs**
   - Each open file uses memory
   - Close files you're not actively analyzing

3. **Select fewer channels**
   - More channels = more rendering work
   - Focus on channels you need

4. **Check system resources**
   - Close other heavy applications
   - Ensure adequate RAM (8GB+ recommended for large files)

---

### High memory usage

**Problem:** UltraLog using excessive RAM.

**Normal memory usage:**
- Empty: ~100MB
- Per file: ~1-2× file size
- Large files may use 500MB+

**Solutions:**
1. Close tabs for files you're not using
2. Restart the application periodically
3. Consider splitting very large log files

---

### Playback stutters

**Problem:** Playback animation is not smooth.

**Solutions:**
1. Use release build
2. Reduce playback speed
3. Select fewer channels
4. Close other applications

---

## Export Issues

### PNG export is blank or black

**Problem:** Exported PNG shows nothing or is entirely black.

**Solutions:**
1. Ensure at least one channel is selected
2. Verify the chart area shows data
3. Try zooming to a different view before export

---

### PDF export fails

**Problem:** Error when trying to export PDF.

**Solutions:**
1. Ensure you have write permission for the save location
2. Try a different save location
3. Avoid special characters in filename

---

### Exported image quality is low

**Problem:** PNG export appears pixelated or low resolution.

**Current behavior:** Export captures the current view at screen resolution.

**Tip:** Maximize the window before exporting for higher resolution.

---

## Platform-Specific Issues

### Windows

**Issue: Application crashes on startup**
- Ensure Windows 10 or later
- Install Visual C++ Redistributable if missing
- Try running as administrator

**Issue: File dialog doesn't open**
- Restart the application
- Check antivirus isn't blocking

---

### macOS

**Issue: Application name shows as "ultralog" in Dock**
- This is normal for command-line distributed binaries
- Wrap in .app bundle for proper name display

**Issue: Drag and drop doesn't work**
- Grant file access permissions when prompted
- Check System Preferences → Security & Privacy → Privacy → Files and Folders

---

### Linux

**Issue: Window decorations missing**
- May occur on some tiling window managers
- Try running with `GDK_BACKEND=x11 ./ultralog-linux`

**Issue: Blurry text on HiDPI displays**
- Set scaling: `GDK_SCALE=2 ./ultralog-linux`
- Or adjust your desktop environment's scaling settings

---

## Getting More Help

### Before Reporting an Issue

1. Check this troubleshooting guide
2. Search existing [GitHub Issues](https://github.com/SomethingNew71/UltraLog/issues)
3. Try the latest release version

### Reporting a Bug

When opening a GitHub issue, include:

1. **UltraLog version** (check Help → About)
2. **Operating system** and version
3. **Steps to reproduce** the issue
4. **Expected behavior** vs actual behavior
5. **Sample log file** if related to parsing (if possible)
6. **Error messages** or screenshots

### Feature Requests

For new features or ECU support:

1. Open a GitHub Issue
2. Describe the use case
3. For ECU support: include ECU name, software version, and sample file

---

## Quick Fixes Summary

| Problem | Quick Fix |
|---------|-----------|
| Can't open on Windows | Click "More info" → "Run anyway" |
| Can't open on macOS | Right-click → Open |
| Can't run on Linux | `chmod +x ultralog-linux` |
| Wrong units | Check Units menu |
| Slow performance | Use release build, close unused tabs |
| File won't load | Check format is supported, try re-export |

---

## Next Steps

- [[FAQ]] - Frequently asked questions
- [[Installation]] - Detailed installation guide
- [[Supported-ECU-Formats]] - File format details
