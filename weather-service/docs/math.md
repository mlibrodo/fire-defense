# 🧮 Math

Centralized geometry & meteorology utilities (in `math.py`) keep adapters small and consistent.

## 📏 Distance — Haversine (km)

```
R = 6371.0
φ1 = radians(lat1); φ2 = radians(lat2)
Δφ = φ2 - φ1
Δλ = radians(lon2 - lon1)
a = sin²(Δφ/2) + cos(φ1) * cos(φ2) * sin²(Δλ/2)
c = 2 * atan2(sqrt(a), sqrt(1-a))
d = R * c
```

## 🧭 Bearing — initial course A→B (deg)

```
y = sin(Δλ) * cos(φ2)
x = cos(φ1)*sin(φ2) - sin(φ1)*cos(φ2)*cos(Δλ)
θ = atan2(y, x)
bearing_deg = (degrees(θ) + 360) % 360
```

## 💨 Wind Vector Projection

**Conventions:** `dir_from_deg` is where wind comes **from**; segment bearing is direction A→B.
Steps:

1. Convert to direction wind **blows to**: `to_deg = (dir_from_deg + 180) % 360`
2. Wind vector: `vx = s*cos(rad(to_deg))`, `vy = s*sin(rad(to_deg))`
3. Segment unit vector: `ux = cos(rad(bearing))`, `uy = sin(rad(bearing))`
4. Components:
   - `along = vx*ux + vy*uy` (tailwind +, headwind -)
   - `cross = -vx*uy + vy*ux` (left +, right -)

## 💧 Moisture & VPD

Saturation vapor pressure (Tetens):
`es(T) = 0.6108 * exp((17.27 * T) / (T + 237.3))` (kPa)
Actual vapor pressure from dewpoint: `ea = es(Td)`
**VPD:** `VPD = es(T) - ea`

## 📊 Rollups

Given an ordered series of `SeriesPoint`:

- `max_gust_ms`, `p95_speed_ms`
- `hours_rh_below_20`, `hours_temp_above_35C`
  Rollups are computed in `rollup(points)` and attached to `SegmentResponse`.
