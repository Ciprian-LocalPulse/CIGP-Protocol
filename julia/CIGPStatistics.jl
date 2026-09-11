module CIGPStatistics

export observed_rtp, frequency_table, confidence_interval, chi_square_uniform, lag1_autocorrelation

function observed_rtp(bets::AbstractVector{<:Integer}, payouts::AbstractVector{<:Integer})
    length(bets) == length(payouts) || throw(ArgumentError("bets and payouts must have equal length"))
    total_bet = sum(bets)
    total_bet > 0 || throw(ArgumentError("total bet must be positive"))
    return sum(payouts) / total_bet
end

function frequency_table(values)
    result = Dict{eltype(values), Int}()
    for value in values
        result[value] = get(result, value, 0) + 1
    end
    return result
end

function confidence_interval(successes::Integer, trials::Integer; z::Real=1.96)
    trials > 0 || throw(ArgumentError("trials must be positive"))
    p = successes / trials
    margin = z * sqrt(p * (1 - p) / trials)
    return (max(0.0, p - margin), min(1.0, p + margin))
end

"""Pearson chi-square statistic against a uniform discrete distribution."""
function chi_square_uniform(counts::AbstractVector{<:Integer})
    isempty(counts) && throw(ArgumentError("at least one bucket is required"))
    total = sum(counts)
    total > 0 || throw(ArgumentError("total count must be positive"))
    expected = total / length(counts)
    return sum((count - expected)^2 / expected for count in counts)
end

"""Lag-one sample autocorrelation; returns zero for a constant sequence."""
function lag1_autocorrelation(values::AbstractVector{<:Real})
    length(values) >= 2 || throw(ArgumentError("at least two observations are required"))
    mean_value = sum(values) / length(values)
    denominator = sum((value - mean_value)^2 for value in values)
    denominator == 0 && return 0.0
    numerator = sum((values[i] - mean_value) * (values[i + 1] - mean_value) for i in 1:length(values)-1)
    return numerator / denominator
end

end
