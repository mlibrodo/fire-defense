from typing import Iterable, List, Optional

from ..math import haversine_miles, nearest_distance_to_polygon_vertices_miles
from ..payloads import Incident
from ..typedefs import LatLon


class InMemoryFireAdapter:
    """Simple in-memory adapter useful for tests and examples."""

    def __init__(self, incidents: Optional[List[Incident]] = None):
        self._incidents = incidents or []

    def add(self, inc: Incident) -> None:
        self._incidents.append(inc)

    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        out = []
        for inc in self._incidents:
            d_point = None
            if inc.geometry and inc.geometry.point:
                d_point = haversine_miles(center, inc.geometry.point)
                if d_point <= radius_miles:
                    out.append(inc)
                    continue
            if inc.geometry and inc.geometry.perimeters:
                d_poly = nearest_distance_to_polygon_vertices_miles(
                    center, inc.geometry.perimeters
                )
                if d_poly is not None and d_poly <= radius_miles:
                    out.append(inc)
        return out

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        for inc in self._incidents:
            if inc.id == incident_id:
                return inc
        return None
