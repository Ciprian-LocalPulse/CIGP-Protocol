module CIGPStatistics

export observed_rtp, frequency_table, confidence_interval

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

end
