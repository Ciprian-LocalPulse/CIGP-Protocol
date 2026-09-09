//! Exact monetary representation.
//!
//! CIGP never represents monetary amounts as floating-point numbers. All
//! amounts are integer counts of a currency's minor unit (e.g. cents for
//! EUR/USD), matching common practice in payment and ledger systems.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A monetary amount expressed as an exact integer count of minor units,
/// together with the ISO 4217 currency code it is denominated in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    /// ISO 4217 currency code, e.g. "EUR".
    pub currency: [u8; 3],
    /// Amount in minor units (e.g. cents). Never a floating-point value.
    pub minor_units: i64,
}

impl Money {
    pub fn new(currency: &str, minor_units: i64) -> Result<Self, MoneyError> {
        let bytes = currency.as_bytes();
        if bytes.len() != 3 || !bytes.iter().all(|b| b.is_ascii_uppercase()) {
            return Err(MoneyError::InvalidCurrencyCode(currency.to_string()));
        }
        let mut code = [0u8; 3];
        code.copy_from_slice(bytes);
        Ok(Money {
            currency: code,
            minor_units,
        })
    }

    pub fn currency_code(&self) -> String {
        String::from_utf8_lossy(&self.currency).to_string()
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.minor_units, self.currency_code())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    #[error("invalid ISO 4217 currency code: {0}")]
    InvalidCurrencyCode(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_lowercase_currency() {
        assert!(Money::new("eur", 100).is_err());
    }

    #[test]
    fn round_trips() {
        let m = Money::new("EUR", 1050).unwrap();
        assert_eq!(m.currency_code(), "EUR");
        assert_eq!(m.minor_units, 1050);
    }
}
