from cigp_anomaly import classify, z_score


def test_z_score_is_deterministic():
    assert z_score(3.0, [1.0, 2.0, 3.0]) == 1.0


def test_anomaly_is_investigation_signal_not_accusation():
    result = classify(10.0, [1.0, 2.0, 3.0])
    assert result["status"] == "ANOMALY"
    assert "investigation" in result["recommended_investigation"].lower()
