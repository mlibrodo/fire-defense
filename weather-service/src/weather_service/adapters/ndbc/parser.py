from __future__ import annotations

from datetime import datetime, timezone
from typing import Optional

import requests


def _latest_ndbc_row(station_id: str) -> Optional[dict]:
    """Fetch latest row from NDBC realtime2 text file for a station.
    Returns dict with time_utc (datetime), WDIR, WSPD, GST (floats), or None if parse fails.
    """
    url = f"https://www.ndbc.noaa.gov/data/realtime2/{station_id}.txt"
    resp = requests.get(url, timeout=20)
    if resp.status_code != 200:
        return None
    txt = resp.text
    header_line = None
    data_lines = []
    for line in txt.splitlines():
        line = line.strip()
        if not line:
            continue
        if line.startswith("#"):
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

    try:
        YY = int(col("YY"))
        MM = int(col("MM"))
        DD = int(col("DD"))
        hh = int(col("hh"))
        mm = int(col("mm"))
        when = datetime(
            2000 + YY if YY < 100 else YY, MM, DD, hh, mm, tzinfo=timezone.utc
        )
        WDIR = float(col("WDIR"))
        WSPD = float(col("WSPD"))
        GST_raw = col("GST", "nan")
        GST = float(GST_raw) if GST_raw not in (None, "", "MM") else float("nan")
        return {"time_utc": when, "WDIR": WDIR, "WSPD": WSPD, "GST": GST}
    except Exception:
        return None
