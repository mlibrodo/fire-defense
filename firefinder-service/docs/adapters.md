# Adapters

FireFinder separates **data access** from **domain logic** with the `FireDataAdapter`
protocol. Any provider (REST, ArcGIS SDK, mocks) can plug in as long as it implements
this interface.

```python
from typing import Iterable, Optional, Protocol
from firefinder_service.payloads import Incident
from firefinder_service.typedefs import LatLon

class FireDataAdapter(Protocol):
    def search_incidents_within(self, center: LatLon, radius_miles: float) -> Iterable[Incident]: ...
    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]: ...
```

The **service** accepts any adapter implementing this API.

---

## Provider options

### 1) ArcGIS **REST** adapter (recommended for most apps)

File: `adapters/arcgis_rest.py`

- **No `arcgis` SDK install required** — uses plain `requests` against `FeatureServer/0/query`.
- Handles ArcGIS quirks by **falling back** among distance unit variants (miles+geodesic → meters → layer units).
- Normalizes responses to our `Incident` dataclass (id, name, state, county, acres, containment, geometry).

**Incidents endpoint (authoritative current feed):**
**WFIGS_Incident_Locations_Current** (item id: `4181a117dc9e43db8598533e29972015`)

- ArcGIS Item: https://www.arcgis.com/home/item.html?id=4181a117dc9e43db8598533e29972015
- REST Service Root:
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Incident_Locations_Current/FeatureServer
- Layer 0 Query:
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Incident_Locations_Current/FeatureServer/0/query

**Breadcrumb (dataset explorer):**
Current Wildland Fire Incident Locations (point locations for recent and ongoing wildland fires):
https://data-nifc.opendata.arcgis.com/datasets/4181a117dc9e43db8598533e29972015_0/explore?location=0.000000%2C0.000000%2C1.62

> **Tip:** ArcGIS servers may be case-sensitive. Use `/ArcGIS/rest/` (not `/arcgis/rest/`).

**Config knob:** `ArcGisRestConfig.perimeters_query_url` (see Perimeters below).
It’s present but **not yet used** by default — we’ll wire perimeter enrichment next.

---

### 2) ArcGIS **SDK** adapter (optional; heavier dependency)

File: `adapters/arcgis_sdk.py` (skeleton; untested by default)

- Requires the official **ArcGIS Python package** (`arcgis`), which is a **large** dependency and typically
  **Conda-first** (pip installs can be fragile).
- Pros: convenient, object-oriented API; Jupyter mapping widgets.
- Cons: heavy, slower cold start, tricky to deploy on some hosts.

**Example (SDK) usage sketch:**

```python
from arcgis.gis import GIS

gis = GIS()  # anonymous
item = gis.content.get("4181a117dc9e43db8598533e29972015")  # WFIGS_Incident_Locations_Current
layer = item.layers[0]
fs = layer.query(where="1=1", out_fields="*", return_geometry=True)  # FeatureSet
for f in fs.features:
    attrs = f.attributes
    geom = f.geometry  # dict with x/y
    # normalize to Incident(...)
```

> The SDK adapter should **normalize** to the same `Incident` dataclass so the service and tests are identical.
> Keep the SDK adapter behind an **extra** (e.g., `pip install .[arcgis]`) to avoid installing it during CI/unit tests.

---

### 3) Waterfall adapter

File: `adapters/waterfall.py`

- Tries adapters in order, e.g., **ArcGIS REST → InMemory**.
- Policy knobs: `failover_on_error`, `failover_on_empty`, `min_results`.
- Optional `on_error` callback for logging/metrics.

**Wiring example:**

```python
from firefinder_service.adapters.arcgis_rest import ArcGisRestAdapter, ArcGisRestConfig
from firefinder_service.adapters.waterfall import WaterfallAdapter, WaterfallPolicy
from firefinder_service.service import FireFinderService, InMemoryFireAdapter
from firefinder_service.payloads import Incident, IncidentGeometry
from firefinder_service.typedefs import LatLon

rest = ArcGisRestAdapter(ArcGisRestConfig())
seed = InMemoryFireAdapter([Incident(id="SEED", name="Seed Fire",
                                     geometry=IncidentGeometry(point=LatLon(39.2, -120.25)))])
svc = FireFinderService(adapter=WaterfallAdapter(
    adapters=[rest, seed],
    policy=WaterfallPolicy(failover_on_error=True, failover_on_empty=True),
))
```

---

### 4) InMemory adapter

- Used for **tests** and **offline development**.
- Stores normalized `Incident` objects (point and/or perimeter rings) in memory.
- Zero external dependencies.

---

## Datasets (incidents & perimeters)

### A) `WFIGS_Incident_Locations_Current` (incidents — points)

- **What:** Point locations for recent and ongoing wildland fires in the United States.
- **Item:** https://www.arcgis.com/home/item.html?id=4181a117dc9e43db8598533e29972015
- **Dataset Explorer (breadcrumb):**
  https://data-nifc.opendata.arcgis.com/datasets/4181a117dc9e43db8598533e29972015_0/explore?location=0.000000%2C0.000000%2C1.62
- **Service Root:**
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Incident_Locations_Current/FeatureServer
- **Query:**
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Incident_Locations_Current/FeatureServer/0/query

**Common fields (subset):**

- `IncidentName`, `POOState`, `POOCounty`, `CreatedOnDateTime`, `PercentContained`
- IDs: `IrwinID`/`IRWINID`, `GlobalID`, `OBJECTID`
- Size: `IncidentSize` (sometimes a perimeter-derived `GISAcres` appears in related layers)

### B) `WFIGS_Interagency_Fire_Perimeters_to_Date_2025` (perimeters — polygons)

- **What:** Best-available perimeters for all reported wildland fires in the U.S. in the **current year to date**.
- **NIFC Open Data (breadcrumb):**
  https://data-nifc.opendata.arcgis.com/search?tags=cy_wildlandfire_opendata%2CCategory
- **Item:** https://www.arcgis.com/home/item.html?id=7c81ab78d8464e5c9771e49b64e834e9
- **Query (layer 0):**
  https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/WFIGS_Interagency_Fire_Perimeters_to_Date_2025/FeatureServer/0/query

**Planned usage (via `ArcGisRestConfig.perimeters_query_url`):**

1. Query perimeters with the same point+distance buffer as incidents (ensure `outSR=4326` so rings are lat/lon).
2. Normalize `geometry.rings` → `List[List[(lat, lon)]]` and attach into `IncidentGeometry.perimeters`.
3. Merge by `IrwinID`/`IncidentName` with the incident list.
4. Distance calculation in the service already picks **min(point, polygon)** basis.

> This perimeter enrichment is not enabled by default yet. The config knob is present so it can be turned on without changing the public API.
