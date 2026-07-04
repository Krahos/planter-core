use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use chrono::Duration;
use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

/// A duration is a unit of time that represents the amount of time required to complete a task.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonNegativeDuration(Duration);

/// Represents an error that occurs when trying to parse a negative duration.
#[derive(Error, Debug)]
pub enum DurationError {
    /// Used when the wanted duration would be negative.
    #[error("Negative values are not allowed for durations")]
    NegativeDuration,
    /// Used when trying to parse an invalid string.
    #[error("Input string couldn't be parsed into a NonNegativeDuration")]
    InvalidInput,
}

#[allow(clippy::expect_used)]
static DURATION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9]+) h$").expect("hardcoded regex is valid"));

impl NonNegativeDuration {
    /// Tries to parse a string and return the corresponding `[NonNegativeDuration]`
    ///
    /// # Arguments
    /// * `s` - The string to parse. Currently, only hours are supported in the format "X h".
    ///
    /// # Returns
    /// * `Ok(NonNegativeDuration)` - If the input string could be parsed into a `NonNegativeDuration`.
    /// * `Err(DurationError)` - If the input string couldn't be parsed into a `NonNegativeDuration`.
    ///
    /// # Errors
    /// * `DurationError::InvalidInput` - If the input string couldn't be parsed into a `NonNegativeDuration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use planter_core::duration::NonNegativeDuration;
    ///
    /// let duration = NonNegativeDuration::parse_from_str("8 h").unwrap();
    /// assert_eq!(duration.num_hours(), 8);
    /// ```
    pub fn parse_from_str(s: &str) -> Result<Self, DurationError> {
        if let Some(caps) = DURATION_RE.captures(s) {
            let hours: i64 = caps[1].parse().map_err(|_| DurationError::InvalidInput)?;
            Ok(NonNegativeDuration(Duration::hours(hours)))
        } else {
            Err(DurationError::InvalidInput)
        }
    }
}

impl TryFrom<Duration> for NonNegativeDuration {
    type Error = DurationError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        if value < Duration::milliseconds(0) {
            Err(DurationError::NegativeDuration)
        } else {
            Ok(NonNegativeDuration(value))
        }
    }
}

impl Deref for NonNegativeDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NonNegativeDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} h", self.num_hours())
    }
}

impl FromStr for NonNegativeDuration {
    type Err = DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_from_str(s)
    }
}

#[cfg(test)]
/// Utilities to test with duration.
pub mod test_utils {
    use proptest::prelude::Strategy;

    /// Generate a random duration string.
    pub fn duration_string() -> impl Strategy<Value = String> {
        r"[0-9]{1,12} h"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::test_utils::duration_string;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parse_from_str_works(s in duration_string()) {
            let hours = s.split(' ').next().unwrap().parse::<i64>().unwrap();
            let duration = NonNegativeDuration::parse_from_str(&s).unwrap();
            assert_eq!(duration.num_hours(), hours);
        }

        #[test]
        fn parse_from_str_fails_with_invalid_input(s in "\\PC*") {
            if !DURATION_RE.is_match(&s) {
                assert!(NonNegativeDuration::parse_from_str(&s).is_err())
            }
        }
    }
}
