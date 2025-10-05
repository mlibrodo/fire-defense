# 🧩 Adapters

This project ships with two built-in adapters and a station resolver for observations.

## Parameters that influence adapter behavior

- **`TimeSpec.mode` (`forecast` | `obs`)** — directs the service toward forecast grids (NWS) or observations (NDBC). The service can fall back between adapters if one is unavailable.
- **`SamplingSpec.level_m_agl` (meters AGL)** — desired wind reference height. Mapped to the nearest supported level by the adapter (e.g., NWS often exposes 10 m winds).

## 🌦️ NWSAdapter (Forecasts)

**Source:** `api.weather.gov` (gridpoints)
**Coverage:** Nationwide (land + marine)
**Spatial:** ~2.5 km grid (NDFD)
**Temporal:** Up to ~7 days; hourly to 3‑hour steps
**Latency:** Forecast issuance cadence

**Typical fields:** `windSpeed`, `windDirection`, `windGust`, `temperature`, `dewpoint`, `relativeHumidity`, `quantitativePrecipitation`

**Flow:**

1. `GET /points/{lat},{lon}` → provides office + `forecastGridData` URL
2. `GET /gridpoints/{office}/{x},{y}` → parse `properties.<field>.values[]`
3. Normalize units (to SI) → build `SeriesPoint` list
4. Project along/cross components using segment bearing (see **Math**)
5. Return `SegmentResponse`

## 🌊 NDBCAdapter (Observations)

**Source:** NDBC realtime2 station text files, e.g. `https://www.ndbc.noaa.gov/data/realtime2/46026.txt`
**Coverage:** Coastal & marine buoys/stations
**Temporal:** Current + recent hours (10‑min / hourly)
**Latency:** Near real‑time

**Fields parsed (when present):** `WDIR` (deg from), `WSPD` (wind speed), `GST` (gust). Some feeds include air temp/pressure.

**Flow:**

1. `NDBCStationResolver.resolve(a, b)` → returns station id or `None`
2. `_latest_ndbc_row(station)` → parse newest row (header + first data line)
3. Normalize to SI → build single `SeriesPoint` at observation `time_utc`
4. Project along/cross → build `SegmentResponse`

### 🔒 Station Resolution & Privacy

`NDBCStationResolver` hides station IDs from callers.

- **Explicit:** `station_id="46026"` (tests/fixed deploys)
- **Nearest (future-ready):** set `csv_path` and pick nearest to point A or midpoint within `max_km`
- If no station is available → adapter raises `RuntimeError("unavailable")` → `WeatherService` falls back to forecast

## ➕ Adding a New Adapter

1. Create `src/weather_service/adapters/my_source.py` implementing `WeatherAdapter`.
2. Export from `adapters/__init__.py` and register in `WeatherService(adapters=[...])`.
3. Write unit tests with mocked HTTP using the `responses` library.
