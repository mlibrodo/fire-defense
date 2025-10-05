from __future__ import annotations

from typing import Optional

from ...typedefs import LatLon


class NDBCStationResolver:
    """Simple resolver for NDBC station selection.
    - If constructed with station_id, returns it.
    - (Optional) You can extend with CSV-based nearest lookup later.
    """

    def __init__(
        self,
        station_id: Optional[str] = None,
        *,
        csv_path: Optional[str] = None,
        max_km: float = 300.0,
    ):
        self.station_id = station_id
        self.csv_path = csv_path
        self.max_km = max_km

    def resolve(self, a: LatLon, b: LatLon) -> Optional[str]:
        if self.station_id:
            return self.station_id
        # Future: if csv_path provided, compute nearest station to point A or midpoint.
        return None
