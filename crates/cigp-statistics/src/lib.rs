//! Deterministic descriptive statistics for CIGP research datasets.
//!
//! These metrics describe a supplied sample. They do not prove fairness,
//! regulatory compliance, or the absence of manipulation.

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StatisticsError {
    #[error("at least one round is required")]
    EmptySample,
    #[error("total wager must be positive")]
    ZeroWager,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RtpReport {
    pub sample_size: usize,
    pub total_bet_minor_units: i128,
    pub total_payout_minor_units: i128,
    pub observed_rtp: f64,
    pub mean_payout_minor_units: f64,
    pub payout_variance: f64,
    pub confidence_interval_95: [f64; 2],
}

/// Build a descriptive report from exact minor-unit amounts.
///
/// The interval is a normal-approximation interval for the mean payout
/// divided by the average bet. It is a descriptive research output, not a
/// claim that a declared RTP is proven.
pub fn rtp_report(bets: &[i64], payouts: &[i64]) -> Result<RtpReport, StatisticsError> {
    if bets.is_empty() || payouts.is_empty() || bets.len() != payouts.len() {
        return Err(StatisticsError::EmptySample);
    }

    let total_bet: i128 = bets.iter().map(|value| i128::from(*value)).sum();
    if total_bet <= 0 {
        return Err(StatisticsError::ZeroWager);
    }
    let total_payout: i128 = payouts.iter().map(|value| i128::from(*value)).sum();
    let count = payouts.len() as f64;
    let mean = total_payout as f64 / count;
    let variance = if payouts.len() < 2 {
        0.0
    } else {
        payouts
            .iter()
            .map(|value| {
                let delta = *value as f64 - mean;
                delta * delta
            })
            .sum::<f64>()
            / (count - 1.0)
    };
    let mean_bet = total_bet as f64 / count;
    let margin = if payouts.len() < 2 {
        0.0
    } else {
        1.96 * (variance / count).sqrt() / mean_bet
    };
    let observed_rtp = total_payout as f64 / total_bet as f64;

    Ok(RtpReport {
        sample_size: payouts.len(),
        total_bet_minor_units: total_bet,
        total_payout_minor_units: total_payout,
        observed_rtp,
        mean_payout_minor_units: mean,
        payout_variance: variance,
        confidence_interval_95: [(observed_rtp - margin).max(0.0), observed_rtp + margin],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_exact_totals_and_expected_rtp() {
        let report = rtp_report(&[100, 100, 100], &[0, 100, 200]).unwrap();
        assert_eq!(report.total_bet_minor_units, 300);
        assert_eq!(report.total_payout_minor_units, 300);
        assert_eq!(report.observed_rtp, 1.0);
        assert!(report.confidence_interval_95[0] <= report.observed_rtp);
    }

    #[test]
    fn rejects_empty_or_zero_wager_samples() {
        assert_eq!(
            rtp_report(&[], &[]).unwrap_err(),
            StatisticsError::EmptySample
        );
        assert_eq!(
            rtp_report(&[0], &[0]).unwrap_err(),
            StatisticsError::ZeroWager
        );
    }
}
