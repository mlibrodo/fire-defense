# 🧰 App Guide: CLI + FastAPI Web

The package exposes both a CLI and a single FastAPI app (HTML + JSON). Web dependencies are **optional** via a `web` extra.

## Install web extras (UV)

```bash
uv add --extra web fastapi uvicorn
# or, if already declared in pyproject:
uv sync --extra web
```

## Parameters: `mode` and `level_m_agl`

These two fields appear in the request model and query string and control **what data is fetched** and **how wind is interpreted**.

### `mode` (TimeSpec.mode)

Selects the data source family.

| Value      | Meaning                         | Typical Adapter |
| ---------- | ------------------------------- | --------------- |
| `forecast` | Predictive forward-looking data | `NWSAdapter`    |
| `obs`      | Real-time / recent observations | `NDBCAdapter`   |

The service may fall back automatically (e.g., NDBC unavailable → NWS).

### `level_m_agl` (SamplingSpec.level_m_agl)

Height in **meters above ground level** at which wind is referenced.

| Example | Meaning                        | Notes                                        |
| ------: | ------------------------------ | -------------------------------------------- |
|     `2` | Near-surface standard          | Common for temperature/RH                    |
|    `10` | Standard wind reference height | Default for automation                       |
|   `20+` | Elevated/canopy/custom         | Used if your source supports multiple levels |

Adapters will map this to the closest supported level (e.g., 10 m winds in NWS). If a source provides only one level, this may be ignored.

## Run CLI

```bash
uv run python -m weather_service.main cli   --alat 39.197 --alon -120.238   --blat 39.250 --blon -120.150   --hours 24 --mode forecast --level_m_agl 10
```

## Run Web

### Option A — Uvicorn import path (preferred)

```bash
uv run uvicorn weather_service.webapp:app --host 0.0.0.0 --port 8100 --reload
```

### Option B — Module wrapper (calls uvicorn in code)

```bash
uv run python -m weather_service.main web --host 0.0.0.0 --port 8100
```

- `GET /` → serves `templates/index.html`
- `GET /api/wind` → JSON response

If you see `ImportError: fastapi` or `uvicorn`, install extras as shown above.

## Environment variables (optional)

- `NDBC_STATIONS_CSV` — local stations CSV for nearest-station lookup
- `NDBC_MAX_KM` — search radius in km for resolver (default: 300)
