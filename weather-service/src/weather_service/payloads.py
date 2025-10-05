from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional, Tuple


@dataclass(frozen=True)
class Wind:
    speed_ms: float
    dir_from_deg: float
    gust_ms: Optional[float] = None
    along_ms: Optional[float] = None  # + tailwind toward B
    cross_ms: Optional[float] = None  # + from left


@dataclass(frozen=True)
class WX:
    rh_pct: Optional[int] = None
    temp_c: Optional[float] = None
    dewpoint_c: Optional[float] = None
    vpd_kpa: Optional[float] = None
    precip_mm_1h: Optional[float] = None
    red_flag_warning: Optional[bool] = None


@dataclass(frozen=True)
class Quality:
    data_age_min: Optional[int] = None  # obs freshness
    source_token: str = "opaque-1"  # hides upstream provider
    qflags: Tuple[str, ...] = field(default_factory=lambda: ("ok",))
