# 🧪 Testing

We use **pytest** and **responses** to validate math, adapters, service routing, and (optionally) web endpoints.
All external HTTP calls (ArcGIS FeatureServer) are **mocked** for speed and determinism.

---

## Install test deps

```bash
# Install dev group (pytest, responses, etc.)
uv sync --group dev

# Run tests
uv run pytest -q

# Coverage (optional)
uv run pytest -q --cov=firefinder_service --cov-report=term-missing
```

> If you’re not using `uv`, `pip install -e .[dev]` and then `pytest -q` works as well.

---

## Test matrix

| Layer      | Goal                                         | Tools                      |
| ---------- | -------------------------------------------- | -------------------------- |
| `math`     | pure functions deterministic & correct       | pytest                     |
| `models`   | Pydantic integrity / (de)serialization       | pytest                     |
| `adapters` | parse & normalize upstream REST responses    | pytest, responses          |
| `service`  | failover routing across adapters (waterfall) | pytest, responses          |
| `web`      | FastAPI endpoints (smoke)                    | pytest, fastapi.testclient |

---

## Conventions

- **Do not** hit live ArcGIS endpoints in unit tests. Use `responses` to stub them.
- Use **regex URL matchers** so the adapter’s fallback variants (miles → meters → no-units) continue to match.
- Keep mocks **minimal but realistic**: attributes, geometry, and the fields your code consumes.
- Web tests are **optional** (skip when `fastapi` extra isn’t installed).

---

## Helpers you’ll see in tests

### Regex to match any `/query` variant

```python
import re
URL_RE = re.compile(r"^https://example\.com/FeatureServer/0/query(?:\?.*)?$")
```

This matches the same base URL regardless of query string (`geometry`, `distance`, `units`, etc.).

### Reusable fake feature

```python
def _fake_feature(name="Garnet", x=-118.125, y=41.678):
    return {
        "attributes": {
            "IncidentName": name,
            "POOState": "NV",
            "POOCounty": "Humboldt",
            "CreatedOnDateTime": 1736146920000,
            "PercentContained": 25.0,
            "IncidentSize": 100.0,
            "IrwinID": "IRWIN-XYZ",
            "OBJECTID": 99,
        },
        "geometry": {"x": x, "y": y},
    }
```

---

## Example: ArcGIS REST happy-path (nearby search)

```python
import json, re, responses
from firefinder_service.adapters.arcgis_rest import ArcGisRestAdapter, ArcGisRestConfig
from firefinder_service.typedefs import LatLon

URL_RE = re.compile(r"^https://example\.com/FeatureServer/0/query(?:\?.*)?$")

@responses.activate
def test_rest_search_basic_success():
    def ok_cb(req):
        body = {"features": [_fake_feature()]}
        return (200, {}, json.dumps(body))

    responses.add_callback(responses.GET, URL_RE, callback=ok_cb, content_type="application/json")

    ad = ArcGisRestAdapter(ArcGisRestConfig(incidents_query_url="https://example.com/FeatureServer/0/query",
                                            perimeters_query_url=None))
    feats = list(ad.search_incidents_within(LatLon(39.195, -120.235), 50))
    assert feats and feats[0].name == "Garnet"
```

Why use `add_callback`? The adapter may try several parameter combos (miles+geodesic → meters → no-units). A single callback matches **all calls** without counting them.

---

## Example: REST error triggers waterfall fallback

```python
import json, re, responses
from firefinder_service.adapters.arcgis_rest import ArcGisRestAdapter, ArcGisRestConfig
from firefinder_service.adapters.waterfall import WaterfallAdapter, WaterfallPolicy
from firefinder_service.service import FireFinderService, InMemoryFireAdapter
from firefinder_service.payloads import Incident, IncidentGeometry
from firefinder_service.typedefs import LatLon
from firefinder_service.models import NearbyFiresRequest

URL_RE = re.compile(r"^https://example\.com/FeatureServer/0/query(?:\?.*)?$")

@responses.activate
def test_waterfall_rest_error_then_memory_seed():
    def error_cb(req):
        return (200, {}, json.dumps({"error": {"message": "Cannot perform query.", "details": ["Invalid parameter"]}}))

    responses.add_callback(responses.GET, URL_RE, callback=error_cb, content_type="application/json")

    rest = ArcGisRestAdapter(ArcGisRestConfig(incidents_query_url="https://example.com/FeatureServer/0/query",
                                              perimeters_query_url=None))
    memory = InMemoryFireAdapter([
        Incident(id="SEED", name="Seed Fire", geometry=IncidentGeometry(point=LatLon(39.2, -120.25)))
    ])

    svc = FireFinderService(
        adapter=WaterfallAdapter(adapters=[rest, memory],
                                 policy=WaterfallPolicy(failover_on_error=True, failover_on_empty=True))
    )
    resp = svc.search_nearby(NearbyFiresRequest(center=LatLon(39.195, -120.235), radius_miles=50))
    assert any(f.id == "SEED" for f in resp.fires)
```

---

## Example: Distance endpoint (FastAPI) smoke test

```python
import pytest

try:
    from fastapi.testclient import TestClient
    from firefinder_service.webapp import app
    HAVE_WEB = True
except Exception:
    HAVE_WEB = False

@pytest.mark.skipif(not HAVE_WEB, reason="fastapi not installed")
def test_health_and_nearby():
    c = TestClient(app)
    assert c.get("/api/health").json()["status"] == "ok"
    r = c.get("/api/nearby", params={"lat": 39.195, "lon": -120.235, "radius_mi": 25})
    assert r.status_code == 200 and "fires" in r.json()
```

---

## Fixtures & structure

- `conftest.py` provides **shared fixtures** (sample incidents, center point, `InMemoryFireAdapter`, `FireFinderService`).
- Keep test files **narrowly scoped**:
  - `test_math_typedefs.py` — haversine, polygon distance helpers.
  - `test_models_payloads.py` — Pydantic model behavior.
  - `test_arcgis_rest_adapter.py` — REST adapter success/error.
  - `test_waterfall_adapter.py` — failover behavior.
  - `test_firefinder_service.py` — orchestration & sorting.
  - `test_webapp_endpoints.py` — optional web smoke tests.

---

## Tips

- When asserting URL parameters, parse `request.url` in the callback and **avoid coupling** to the exact ordering of params.
- Keep **time fields deterministic** by stubbing timestamps if your code exposes them.
- Prefer **small samples** over large fixtures — clarity beats completeness.
- Use `pytest -q -k <name>` to run a single test file or case during development.

---

Happy testing! 🎯
