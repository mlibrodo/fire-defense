from __future__ import annotations

from typing import Optional, Protocol

from .models import SegmentRequest, SegmentResponse
from .typedefs import LatLon


class WeatherAdapter(Protocol):
    """Adapters must normalize to SegmentResponse and hide methodology."""

    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        ...


class StationResolver(Protocol):
    def resolve(self, a: LatLon, b: LatLon) -> Optional[str]:
        """Return a provider-specific handle (e.g., NDBC station_id) for this segment."""
        ...
