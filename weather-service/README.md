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
src/weather_service/
├── __init__.py
├── typedefs.py        # Core types & enums (atomic building blocks)
├── payloads.py        # Atomic weather payloads (Wind, WX, Quality)
├── models.py          # Domain models (SeriesPoint, Segment*, Rollups, Request/Response)
├── math.py            # Shared math utilities (bearing, projection, VPD, rollups)
├── protocols.py       # Adapter + StationResolver protocols
├── service.py         # WeatherService orchestrator (routing/fallback logic)
├── main.py            # CLI + launcher for FastAPI (imports webapp)
├── webapp.py          # FastAPI app (routers for / and /api)
└── adapters/
    ├── __init__.py
    ├── nws.py         # NWS forecast adapter
    └── ndbc/          # NDBC real-time observation adapter subpackage
        ├── adapter.py
        ├── parser.py
        └── resolver.py
```

---

## 🧩 Architecture

The design follows a clean dependency hierarchy:

```
typedefs  →  payloads  →  models  →  adapters  →  service
                  ↑            ↑         ↑
                 math  ←───────┘         │
                 protocols ──────────────┘
```

- **typedefs.py** — minimal, reusable primitives (no domain logic)
- **payloads.py** — atomic weather concepts (Wind, WX, Quality)
- **models.py** — high-level typed request/response structures
- **math.py** — stateless pure functions (geometry & physics)
- **protocols.py** — defines `WeatherAdapter` + `StationResolver` interfaces
- **adapters/** — concrete data source implementations (NWS, NDBC, etc.)
- **service.py** — orchestrates adapters; single public entrypoint

---

## ⚙️ Public API (source-agnostic)

```python
from datetime import datetime, timezone
from weather_service import (
    WeatherService, SegmentRequest, LatLon, TimeSpec, TimeMode,
    SamplingSpec, SamplingStrategy, UnitsSpec,
)
from weather_service.adapters import NWSAdapter, NDBCAdapter
from weather_service.adapters.ndbc import NDBCStationResolver

# Internal bootstrap (not public API)
resolver = NDBCStationResolver(csv_path="/opt/data/ndbc/stations.csv", max_km=300)

svc = WeatherService(adapters=[
    NDBCAdapter(resolver=resolver, source_token="opaque-obs-1"),
    NWSAdapter(source_token="opaque-fcst-1"),
])

# Publicly exposed interface remains simple and provider-agnostic:
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

## 🚀 Deployment (Render + uv)

The project is preconfigured for [Render](https://render.com) using **uv** for dependency management.

### 🛠️ Build & start commands

**Build:**

```bash
pip install uv && uv sync --extra web --frozen
```

**Start:**

```bash
uv run uvicorn weather_service.webapp:app --host 0.0.0.0 --port $PORT
```

Render automatically provides `$PORT` and handles HTTPS and health checks.

Once live, visit:

```
https://<your-service-name>.onrender.com/
```

> 💡 **More info:**
> For detailed deployment steps, environment variables, and Render health check setup,
> see [docs/RENDER_DEPLOY.md](docs/RENDER_DEPLOY.md).

---

## 📚 References

- [NOAA Weather.gov API Docs](https://www.weather.gov/documentation/services-web-api)
- [NDBC Realtime Data](https://www.ndbc.noaa.gov/)
- [HRRR/NOMADS GRIB2 Subset](https://nomads.ncep.noaa.gov/)
- [MADIS Public Data Services](https://madis.ncep.noaa.gov/)

---

### More documentation

- [Adapters](docs/adapters.md) — NWS vs NDBC, StationResolver, adding new sources
- [Math](docs/math.md) — bearings, haversine, wind projections, VPD, rollups
- [App (CLI + Web)](docs/app.md) — running the CLI and the FastAPI web app
- [Testing](docs/testing.md) — pytest + responses + examples
- [Roadmap](docs/roadmap.md) — planned work
