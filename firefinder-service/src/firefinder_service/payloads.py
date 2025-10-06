from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from .typedefs import LatLon, Polygon


@dataclass(frozen=True)
class IncidentGeometry:
    point: Optional[LatLon] = None
    perimeters: Optional[Polygon] = None  # optional polygon (rings)


@dataclass(frozen=True)
class Incident:
    id: str
    name: str
    state: Optional[str] = None
    county: Optional[str] = None
    created: Optional[str] = None
    containment_percent: Optional[float] = None
    acres: Optional[float] = None
    severity: Optional[str] = None  # e.g., low/medium/high computed heuristically
    geometry: Optional[IncidentGeometry] = None
