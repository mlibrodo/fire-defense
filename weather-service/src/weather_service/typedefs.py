from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from enum import Enum


@dataclass(frozen=True)
class LatLon:
    lat: float
    lon: float


class TimeMode(str, Enum):
    FORECAST = "forecast"
    OBS = "obs"


@dataclass(frozen=True)
class TimeSpec:
    mode: TimeMode
    start: datetime  # UTC
    hours: int  # horizon


class SamplingStrategy(str, Enum):
    POINT_A = "pointA"
    BLEND_A_MID_B = "blend(A,mid,B)"
    PATH_INTEGRATED = "path-integrated"


@dataclass(frozen=True)
class SamplingSpec:
    strategy: SamplingStrategy
    level_m_agl: int = 10  # wind height


class SpeedUnit(str, Enum):
    MS = "m/s"
    KPH = "km/h"
    MPH = "mph"


class DirUnit(str, Enum):
    DEG = "deg"


class TempUnit(str, Enum):
    C = "C"
    F = "F"


class PressureUnit(str, Enum):
    KPA = "kPa"


class PrecipUnit(str, Enum):
    MM = "mm"


@dataclass(frozen=True)
class UnitsSpec:
    speed: SpeedUnit = SpeedUnit.MS
    dir: DirUnit = DirUnit.DEG
    temp: TempUnit = TempUnit.C
    pressure: PressureUnit = PressureUnit.KPA
    precip: PrecipUnit = PrecipUnit.MM
