from __future__ import annotations

from math import asin, cos, radians, sin, sqrt
from typing import Optional

from .typedefs import LatLon, Polygon


def haversine_miles(a: LatLon, b: LatLon) -> float:
    R = 3958.7613
    dlat = radians(b.lat - a.lat)
    dlon = radians(b.lon - a.lon)
    aa = (
        sin(dlat / 2) ** 2
        + cos(radians(a.lat)) * cos(radians(b.lat)) * sin(dlon / 2) ** 2
    )
    return 2 * R * asin(sqrt(aa))


def nearest_distance_to_polygon_vertices_miles(
    pt: LatLon, polygon: Polygon
) -> Optional[float]:
    """Fast approximation: distance to nearest vertex of polygon (miles).
    Returns None if the polygon is empty.
    """
    best = None
    for ring in polygon:
        for lat, lon in ring:
            d = haversine_miles(pt, LatLon(lat=lat, lon=lon))
            if best is None or d < best:
                best = d
    return best
