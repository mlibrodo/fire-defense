# 🧰 App Guide: CLI + FastAPI Web

The package exposes both a **CLI** and a **FastAPI web application** for interactive use.
Web dependencies are **optional** and installed via the `web` extra.

---

## Install web extras (UV)

```bash
uv add --extra web fastapi uvicorn
# or, if already declared in pyproject:
uv sync --extra web
```

This installs only the additional packages required for the FastAPI web server.

---

## Command-line Interface (CLI)

FireFinder’s CLI provides quick access to the same service logic as the web API.

### Run a nearby fire search

```bash
uv run python -m firefinder_service.main cli --mode nearby     --lat 39.195 --lon -120.235 --radius-mi 50
```

This returns JSON for fires within 50 miles of the given latitude/longitude.

### Compute distance to a specific incident

```bash
uv run python -m firefinder_service.main cli --mode distance     --lat 39.195 --lon -120.235 --id IRWIN123
```

This computes the shortest distance between the given coordinates and the specified fire incident.

---

## Web Application (FastAPI)

FireFinder includes a FastAPI-based web app, exposing HTML and JSON endpoints.

### Option A — Uvicorn import path (preferred)

```bash
uv run uvicorn firefinder_service.webapp:app --host 0.0.0.0 --port 8100 --reload
```

### Option B — Module wrapper (calls uvicorn in code)

```bash
uv run python -m firefinder_service.main web --host 0.0.0.0 --port 8100
```

---

## Endpoints

| Method | Path                                        | Description                                              |
| ------ | ------------------------------------------- | -------------------------------------------------------- |
| `GET`  | `/`                                         | Serves `templates/index.html` (basic HTML harness)       |
| `GET`  | `/api/nearby?lat=...&lon=...&radius_mi=...` | Returns a JSON list of nearby fires                      |
| `GET`  | `/api/incident/{incident_id}`               | Returns incident details for a given ID                  |
| `GET`  | `/api/distance?lat=...&lon=...&id=...`      | Returns shortest distance (point or polygon) to incident |
| `GET`  | `/api/health`                               | Health check endpoint (`{"status": "ok"}`)               |

All numeric fields use **miles** as the default unit for spatial calculations.

---

## Request Parameters

These fields appear in both the CLI and API query strings.

| Parameter   | Type  | Description                                 |
| ----------- | ----- | ------------------------------------------- |
| `lat`       | float | Latitude in decimal degrees                 |
| `lon`       | float | Longitude in decimal degrees                |
| `radius_mi` | float | Search radius in miles (for nearby queries) |
| `id`        | str   | Incident ID (IrwinID / GlobalID / OBJECTID) |
| `mode`      | str   | CLI mode: `nearby` or `distance`            |

Example:

```
GET /api/nearby?lat=39.195&lon=-120.235&radius_mi=50
```

---

## Example JSON Output

```json
{
  "fires": [
    {
      "id": "IRWIN-XYZ",
      "name": "Garnet",
      "state": "NV",
      "county": "Humboldt",
      "acres": 100.0,
      "containment_percent": 25.0,
      "distance_miles": 34.2,
      "severity": "medium"
    }
  ]
}
```

---

## Environment Variables (optional)

| Variable                  | Default | Description                                 |
| ------------------------- | ------- | ------------------------------------------- |
| `FIREFINDER_ADAPTER_MODE` | `rest`  | Switch between `rest`, `sdk`, or `inmemory` |
| `FIREFINDER_PORT`         | `8100`  | Port for FastAPI app                        |
| `FIREFINDER_DEBUG`        | `false` | Enables extra logging for troubleshooting   |

(These are placeholders — you may introduce them later when supporting configurable adapter selection.)

---

## Development Workflow

1. **Install dev deps**
   ```bash
   uv sync --group dev
   ```
2. **Run tests**
   ```bash
   pytest -q
   ```
3. **Start app**
   ```bash
   uv run uvicorn firefinder_service.webapp:app --host 0.0.0.0 --port 8100 --reload
   ```

---

## Troubleshooting

- **FastAPI not found** → install extras: `uv sync --extra web`
- **Port already in use** → change with `--port 8101`
- **ArcGIS REST network error** → check firewall or dataset availability at:
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Incident_Locations_Current/FeatureServer

---

## Notes

The FastAPI routes directly delegate to the same `FireFinderService` that powers the CLI, ensuring identical logic for distance, sorting, and normalization.
