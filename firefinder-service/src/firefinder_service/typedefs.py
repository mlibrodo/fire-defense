from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Sequence, Tuple


class DistanceUnit(str, Enum):
    MILES = "miles"
    METERS = "meters"
    KILOMETERS = "kilometers"


@dataclass(frozen=True)
class LatLon:
    lat: float
    lon: float


# Polygon encoding: list of rings; each ring is a list of (lat, lon) tuples.
Polygon = Sequence[Sequence[Tuple[float, float]]]
