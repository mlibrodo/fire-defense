from __future__ import annotations

from datetime import datetime, timezone
from typing import List, Optional

import pandas as pd
import requests

from ..math import bearing_deg, haversine_km, project_wind_along_cross, rollup
from ..models import SegmentMeta, SegmentRequest, SegmentResponse, SeriesPoint
from ..payloads import WX, Quality, Wind
from ..typedefs import UnitsSpec


def _latest_ndbc_row(station_id: str) -> Optional[dict]:
    url = f"https://www.ndbc.noaa.gov/data/realtime2/{station_id}.txt"
    txt = requests.get(url, timeout=20).text
    lines = txt.splitlines()

    # Find header line (starts with #) and data lines
    header_line = None
    data_lines = []

    for line in lines:
        line = line.strip()
        if not line:
            continue
        if line.startswith("#"):
            # Remove # and extract header
            header_line = line[1:].strip()
        else:
            data_lines.append(line)

    if not header_line or not data_lines:
        return None

    header = header_line.split()
    idx = {h: i for i, h in enumerate(header)}
    row = data_lines[0].split()  # newest

    def col(name, default=None):
        try:
            return row[idx[name]]
        except Exception:
            return default

    YY, MM, DD, hh, mm = map(
        int, [col("YY"), col("MM"), col("DD"), col("hh"), col("mm")]
    )
    t = datetime(2000 + YY if YY < 100 else YY, MM, DD, hh, mm, tzinfo=timezone.utc)
    return {
        "time_utc": t,
        "WDIR": float(col("WDIR")),
        "WSPD": float(col("WSPD")),  # m/s
        "GST": float(col("GST", "nan")),  # m/s
    }


class NDBCAdapter:
    def __init__(self, station_id: str, source_token: str = "opaque-ndbc"):
        self.station_id = station_id
        self.source_token = source_token

    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        a, b = req.a, req.b
        bearing = bearing_deg(a, b)
        length = haversine_km(a, b)

        latest = _latest_ndbc_row(self.station_id)
        points: List[SeriesPoint] = []
        if latest:
            s_ms = float(latest["WSPD"])
            ddeg = float(latest["WDIR"])
            along, cross = project_wind_along_cross(s_ms, ddeg, bearing)
            wind = Wind(
                speed_ms=s_ms,
                dir_from_deg=ddeg,
                gust_ms=(None if pd.isna(latest["GST"]) else float(latest["GST"])),
                along_ms=along,
                cross_ms=cross,
            )
            # NDBC feed here does not include RH/Temp; you could extend with
            # co-located sources
            wx = WX()
            q = Quality(
                data_age_min=5,  # you may compute real age by comparing now-obs time
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
