use std::collections::HashMap;

use eyre::Error;

use crate::core::prover::Prover;
use crate::core::request::{Proof, Request};

/// Manager is the kernel of awl task management
pub struct Manager {
    provers: HashMap<String, Box<dyn Prover>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            provers: HashMap::new(),
        }
    }

    pub fn add_prover(&mut self, name: impl Into<String>, prover: impl Prover) {
        assert!(
            !self.provers.contains_key(&name.into()),
            "Prover already exists"
        );
        self.provers.insert(name.into(), Box::new(prover));
    }

    pub fn get_prover(&self, name: impl Into<String>) -> Option<&Box<dyn Prover>> {
        self.provers.get(&name.into())
    }

    pub async fn prove(
        &self,
        prover_name: impl Into<String>,
        request: impl Request,
    ) -> Result<Proof, Error> {
        let prover = self.get_prover(prover_name).ok_or(eyre!("Prover not found"))?;

        prover
            .prove(request)
            .map_err(|e| {
                error!("Prover failed to prove: {e}");
                eyre!("Prover failed to prove: {e}")
            )
    }
}
