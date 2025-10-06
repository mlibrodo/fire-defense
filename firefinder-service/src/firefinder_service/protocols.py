from __future__ import annotations

from typing import Iterable, Optional, Protocol

from .payloads import Incident
from .typedefs import LatLon


class FireDataAdapter(Protocol):
    """Abstract provider protocol. Implementations fetch incidents and optional perimeters."""

    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        ...

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        ...
