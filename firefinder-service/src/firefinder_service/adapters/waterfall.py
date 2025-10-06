from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Iterable, List, Optional, Sequence

from ..payloads import Incident
from ..protocols import FireDataAdapter
from ..typedefs import LatLon


@dataclass(frozen=True)
class WaterfallPolicy:
    """
    Controls when we fail over to the next adapter.
    - failover_on_error: if adapter raises, try next
    - failover_on_empty: if adapter returns zero results, try next
    """

    failover_on_error: bool = True
    failover_on_empty: bool = True
    # If >0, will continue trying adapters until at least this many results found
    min_results: int = 0


class WaterfallAdapter(FireDataAdapter):
    """
    Tries a list of FireDataAdapter implementations in order.
    Example: [ArcGisRestAdapter(...), InMemoryFireAdapter(...)]
    """

    def __init__(
        self,
        adapters: Sequence[FireDataAdapter],
        policy: WaterfallPolicy | None = None,
        on_error: Optional[Callable[[Exception, FireDataAdapter, str], None]] = None,
    ) -> None:
        if not adapters:
            raise ValueError("WaterfallAdapter requires at least one adapter")
        self._adapters: List[FireDataAdapter] = list(adapters)
        self._policy = policy or WaterfallPolicy()
        self._on_error = on_error

    # -------- FireDataAdapter API --------

    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        aggregate: List[Incident] = []
        last_exc: Optional[Exception] = None

        for idx, adp in enumerate(self._adapters):
            try:
                res = list(adp.search_incidents_within(center, radius_miles))
            except Exception as e:
                last_exc = e
                if self._on_error:
                    self._on_error(e, adp, "search_incidents_within")
                if self._policy.failover_on_error and idx < len(self._adapters) - 1:
                    continue
                # No more failover → re-raise last error
                raise

            aggregate = res
            if res and (
                not self._policy.min_results or len(res) >= self._policy.min_results
            ):
                return res  # good enough, stop early

            # Decide whether to fall through
            if (not res and self._policy.failover_on_empty) and idx < len(
                self._adapters
            ) - 1:
                continue
            else:
                return res  # either empty is acceptable, or no more adapters

        # If loop exhausted with errors only
        if last_exc:
            raise last_exc
        return aggregate

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        last_exc: Optional[Exception] = None
        for idx, adp in enumerate(self._adapters):
            try:
                inc = adp.get_incident_by_id(incident_id)
            except Exception as e:
                last_exc = e
                if self._on_error:
                    self._on_error(e, adp, "get_incident_by_id")
                if self._policy.failover_on_error and idx < len(self._adapters) - 1:
                    continue
                raise
            if inc is not None:
                return inc
            if self._policy.failover_on_empty and idx < len(self._adapters) - 1:
                continue
            return None
        if last_exc:
            raise last_exc
        return None
