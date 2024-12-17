use std::fmt::Debug;

type RequestID = alloy_primitives::B256;

/// A trait for request
pub trait Request: Debug + Clone {
    /// Get the request id
    fn id(&self) -> RequestID;
}

/// A trait for proof
pub trait Proof: Debug + Clone {}
