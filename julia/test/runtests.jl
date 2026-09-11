include("../CIGPStatistics.jl")
using .CIGPStatistics
using Test

@test observed_rtp([100, 100], [50, 150]) == 1.0
@test frequency_table([:A, :B, :A]) == Dict(:A => 2, :B => 1)
@test chi_square_uniform([10, 10, 10]) == 0.0
@test lag1_autocorrelation([1.0, 1.0, 1.0]) == 0.0
@test_throws ArgumentError observed_rtp([0], [0])
