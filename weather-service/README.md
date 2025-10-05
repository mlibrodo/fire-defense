# 🌤️ Weather Service

This package provides a **source-agnostic interface** for retrieving and normalizing
weather and wind data used by the **fire defense decider**.
It hides upstream data sources (NOAA/NWS/NDBC/HRRR/etc.) behind a consistent,
typed Python API.

---

## 🔍 Purpose

Different NOAA endpoints expose similar but inconsistent weather fields.
This library abstracts them into a single, typed interface that always returns:

- **Wind** (speed, direction, gust, along/cross components)
- **Weather** (temperature, humidity, dewpoint, VPD, precipitation)
- **Rollups** (summary stats useful for automation decisions)

The consumer — e.g., the fire defense decider — does **not** know or care which
data source (forecast vs. observation) provided the data.

---

## 🧱 Package layout

```
src/weather/
├── __init__.py
├── typedefs.py        # Core types & enums (atomic building blocks)
├── payloads.py        # Atomic weather payloads (Wind, WX, Quality)
├── models.py          # Domain models (SeriesPoint, Segment*, Rollups, Request/Response)
├── math.py            # Shared math utilities (bearing, projection, VPD, rollups)
├── protocols.py       # Adapter protocol definition
├── service.py         # WeatherService orchestrator (routing/fallback logic)
└── adapters/
    ├── __init__.py
    ├── nws.py         # NWS forecast adapter
    └── ndbc.py        # NDBC real-time observation adapter
```

---

## 🌦️ NWS vs NDBC — Data Source Differences

| Feature                | **NWS (National Weather Service)**                       | **NDBC (National Data Buoy Center)**         |
| ---------------------- | -------------------------------------------------------- | -------------------------------------------- |
| **Type**               | Forecast (gridded model output)                          | Real-time observation (station data)         |
| **Coverage**           | Nationwide (land + marine)                               | Coastal & marine stations only               |
| **Latency**            | Forecast data, issued hourly                             | Live data, updated every ~10 min             |
| **Fields available**   | wind speed/dir/gust, temp, dewpoint, RH, precip, hazards | wind speed/dir/gust, sometimes pressure/temp |
| **Spatial resolution** | ~2.5 km grid (via api.weather.gov gridpoints)            | Individual station coordinates               |
| **Time horizon**       | Up to 7 days forecast                                    | Current & recent hours only                  |
| **Use case**           | Predict upcoming conditions                              | Observe current conditions                   |
| **Adapter**            | `NWSAdapter` (forecast)                                  | `NDBCAdapter` (observation)                  |

**In short:**
Use **NDBC** for _now_ (real-time sensor data), and **NWS** for _next_ (forecast grids).
The WeatherService can blend or fallback between them transparently.

---

## 🧩 Architecture

The design follows a clean dependency hierarchy:

```
typedefs  →  payloads  →  models  →  adapters  →  service
                  ↑            ↑         ↑
                 math  ←───────┘         │
                 protocols ──────────────┘
```

