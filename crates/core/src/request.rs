use std::{fmt::Debug, fmt::Display, hash::Hash};

/// A trait for request id
pub trait RequestID: Display + Debug + Clone + PartialEq + Eq + Hash {}

/// A trait for request
pub trait Request: Debug + Clone {
    /// The request id type
    type ID: RequestID;

    /// Get the request id
    fn id(&self) -> Self::ID;
}

/// A trait for proof
pub trait Proof: Debug + Clone {}
