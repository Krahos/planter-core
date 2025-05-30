use std::ops::Deref;

use chrono::Duration;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use thiserror::Error;

/// A duration is a unit of time that represents the amount of time required to complete a task.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveDuration(Duration);

/// Represents an error that occurs when trying to parse a negative duration.
#[derive(Error, Debug)]
pub enum DurationError {
    /// Used when the wanted duration would be negative.
    #[error("Negative values are not allowed for durations")]
    NegativeDuration,
    /// Used when the wanted duration would exceed the maximum allowed value.
    #[error("Duration would exceed maximum allowed value")]
    ExceedsMaximumDuration,
    /// Used when trying to parse an invalid string.
    #[error("Input string couldn't be parsed into a PositiveDuration")]
    InvalidInput,
}

impl PositiveDuration {
    /// Tries to parse a string and return the corresponding `[PositiveDuration]`
    ///
    /// # Arguments
    /// * `s` - The string to parse. Currently, only hours are supported in the format "X h".
    ///
    /// # Returns
    /// * `Ok(PositiveDuration)` - If the input string could be parsed into a `PositiveDuration`.
    /// * `Err(DurationError)` - If the input string couldn't be parsed into a `PositiveDuration`.
    ///
    /// # Errors
    /// * `DurationError::InvalidInput` - If the input string couldn't be parsed into a `PositiveDuration`.
    /// * `DurationError::ExceedsMaximumDuration` - If the parsed duration exceeds the maximum allowed value.
    ///
    /// # Panics
    /// This function uses `expect`, but it should only panic in case of a bug.
    ///
    /// # Examples
    ///
    /// ```
    /// use planter_core::duration::PositiveDuration;
    ///
    /// let duration = PositiveDuration::parse_from_str("8 h").unwrap();
    /// assert_eq!(duration.num_hours(), 8);
    /// ```
    ///
    /// ```should_panic
    /// use planter_core::duration::PositiveDuration;
    ///
    /// /// Passing invalid input will result in an `[DurationError::InvalidInput]`
    /// let duration = PositiveDuration::parse_from_str("random garbage").unwrap();
    /// ```
    #[allow(clippy::expect_used)]
    #[allow(clippy::unwrap_in_result)]
    pub fn parse_from_str(s: &str) -> Result<Self, DurationError> {
        let bytes = s.as_bytes();
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^[0-9]{1,12} h$")
                .expect("It wasn't possible to compile a hardcoded regex. This is a bug.")
        });
        if RE.is_match(bytes) {
            let hours = s.split(' ').next().expect("Expecting to retrieve the hours from the string after matching the regex. This is a bug.").parse::<i64>().expect("Expecting to convert the hours to an i64. This is a bug.");
            if hours > MAX_DURATION {
                Err(DurationError::ExceedsMaximumDuration)
            } else {
                Ok(PositiveDuration(Duration::hours(hours)))
            }
        } else {
            Err(DurationError::InvalidInput)
        }
    }
}

/// Maximum duration allowed is ~31.68809 years.
pub const MAX_DURATION: i64 = 999_999_999_999;

impl TryFrom<Duration> for PositiveDuration {
    type Error = DurationError;

    /// Creates a new `PositiveDuration` from a `chrono::Duration`.
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        if value < Duration::milliseconds(0) {
            Err(DurationError::NegativeDuration)
        } else if value > Duration::milliseconds(MAX_DURATION) {
            Err(DurationError::ExceedsMaximumDuration)
        } else {
            Ok(PositiveDuration(value))
        }
    }
}

impl Deref for PositiveDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
/// Utilities to run tests with duration.
pub mod test_utils {
    use proptest::prelude::Strategy;

    /// Generate a random duration string.
    pub fn duration_string() -> impl Strategy<Value = String> {
        r"[0-9]{1,12} h".prop_map(|s: String| s.to_owned())
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
            if !(0..=MAX_DURATION).contains(&hours) {
                assert!(PositiveDuration::parse_from_str(&s).is_err());
            } else {
                let duration = PositiveDuration::parse_from_str(&s).unwrap();
                assert_eq!(duration.num_hours(), hours);
            }
        }

        #[test]
        fn parse_from_str_fails_with_invalid_input(s in "\\PC*") {
            let bytes = s.as_bytes();
            static RE: Lazy<Regex> = Lazy::new(|| {
                Regex::new(r"^[0-9]{1,12} h$")
                    .expect("It wasn't possible to compile a hardcoded regex. This is a bug.")
            });
            if !RE.is_match(bytes) {
                assert!(PositiveDuration::parse_from_str(&s).is_err())
            }
        }
    }
}
