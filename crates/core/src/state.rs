use chrono::{DateTime, Utc};

/// The state of a proof request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    /// The proof request is pending.
    Pending,
    /// The proof request is running.
    Running,
    /// The proof request succeeded.
    Succeeded,
    /// The proof request failed.
    Failed,
    /// The proof request timed out.
    TimedOut,
    /// The proof request was cancelled.
    Cancelled,
}

/// A rich state of a proof request.
pub struct RichState {
    inner: State,
    message: String,
    timestamp: DateTime<Utc>,
}

impl RichState {
    pub fn new(state: State, message: impl Into<String>) -> Self {
        Self {
            inner: state,
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
}
