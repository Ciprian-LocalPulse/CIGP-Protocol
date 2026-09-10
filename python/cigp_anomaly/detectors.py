from math import sqrt


def z_score(value: float, observations: list[float]) -> float:
    if len(observations) < 2:
        raise ValueError("at least two observations are required")
    mean = sum(observations) / len(observations)
    variance = sum((item - mean) ** 2 for item in observations) / (len(observations) - 1)
    deviation = sqrt(variance)
    return 0.0 if deviation == 0 else (value - mean) / deviation


def classify(value: float, observations: list[float], threshold: float = 3.0) -> dict[str, object]:
    score = z_score(value, observations)
    status = "NORMAL" if abs(score) < threshold else "ANOMALY"
    return {
        "status": status,
        "reason": "value outside configured z-score threshold" if status == "ANOMALY" else "within configured threshold",
        "metric": "z_score",
        "observed_value": value,
        "expected_value": sum(observations) / len(observations),
        "threshold": threshold,
        "recommended_investigation": "Investigation recommended: review source events and configuration" if status == "ANOMALY" else "None",
    }
