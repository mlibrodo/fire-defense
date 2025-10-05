from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Optional, Tuple

from .payloads import WX, Quality, Wind
from .typedefs import LatLon, SamplingSpec, SamplingStrategy, TimeSpec, UnitsSpec


@dataclass(frozen=True)
class SegmentMeta:
    bearing_deg: float
    length_km: float


@dataclass(frozen=True)
class SeriesPoint:
    time_utc: datetime
    wind: Wind
    wx: WX
    quality: Quality


@dataclass(frozen=True)
class Rollups:
    max_gust_ms: Optional[float] = None
    max_along_ms: Optional[float] = None
    max_cross_ms_abs: Optional[float] = None
    min_rh_pct: Optional[int] = None
    vpd_kpa_p95: Optional[float] = None
    hours_rh_below_20: Optional[int] = None


@dataclass(frozen=True)
class SegmentRequest:
    a: LatLon
    b: LatLon
    time: TimeSpec
    sampling: SamplingSpec
    units: UnitsSpec


@dataclass(frozen=True)
class SegmentResponse:
    segment: SegmentMeta
    series: Tuple[SeriesPoint, ...]
    rollups: Rollups
    meta_units: UnitsSpec
    horizon_hours: int
    sampling: SamplingStrategy
