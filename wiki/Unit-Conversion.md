# Unit Conversion

UltraLog provides comprehensive unit conversion for 8 measurement categories. This page details all available options and conversion formulas.

## Overview

- Access unit settings via the **Units** menu
- Changes apply immediately to all displayed values
- Original data is never modified - conversions are display-only
- Preferences are maintained for the session

---

## Temperature

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Kelvin | K | SI base unit |
| Celsius | °C | Metric standard |
| Fahrenheit | °F | Imperial standard |

### Conversion Formulas

```
Celsius = Kelvin - 273.15
Fahrenheit = (Kelvin - 273.15) × 9/5 + 32
```

### ECU Data Notes

- **Haltech:** Stores temperature in Kelvin
- **ECUMaster:** Typically in Celsius
- **Speeduino:** Varies by configuration

### Common Temperature Channels

- Coolant Temperature
- Intake Air Temperature (IAT)
- Oil Temperature
- Exhaust Gas Temperature (EGT)
- Fuel Temperature

---

## Pressure

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Kilopascals | kPa | SI standard |
| PSI | psi | Pounds per square inch |
| Bar | bar | Metric bar |

### Conversion Formulas

```
PSI = kPa × 0.145038
Bar = kPa × 0.01
```

### Common Pressure Channels

- Manifold Absolute Pressure (MAP)
- Boost Pressure
- Oil Pressure
- Fuel Pressure
- Barometric Pressure

### Understanding MAP Values

| kPa | psi | bar | Condition |
|-----|-----|-----|-----------|
| 101.3 | 14.7 | 1.01 | Atmospheric (sea level) |
| 50 | 7.25 | 0.5 | Vacuum (naturally aspirated) |
| 150 | 21.8 | 1.5 | ~7 psi boost |
| 200 | 29.0 | 2.0 | ~14 psi boost |
| 250 | 36.3 | 2.5 | ~21 psi boost |

---

## Speed

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Kilometers per hour | km/h | Metric standard |
| Miles per hour | mph | Imperial standard |

### Conversion Formula

```
mph = km/h × 0.621371
```

### Common Speed Channels

- Vehicle Speed
- Wheel Speed (individual wheels)

---

## Distance

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Kilometers | km | Metric standard |
| Miles | mi | Imperial standard |

### Conversion Formula

```
miles = km × 0.621371
```

### Common Distance Channels

- Trip Distance
- Odometer
- GPS Distance

---

## Fuel Economy

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Liters per 100km | L/100km | Metric (lower is better) |
| Miles per gallon | MPG | Imperial (higher is better) |

### Conversion Formula

```
MPG = 235.215 / L/100km
```

**Note:** These units are inversely related - L/100km measures fuel consumption while MPG measures fuel efficiency.

### Interpretation Guide

| L/100km | MPG | Efficiency |
|---------|-----|------------|
| 5 | 47 | Excellent |
| 8 | 29 | Good |
| 12 | 20 | Average |
| 15 | 16 | Poor |
| 20 | 12 | Very Poor |

---

## Volume

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Liters | L | Metric standard |
| Gallons | gal | US gallon |

### Conversion Formula

```
gallons = liters × 0.264172
```

### Common Volume Channels

- Fuel Level
- Fuel Used
- Tank Capacity

---

## Flow Rate

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Liters per minute | L/min | Metric standard |
| Gallons per minute | GPM | Imperial standard |

### Conversion Formula

```
GPM = L/min × 0.264172
```

### Common Flow Rate Channels

- Fuel Flow
- Air Flow
- Water/Coolant Flow

### Fuel Flow Reference

| L/min | GPM | Approximate Power (gasoline) |
|-------|-----|------------------------------|
| 0.5 | 0.13 | ~100 HP |
| 1.0 | 0.26 | ~200 HP |
| 1.5 | 0.40 | ~300 HP |
| 2.0 | 0.53 | ~400 HP |

*Note: Actual power depends on BSFC and other factors*

---

## Acceleration

### Available Units

| Unit | Symbol | Description |
|------|--------|-------------|
| Meters per second squared | m/s² | SI standard |
| Gravitational force | g | Earth gravity reference |

### Conversion Formula

```
g = m/s² / 9.80665
```

### Reference Values

| m/s² | g | Context |
|------|---|---------|
| 9.81 | 1.0 | Earth gravity |
| 4.9 | 0.5 | Moderate acceleration |
| 9.8 | 1.0 | Hard braking/acceleration |
| 14.7 | 1.5 | Very hard maneuver |
| 19.6 | 2.0 | Extreme (racing) |

### Common Acceleration Channels

- Longitudinal G (acceleration/braking)
- Lateral G (cornering)
- Vertical G (suspension movement)

---

## Default Units

UltraLog uses these defaults:

| Category | Default |
|----------|---------|
| Temperature | Celsius |
| Pressure | kPa |
| Speed | km/h |
| Distance | km |
| Fuel Economy | L/100km |
| Volume | Liters |
| Flow Rate | L/min |
| Acceleration | m/s² |

---

## How Unit Conversion Works

### Data Flow

1. **Raw Data** - ECU logs data in native format
2. **Parser** - Converts to base units (Kelvin, kPa, etc.)
3. **Storage** - Data stored in base units
4. **Display** - User preference conversion applied
5. **Legend** - Shows converted value with unit symbol

### Example

```
Haltech logs coolant temp as 363.15 (Kelvin)
→ Parser reads raw value: 363.15 K
→ Stored in memory: 363.15 K
→ User preference: Celsius
→ Displayed: 90.0 °C
→ If changed to Fahrenheit: 194.0 °F
```

### Why Base Units?

Using base units internally:
- Ensures consistency across ECU formats
- Simplifies conversion logic
- Prevents cumulative rounding errors
- Allows instant unit switching

---

## Tips

### Choosing Units

- Use units familiar to your region/industry
- Match units to your reference documentation
- Consider your audience if sharing exports

### Comparing Data

- Use the same units when comparing logs
- Note that some ECUs log in different units
- Field Normalization helps standardize naming

### Troubleshooting

If values seem wrong:
1. Check your Unit Preferences
2. Verify the ECU's native units
3. Some channels may not support unit conversion

---

## Next Steps

- [[User-Guide]] - Complete feature reference
- [[Field-Normalization]] - Channel name standardization
- [[Supported-ECU-Formats]] - ECU-specific information
