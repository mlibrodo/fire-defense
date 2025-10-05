from __future__ import annotations

import math
from typing import Iterable, List, Optional, Tuple

from .models import Rollups, SeriesPoint
from .typedefs import LatLon


def bearing_deg(a: LatLon, b: LatLon) -> float:
    φ1, φ2 = math.radians(a.lat), math.radians(b.lat)
    dλ = math.radians(b.lon - a.lon)
    y = math.sin(dλ) * math.cos(φ2)
    x = math.cos(φ1) * math.sin(φ2) - math.sin(φ1) * math.cos(φ2) * math.cos(dλ)
    return (math.degrees(math.atan2(y, x)) + 360) % 360


def haversine_km(a: LatLon, b: LatLon) -> float:
    R = 6371.0088
    dφ = math.radians(b.lat - a.lat)
    dλ = math.radians(b.lon - a.lon)
    φ1 = math.radians(a.lat)
    φ2 = math.radians(b.lat)
    h = math.sin(dφ / 2) ** 2 + math.cos(φ1) * math.cos(φ2) * math.sin(dλ / 2) ** 2
    return 2 * R * math.asin(math.sqrt(h))


def project_wind_along_cross(
    speed_ms: float, dir_from_deg: float, bearing_AB_deg: float
) -> Tuple[float, float]:
    d = math.radians(dir_from_deg)  # met "from" direction
    u = -speed_ms * math.sin(d)  # east (toward)
    v = -speed_ms * math.cos(d)  # north (toward)
    θ = math.radians(bearing_AB_deg)
    ex, ey = math.sin(θ), math.cos(θ)  # along-track
    ax, ay = -ey, ex  # left of track
    along = u * ex + v * ey
    cross = u * ax + v * ay
    return along, cross


def compute_vpd_kpa(temp_c: float, dewpoint_c: float) -> float:
    es = 0.6108 * math.exp((17.27 * temp_c) / (temp_c + 237.3))
    ea = 0.6108 * math.exp((17.27 * dewpoint_c) / (dewpoint_c + 237.3))
    return max(es - ea, 0.0)


def _p95(xs: List[float]) -> Optional[float]:
    if not xs:
        return None
    xs = sorted(xs)
    i = max(0, min(len(xs) - 1, int(round(0.95 * (len(xs) - 1)))))
    return xs[i]


def rollup(series: Iterable[SeriesPoint]) -> Rollups:
    rows = list(series)
    if not rows:
        return Rollups()
    gusts = [p.wind.gust_ms for p in rows if p.wind.gust_ms is not None]
    alongs = [p.wind.along_ms for p in rows if p.wind.along_ms is not None]
    crosses = [abs(p.wind.cross_ms) for p in rows if p.wind.cross_ms is not None]
    rhs = [p.wx.rh_pct for p in rows if p.wx.rh_pct is not None]
    vpds = [p.wx.vpd_kpa for p in rows if p.wx.vpd_kpa is not None]
    hours_below_20 = sum(
        1 for r in rows if r.wx.rh_pct is not None and r.wx.rh_pct < 20
    )
    return Rollups(
        max_gust_ms=max(gusts) if gusts else None,
        max_along_ms=max(alongs) if alongs else None,
        max_cross_ms_abs=max(crosses) if crosses else None,
        min_rh_pct=min(rhs) if rhs else None,
        vpd_kpa_p95=_p95([v for v in vpds if v is not None]),
        hours_rh_below_20=hours_below_20,
    )
