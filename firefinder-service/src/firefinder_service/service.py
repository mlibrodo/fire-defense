from __future__ import annotations

from typing import List, Optional

from .adapters import (
    ArcGisRestAdapter,
    ArcGisRestConfig,
    InMemoryFireAdapter,
    WaterfallAdapter,
    WaterfallPolicy,
)
from .math import haversine_miles, nearest_distance_to_polygon_vertices_miles
from .models import (
    DistanceResponse,
    NearbyFire,
    NearbyFiresRequest,
    NearbyFiresResponse,
)
from .payloads import Incident, IncidentGeometry
from .protocols import FireDataAdapter
from .typedefs import LatLon


def _severity_from(acres: Optional[float], containment: Optional[float]) -> str:
    if acres is None:
        return "unknown"
    if acres >= 10000:
        return "high"
    if acres >= 1000:
        return "medium"
    return "low"


class FireFinderService:
    """Stable public interface for fire search + distance calculations."""

    def __init__(self, adapter: FireDataAdapter):
        self.adapter = adapter

    def search_nearby(self, req: NearbyFiresRequest) -> NearbyFiresResponse:
        incidents = self.adapter.search_incidents_within(req.center, req.radius_miles)
        out: List[NearbyFire] = []
        for inc in incidents:
            dist_candidates = []
            if inc.geometry and inc.geometry.point:
                dist_candidates.append(
                    ("point", haversine_miles(req.center, inc.geometry.point))
                )
            if inc.geometry and inc.geometry.perimeters:
                dpoly = nearest_distance_to_polygon_vertices_miles(
                    req.center, inc.geometry.perimeters
                )
                if dpoly is not None:
                    dist_candidates.append(("polygon_vertices", dpoly))
            basis = None
            dist = None
            if dist_candidates:
                basis, dist = min(dist_candidates, key=lambda x: x[1])
            sev = _severity_from(inc.acres, inc.containment_percent)
            out.append(
                NearbyFire(
                    id=inc.id,
                    name=inc.name,
                    state=inc.state,
                    county=inc.county,
                    created=inc.created,
                    containment_percent=inc.containment_percent,
                    acres=inc.acres,
                    severity=sev,
                    distance_miles=round(dist, 2) if dist is not None else None,
                    sources=[],
                )
            )
        out.sort(
            key=lambda x: (x.distance_miles is None, x.distance_miles or 9e9, x.name)
        )
        return NearbyFiresResponse(fires=out)

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        return self.adapter.get_incident_by_id(incident_id)

    def distance_to_incident(
        self, point: LatLon, incident_id: str
    ) -> Optional[DistanceResponse]:
        inc = self.get_incident_by_id(incident_id)
        if not inc:
            return None
        candidates = []
        if inc.geometry and inc.geometry.point:
            candidates.append(("point", haversine_miles(point, inc.geometry.point)))
        if inc.geometry and inc.geometry.perimeters:
            dpoly = nearest_distance_to_polygon_vertices_miles(
                point, inc.geometry.perimeters
            )
            if dpoly is not None:
                candidates.append(("polygon_vertices", dpoly))
        if not candidates:
            return None
        basis, dist = min(candidates, key=lambda x: x[1])
        return DistanceResponse(
            incident_id=inc.id,
            incident_name=inc.name,
            distance_miles=round(dist, 2),
            basis=basis,
        )


def build_service() -> FireFinderService:
    # Primary: live REST
    rest = ArcGisRestAdapter(
        ArcGisRestConfig(
            incidents_query_url=(
                "https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/"
                "WFIGS_Incident_Locations_Current/FeatureServer/0/query"
            ),
            # perimeters_query_url can remain default or None
        )
    )

    # Fallback: in-memory seed (for offline/dev)
    seed = InMemoryFireAdapter(
        incidents=[
            Incident(
                id="IRWIN123",
                name="Garnet Fire (seed)",
                geometry=IncidentGeometry(point=LatLon(39.20, -120.25)),
                acres=3200,
                containment_percent=45.0,
            )
        ]
    )

    adapter = WaterfallAdapter(
        adapters=[rest, seed],
        policy=WaterfallPolicy(
            failover_on_error=True,  # network/service errors fall through
            failover_on_empty=True,  # empty results fall through
            min_results=0,  # set >0 if you want to keep trying until you have N results
        ),
        on_error=lambda e, a, fn: print(
            f"[waterfall] {a.__class__.__name__}.{fn} error: {e}"
        ),
    )

    return FireFinderService(adapter=adapter)
