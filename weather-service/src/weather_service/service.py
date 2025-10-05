from dataclasses import dataclass
from typing import Dict, List, Optional

from .models import SegmentRequest, SegmentResponse
from .protocols import WeatherAdapter


@dataclass(frozen=True)
class ServicePolicy:
    """Configuration for WeatherService adapter routing."""

    prefer_obs: bool = True
    obs_staleness_max_min: int = 60  # Max acceptable age for observation data
    allow_fallback_to_forecast: bool = True


class WeatherService:
    """Top-level orchestrator that queries adapters in order until one succeeds."""

    def __init__(
        self, adapters: List[WeatherAdapter], policy: Optional[ServicePolicy] = None
    ):
        self.adapters = adapters
        self.policy = policy or ServicePolicy()

    def segment(self, req: SegmentRequest) -> SegmentResponse:
        """Return the first available SegmentResponse from the configured adapters."""
        errors: Dict[str, str] = {}

        for ad in self.adapters:
            # Optional: skip adapters based on simple policy heuristics
            if not self._adapter_allowed(ad):
                continue

            try:
                return ad.get_segment_series(req)
            except Exception as e:
                errors[ad.__class__.__name__] = str(e)
                continue

        raise RuntimeError(f"No data available from adapters: {errors}")

    def _adapter_allowed(self, ad: WeatherAdapter) -> bool:
        """Placeholder for future logic — e.g., skip obs if staleness too high."""
        # Could inspect adapter metadata here if they expose data_age_min, etc.
        return True
