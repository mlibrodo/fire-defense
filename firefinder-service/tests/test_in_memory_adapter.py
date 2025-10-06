def test_inmemory_search_hits_point(inmemory_adapter, pt_center):
    svc_center = pt_center
    results = list(
        inmemory_adapter.search_incidents_within(svc_center, radius_miles=50)
    )
    ids = {x.id for x in results}
    # Should at least include the point incident "A"
    assert "A" in ids


def test_inmemory_get_by_id(inmemory_adapter):
    inc = inmemory_adapter.get_incident_by_id("B")
    assert inc is not None and inc.name == "Bravo"
