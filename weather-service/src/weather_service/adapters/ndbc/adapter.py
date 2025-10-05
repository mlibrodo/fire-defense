from __future__ import annotations

import math
from typing import List

from ...math import bearing_deg, haversine_km, project_wind_along_cross, rollup
from ...models import SegmentMeta, SegmentRequest, SegmentResponse, SeriesPoint
from ...payloads import WX, Quality, Wind
from ...protocols import StationResolver
from ...typedefs import UnitsSpec
from .parser import _latest_ndbc_row


class NDBCAdapter:
    def __init__(self, resolver: StationResolver, source_token: str = "opaque-ndbc"):
        self.resolver = resolver
        self.source_token = source_token

    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        a, b = req.a, req.b
        bearing = bearing_deg(a, b)
        length = haversine_km(a, b)

        station_id = self.resolver.resolve(a, b)
        if not station_id:
            # no station available -> let WeatherService fall back to next adapter
            raise RuntimeError("unavailable")

        latest = _latest_ndbc_row(station_id)
        points: List[SeriesPoint] = []
        if latest:
            s_ms = float(latest["WSPD"])
            ddeg = float(latest["WDIR"])
            along, cross = project_wind_along_cross(s_ms, ddeg, bearing)
            gust_val = latest.get("GST", float("nan"))
            gust_ms = None if math.isnan(gust_val) else float(gust_val)
            wind = Wind(
                speed_ms=s_ms,
                dir_from_deg=ddeg,
                gust_ms=gust_ms,
                along_ms=along,
                cross_ms=cross,
            )
            wx = WX()
            q = Quality(
                data_age_min=None,
                source_token=self.source_token,
                qflags=("ok",),
            )
            points.append(
                SeriesPoint(time_utc=latest["time_utc"], wind=wind, wx=wx, quality=q)
            )

        resp = SegmentResponse(
            segment=SegmentMeta(bearing_deg=bearing, length_km=length),
            series=tuple(points),
            rollups=rollup(points),
            meta_units=req.units if isinstance(req.units, UnitsSpec) else UnitsSpec(),
            horizon_hours=req.time.hours,
            sampling=req.sampling.strategy,
        )
        return resp
