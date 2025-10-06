from __future__ import annotations

from typing import List, Optional

from pydantic import BaseModel, Field

from .typedefs import LatLon


class NearbyFire(BaseModel):
    id: str
    name: str
    state: Optional[str] = None
    county: Optional[str] = None
    created: Optional[str] = None
    containment_percent: Optional[float] = None
    acres: Optional[float] = None
    severity: Optional[str] = None
    distance_miles: Optional[float] = None
    sources: list[str] = Field(default_factory=list)


class NearbyFiresRequest(BaseModel):
    center: LatLon
    radius_miles: float = Field(gt=0)


class NearbyFiresResponse(BaseModel):
    fires: List[NearbyFire]


class DistanceResponse(BaseModel):
    incident_id: str
    incident_name: str
    distance_miles: float
    basis: str
