# Field Normalization

Field normalization maps ECU-specific channel names to standardized names, making it easier to work with data from different ECU systems.

## Overview

Different ECU manufacturers use different names for the same data:

| Data | Haltech | ECUMaster | Speeduino |
|------|---------|-----------|-----------|
| Engine speed | Engine Speed | engine/rpm | RPM |
| Air/Fuel Ratio | AFR Bank 1 | fuel/afr | AFR |
| Manifold Pressure | MAP | engine/map | MAP |

Field normalization shows all of these as consistent, standardized names.

---

## Enabling/Disabling

### Toggle Normalization

**View → Field Normalization** toggles the feature on/off.

When **enabled:**
- Channel names show standardized versions
- Makes cross-ECU comparison easier
- Original names still used internally

When **disabled:**
- Channel names show exactly as stored in log file
- Useful for debugging or when you prefer ECU-specific names

---

## Built-in Mappings

UltraLog includes 100+ predefined mappings organized by category.

### Engine Parameters

| ECU Names | Normalized Name |
|-----------|-----------------|
| RPM, Engine_Speed, engine/rpm, EngineSpeed | Engine RPM |
| MAP, Manifold_Press, engine/map, BoostPressure | Manifold Pressure |
| TPS, Throttle_Pos, Throttle, engine/tps | Throttle Position |
| Load, Engine_Load, CalculatedLoad | Engine Load |
| VE, Vol_Eff, VolumetricEfficiency | Volumetric Efficiency |

### Fuel System

| ECU Names | Normalized Name |
|-----------|-----------------|
| AFR, Act_AFR, AFR1, AirFuelRatio | AFR |
| Lambda, Lambda1, O2_Lambda | Lambda |
| FuelPW, Inj_PW, InjectorPulseWidth | Injector Pulse Width |
| FuelPress, Fuel_Pressure | Fuel Pressure |
| STFT, ShortTermFuelTrim | Short Term Fuel Trim |
| LTFT, LongTermFuelTrim | Long Term Fuel Trim |
| InjDuty, Injector_Duty | Injector Duty Cycle |

### Ignition System

| ECU Names | Normalized Name |
|-----------|-----------------|
| Ign_Timing, SparkAdvance, Timing | Ignition Timing |
| Dwell, Dwell_Time, CoilDwell | Dwell Time |
| Knock, KnockLevel, Knock_Retard | Knock Level |

### Temperatures

| ECU Names | Normalized Name |
|-----------|-----------------|
| CLT, Coolant_Temp, CoolantTemperature | Coolant Temp |
| IAT, Intake_Temp, IntakeAirTemp | Intake Air Temp |
| OilTemp, Oil_Temperature | Oil Temp |
| EGT, ExhaustTemp, EGT1 | Exhaust Gas Temp |
| FuelTemp, Fuel_Temperature | Fuel Temp |

### Pressures

| ECU Names | Normalized Name |
|-----------|-----------------|
| BoostPress, Boost, BoostPressure | Boost Pressure |
| OilPress, Oil_Pressure | Oil Pressure |
| FuelRailPress, Rail_Pressure | Fuel Rail Pressure |
| Baro, BaroPressure, Barometric | Barometric Pressure |

### Vehicle Data

| ECU Names | Normalized Name |
|-----------|-----------------|
| VSS, Vehicle_Speed, Speed | Vehicle Speed |
| Gear, GearPosition, CurrentGear | Gear Position |
| BattV, Battery_Voltage, Vbat | Battery Voltage |

### Oxygen Sensors

| ECU Names | Normalized Name |
|-----------|-----------------|
| O2_1, WBO2_1, O2Sensor1 | O2 Sensor 1 |
| O2_2, WBO2_2, O2Sensor2 | O2 Sensor 2 |

---

## Custom Mappings

Create your own mappings for channel names not covered by built-in mappings.

### Opening the Editor

**View → Normalization Editor**

### Adding a Mapping

1. Open the Normalization Editor
2. In the **Source Name** field, enter the ECU-specific name exactly as it appears
3. In the **Target Name** field, enter your preferred standardized name
4. Click **Add**

### Example

Your ECU logs a channel called `MyCustomAFR_Gauge`:

1. Source Name: `MyCustomAFR_Gauge`
2. Target Name: `AFR`
3. Click Add

Now `MyCustomAFR_Gauge` displays as `AFR`.

### Removing a Mapping

In the Normalization Editor:
1. Find the mapping in the list
2. Click the **Remove** or **×** button

### Persistence

**Important:** Custom mappings are stored in memory only and reset when UltraLog closes.

Future versions may add persistent storage for custom mappings.

---

## How Normalization Works

### Processing Order

1. Channel name read from log file
2. Check custom mappings first
3. If no custom match, check built-in mappings
4. If no match found, use original name
5. Display normalized name in UI

### Case Sensitivity

Mappings are **case-insensitive** for matching:
- `RPM`, `rpm`, `Rpm` all match

The normalized name preserves the specified case.

### Partial Matching

Mappings use **exact matching** only:
- `RPM` matches `RPM`
- `RPM` does NOT match `Engine_RPM_Value`

Use the exact channel name for matching.

---

## Use Cases

### Cross-ECU Comparison

When analyzing logs from different ECU systems:

1. Enable Field Normalization
2. Load logs from different ECUs
3. Channel names show consistently
4. Easier to compare data across tabs

### Team Standardization

When multiple team members use UltraLog:

1. Agree on standard channel names
2. Each member adds custom mappings as needed
3. Everyone sees consistent names

