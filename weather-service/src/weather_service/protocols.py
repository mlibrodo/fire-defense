from __future__ import annotations

from typing import Protocol

from .models import SegmentRequest, SegmentResponse


class WeatherAdapter(Protocol):
    """Adapters must normalize to SegmentResponse and hide methodology."""

    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        ...
