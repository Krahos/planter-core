use std::ops::Deref;

use chrono::Duration;
use thiserror::Error;

/// A duration is a unit of time that represents the amount of time required to complete a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}

/// Maximum duration allowed is 31.68809 years.
pub const MAX_DURATION: i64 = 1_000_000_000_000;

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
