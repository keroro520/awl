use crate::request::{Proof, Request};

/// A trait for prover
pub trait Prover {
    /// Prove the request
    fn prove(&self, request: impl Request) -> impl Proof;
}