- **typedefs.py** — minimal, reusable primitives (no domain logic).
- **payloads.py** — atomic weather concepts (Wind, WX, Quality).
- **models.py** — high-level, typed request/response structures.
- **math.py** — stateless pure functions (geometry & physics).
- **protocols.py** — defines `WeatherAdapter` interface.
- **adapters/** — concrete data source implementations (NWS, NDBC, etc.).
- **service.py** — orchestrates adapters; provides single public entrypoint.

---

## ⚙️ Public API

```python
from datetime import datetime, timezone
from weather import (
    WeatherService, SegmentRequest, LatLon, TimeSpec, TimeMode,
    SamplingSpec, SamplingStrategy, UnitsSpec
)
from weather_service.adapters import NWSAdapter, NDBCAdapter

svc = WeatherService(adapters=[
    NDBCAdapter(station_id="46026", source_token="opaque-obs-1"),
    NWSAdapter(source_token="opaque-fcst-1"),
])

req = SegmentRequest(
    a=LatLon(39.197, -120.238),
    b=LatLon(39.250, -120.150),
    time=TimeSpec(mode=TimeMode.FORECAST, start=datetime.now(timezone.utc), hours=24),
    sampling=SamplingSpec(strategy=SamplingStrategy.POINT_A, level_m_agl=10),
    units=UnitsSpec(),
)

resp = svc.segment(req)
print(resp.segment)
print(len(resp.series), "points")
```

### Returns → `SegmentResponse`

A strongly-typed dataclass containing:

```python
SegmentResponse(
  segment = SegmentMeta(bearing_deg=37.8, length_km=8.7),
  series = (
    SeriesPoint(
      time_utc = datetime(...),
      wind = Wind(speed_ms=6.1, dir_from_deg=210.0, along_ms=3.8, cross_ms=-4.7),
      wx   = WX(temp_c=27.0, rh_pct=18, vpd_kpa=2.6, precip_mm_1h=0.0),
      quality = Quality(source_token="opaque-fcst-1")
    ),
    ...
  ),
  rollups = Rollups(max_gust_ms=14.2, hours_rh_below_20=7, ...),
  meta_units = UnitsSpec(),
  horizon_hours = 24,
  sampling = SamplingStrategy.POINT_A
)
```

---

## 🧮 Key Math Utilities

All vector math and meteorological conversions live in `math.py`:

- `bearing_deg(a, b)` — great-circle bearing A→B
- `haversine_km(a, b)` — path length
- `project_wind_along_cross(speed, dir, bearing)` — tail/cross wind components
- `compute_vpd_kpa(temp, dewpoint)` — vapor pressure deficit
- `rollup(series)` — derive max/min/p95 metrics for the decider

---

## 🪶 Adding a new data source

1. Create a new adapter in `src/weather/adapters/`:
   ```bash
   touch src/weather/adapters/mynewsource.py
   ```
2. Implement the `WeatherAdapter` protocol:

   ```python
   from weather_service.protocols import WeatherAdapter
   from weather_service.models import SegmentRequest, SegmentResponse

   class MyNewSourceAdapter(WeatherAdapter):
       def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
           # 1. Fetch data from your API
           # 2. Convert to canonical units (m/s, °C, %, mm)
           # 3. Build SeriesPoint objects
           # 4. Return SegmentResponse
           ...
   ```

3. Register it in `WeatherService`:
   ```python
   from weather_service.adapters import MyNewSourceAdapter
   svc = WeatherService(adapters=[MyNewSourceAdapter(), ...])
   ```

---

## 🧪 Testing

Each layer can be unit-tested in isolation:

| Layer      | Test focus                             |
| ---------- | -------------------------------------- |
| `math`     | pure functions — deterministic, no I/O |
| `adapters` | mock upstream HTTP responses           |
| `service`  | adapter fallback logic                 |
| `models`   | serialization / consistency checks     |

You can serialize any dataclass tree via:

```python
from dataclasses import asdict
json_payload = asdict(resp)
```

---

## 🧭 Roadmap

- [ ] HRRR gridded model adapter
- [ ] MADIS quality-controlled obs adapter
- [ ] `schemas.py` for external API serialization (Pydantic v2)
- [ ] Fire risk indices (FFWI, ERC) computed from existing fields
- [ ] Local caching / rate limiting layer

---

## 📚 References

- [NOAA Weather.gov API Docs](https://www.weather.gov/documentation/services-web-api)
- [NDBC Realtime Data](https://www.ndbc.noaa.gov/)
- [HRRR/NOMADS GRIB2 Subset](https://nomads.ncep.noaa.gov/)
- [MADIS Public Data Services](https://madis.ncep.noaa.gov/)

---

**Author:**
Internal Fire Defense / Weather Ingestion Layer
© 2025 — All rights reserved.
