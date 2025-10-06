# 🔥 FireFinder Service

This package provides a **source-agnostic interface** for discovering nearby wildland fire incidents
and computing distances from a user location to incidents using **incident point locations** and
(optionally) **fire perimeter polygons**. It hides upstream data sources (e.g., ArcGIS REST / NIFC)
behind a consistent, typed Python API.

---

## 🔍 Purpose

Different upstream datasets expose similar concepts (incident name, size, geometry) with varying
field names and formats. FireFinder normalizes those into a **stable public interface** so that
callers do **not** need to know which provider is used.

It returns:

- **Nearby fires** — normalized list with distance, acres, containment, severity.
- **Incident lookup** — fetch by ID (IrwinID / GlobalID / OBJECTID) or name.
- **Distance** — from an arbitrary point to an incident (uses the shortest of point vs polygon distance).

---

## 🧱 Package layout

```
src/firefinder_service/
├── __init__.py
├── typedefs.py        # Core types & enums (LatLon, DistanceUnit, geometry helpers)
├── payloads.py        # Atomic fire concepts (Incident, IncidentGeometry)
├── models.py          # Request/Response models (NearbyFires*, DistanceResponse)
├── service.py         # FireFinderService orchestrator (adapter-agnostic logic)
├── webapp.py          # FastAPI app (routers for / and /api)
├── main.py            # CLI + uvicorn launcher (imports webapp)
└── adapters/
    ├── arcgis_rest.py # ArcGIS REST adapter (FeatureServer /query via requests)
    └── waterfall.py   # Waterfall adapter (REST → InMemory fallback)
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

- **typedefs.py** — minimal, reusable primitives (no domain logic).
- **payloads.py** — atomic fire concepts (Incident, IncidentGeometry).
- **models.py** — high-level typed request/response structures used by callers.
- **math** — stateless pure functions (haversine distance + polygon approximations).
- **protocols** — represented by `FireDataAdapter` (the adapter interface).
- **adapters/** — concrete providers (ArcGIS REST, InMemory for tests), plus `WaterfallAdapter`.
- **service.py** — orchestrates adapters; presents a single public entrypoint.

---

## ⚙️ Public API (source-agnostic)

```python
from firefinder_service import FireFinderService, InMemoryFireAdapter, LatLon
from firefinder_service.models import NearbyFiresRequest

# Build an adapter (in production, use Waterfall + ArcGIS REST; here we keep it simple)
adapter = InMemoryFireAdapter()
svc = FireFinderService(adapter=adapter)

req = NearbyFiresRequest(center=LatLon(39.197, -120.238), radius_miles=25)
resp = svc.search_nearby(req)
print([f.name for f in resp.fires])
```

---

## 🚀 Deployment (Render + uv)

Preconfigured for Render using **uv**.

**Build:**

```bash
pip install uv && uv sync --extra web --frozen
```

**Start:**

```bash
uv run uvicorn firefinder_service.webapp:app --host 0.0.0.0 --port $PORT
```

Health check: `GET /api/health` → `{ "status": "ok" }`

More details in **docs/render_deploy.md**.

---

## 📚 References

- NIFC Open Data Search: https://data-nifc.opendata.arcgis.com/search?tags=cy_wildlandfire_opendata%2CCategory
- WFIGS Incident Locations (current): https://www.arcgis.com/home/item.html?id=4181a117dc9e43db8598533e29972015
- WFIGS Interagency Fire Perimeters to Date (current year): https://www.arcgis.com/home/item.html?id=7c81ab78d8464e5c9771e49b64e834e9

See **docs/adapters.md** for direct FeatureServer URLs and adapter specifics.
