from __future__ import annotations

from typing import Dict, List

from .models import SegmentRequest, SegmentResponse
from .protocols import WeatherAdapter


class WeatherService:
    def __init__(self, adapters: List[WeatherAdapter]):
        self.adapters = adapters

    def segment(self, req: SegmentRequest) -> SegmentResponse:
        errors: Dict[str, str] = {}
        for ad in self.adapters:
            try:
                return ad.get_segment_series(req)
            except Exception as e:
                errors[ad.__class__.__name__] = str(e)
                continue
        raise RuntimeError(f"No data available from adapters: {errors}")
