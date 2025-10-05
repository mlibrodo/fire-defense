from __future__ import annotations

from datetime import timezone
from typing import List

import requests

from ..math import bearing_deg, haversine_km, project_wind_along_cross, rollup
from ..models import SegmentMeta, SegmentRequest, SegmentResponse, SeriesPoint
from ..payloads import WX, Quality, Wind
from ..typedefs import UnitsSpec

_UA = {"User-Agent": "WindAB/1.0 (contact@example.com)"}


def _parse_valid_time(valid: str):
    # "YYYY-MM-DDTHH:MM:SSZ/PT1H"
    start = valid.split("/")[0].replace("Z", "+00:00")
    from datetime import datetime

    return datetime.fromisoformat(start).astimezone(timezone.utc)


class NWSAdapter:
    def __init__(self, source_token: str = "opaque-nws"):
        self.source_token = source_token

    def _grid_values(self, lat: float, lon: float):
        pt = requests.get(
            f"https://api.weather.gov/points/{lat},{lon}", headers=_UA, timeout=20
        ).json()
        grid_url = pt["properties"]["forecastGridData"]
        g = requests.get(grid_url, headers=_UA, timeout=30).json()["properties"]
        return g

    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        a, b = req.a, req.b
        bearing = bearing_deg(a, b)
        length = haversine_km(a, b)

        g = self._grid_values(a.lat, a.lon)
        spds = g["windSpeed"]["values"]  # km/h
        dirs = g["windDirection"]["values"]  # deg-from
        temps = g.get("temperature", {}).get("values", [])  # C
        dews = g.get("dewpoint", {}).get("values", [])  # C
        rhs = g.get("relativeHumidity", {}).get("values", [])  # %
        prec = g.get("quantitativePrecipitation", {}).get(
            "values", []
        )  # kg/m^2 ~ mm for 1h accum

        # index by timestamp
        def _to_map(vals):
            out = {}
            for v in vals:
                t = _parse_valid_time(v["validTime"])
                out[t] = v["value"]
            return out

        m_spd = _to_map(spds)
        m_dir = _to_map(dirs)
        m_tmp = _to_map(temps)
        m_dew = _to_map(dews)
        m_rh = _to_map(rhs)
        m_pr = _to_map(prec)

        times = sorted(set(m_spd) & set(m_dir))
        points: List[SeriesPoint] = []
        for t in times:
            s_ms = (m_spd[t] or 0.0) / 3.6
            ddeg = float(m_dir[t] or 0.0)
            along, cross = project_wind_along_cross(s_ms, ddeg, bearing)

            wx = WX(
                rh_pct=None if t not in m_rh else int(m_rh[t]),
                temp_c=None if t not in m_tmp else float(m_tmp[t]),
                dewpoint_c=None if t not in m_dew else float(m_dew[t]),
                vpd_kpa=None,  # optional: compute if both temp & dew exist
                precip_mm_1h=None if t not in m_pr else float(m_pr[t]),
                red_flag_warning=None,
            )
            wind = Wind(
                speed_ms=s_ms,
                dir_from_deg=ddeg,
                gust_ms=None,  # NWS grid sometimes has "windGust"—you can add it similarly
                along_ms=along,
                cross_ms=cross,
            )
            q = Quality(
                data_age_min=None,
                source_token=self.source_token,
                qflags=("ok",),
            )
            points.append(SeriesPoint(time_utc=t, wind=wind, wx=wx, quality=q))

        resp = SegmentResponse(
            segment=SegmentMeta(bearing_deg=bearing, length_km=length),
            series=tuple(points),
            rollups=rollup(points),
            meta_units=req.units if isinstance(req.units, UnitsSpec) else UnitsSpec(),
            horizon_hours=req.time.hours,
            sampling=req.sampling.strategy,
        )
        return resp