### Personal Preference

Simplify complex channel names:

- `Bank1_WBO2_AFR_Calibrated` → `AFR`
- `Engine_Coolant_Temperature_Sensor_1` → `Coolant`

---

## Best Practices

### Naming Conventions

Recommended standard names:

| Category | Recommended Format |
|----------|-------------------|
| Temperature | `[Type] Temp` (e.g., "Coolant Temp") |
| Pressure | `[Type] Pressure` (e.g., "Oil Pressure") |
| Ratio | Abbreviation (e.g., "AFR", "Lambda") |
| Speed | `[Type] Speed` (e.g., "Vehicle Speed") |
| Percentage | `[Item] %` (e.g., "TPS %") |

### Avoiding Conflicts

- Don't map different channels to the same name
- Use descriptive target names
- Document your custom mappings

### When to Disable

Consider disabling normalization when:
- Debugging parsing issues
- You need exact ECU channel names
- Working with ECU-specific documentation

---

## Troubleshooting

### Mapping Not Working

1. Verify the source name matches exactly (check spaces, underscores)
2. Check if a built-in mapping conflicts
3. Ensure normalization is enabled (View menu)

### Wrong Channel Normalized

1. Check for conflicting custom mappings
2. Verify the source name is correct
3. Remove and re-add the mapping

### Finding Original Names

To see original channel names:
1. Disable Field Normalization (View menu)
2. Channel names show as stored in log file

---

## Complete Built-in Mapping List

### A-D

| Source | Target |
|--------|--------|
| Act_AFR | AFR |
| AFR | AFR |
| AFR1 | AFR |
| AFR_Bank1 | AFR |
| AirFuelRatio | AFR |
| Baro | Barometric Pressure |
| BaroPressure | Barometric Pressure |
| Barometric | Barometric Pressure |
| BattV | Battery Voltage |
| Battery_Voltage | Battery Voltage |
| Boost | Boost Pressure |
| BoostPress | Boost Pressure |
| BoostPressure | Boost Pressure |
| CLT | Coolant Temp |
| Coolant_Temp | Coolant Temp |
| CoolantTemperature | Coolant Temp |
| CurrentGear | Gear Position |
| Dwell | Dwell Time |
| Dwell_Time | Dwell Time |

### E-L

| Source | Target |
|--------|--------|
| EGT | Exhaust Gas Temp |
| EGT1 | Exhaust Gas Temp |
| Engine_Load | Engine Load |
| Engine_Speed | Engine RPM |
| EngineSpeed | Engine RPM |
| ExhaustTemp | Exhaust Gas Temp |
| Fuel_Pressure | Fuel Pressure |
| Fuel_Temperature | Fuel Temp |
| FuelPress | Fuel Pressure |
| FuelRailPress | Fuel Rail Pressure |
| FuelTemp | Fuel Temp |
| Gear | Gear Position |
| GearPosition | Gear Position |
| IAT | Intake Air Temp |
| Ign_Timing | Ignition Timing |
| Inj_PW | Injector Pulse Width |
| InjDuty | Injector Duty Cycle |
| Injector_Duty | Injector Duty Cycle |
| InjectorPulseWidth | Injector Pulse Width |
| Intake_Temp | Intake Air Temp |
| IntakeAirTemp | Intake Air Temp |
| Knock | Knock Level |
| Knock_Retard | Knock Level |
| KnockLevel | Knock Level |
| Lambda | Lambda |
| Lambda1 | Lambda |
| Load | Engine Load |
| LongTermFuelTrim | Long Term Fuel Trim |
| LTFT | Long Term Fuel Trim |

### M-R

| Source | Target |
|--------|--------|
| Manifold_Press | Manifold Pressure |
| MAP | Manifold Pressure |
| O2_1 | O2 Sensor 1 |
| O2_2 | O2 Sensor 2 |
| O2_Lambda | Lambda |
| O2Sensor1 | O2 Sensor 1 |
| O2Sensor2 | O2 Sensor 2 |
| Oil_Pressure | Oil Pressure |
| Oil_Temperature | Oil Temp |
| OilPress | Oil Pressure |
| OilTemp | Oil Temp |
| Rail_Pressure | Fuel Rail Pressure |
| RPM | Engine RPM |

### S-Z

| Source | Target |
|--------|--------|
| ShortTermFuelTrim | Short Term Fuel Trim |
| SparkAdvance | Ignition Timing |
| Speed | Vehicle Speed |
| STFT | Short Term Fuel Trim |
| Throttle | Throttle Position |
| Throttle_Pos | Throttle Position |
| Timing | Ignition Timing |
| TPS | Throttle Position |
| Vbat | Battery Voltage |
| VE | Volumetric Efficiency |
| Vehicle_Speed | Vehicle Speed |
| Vol_Eff | Volumetric Efficiency |
| VolumetricEfficiency | Volumetric Efficiency |
| VSS | Vehicle Speed |
| WBO2_1 | O2 Sensor 1 |
| WBO2_2 | O2 Sensor 2 |

### ECUMaster Hierarchical Paths

| Source | Target |
|--------|--------|
| engine/rpm | Engine RPM |
| engine/map | Manifold Pressure |
| engine/tps | Throttle Position |
| engine/coolant_temp | Coolant Temp |
| fuel/afr | AFR |
| fuel/lambda | Lambda |

---

## Next Steps

- [[User-Guide]] - Complete feature reference
- [[Unit-Conversion]] - Configure display units
- [[Supported-ECU-Formats]] - ECU-specific information
