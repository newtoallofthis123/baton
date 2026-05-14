use std::path::PathBuf;

use crate::model::{Agent, Conversation, ResolvedSession, SessionSummary};
use crate::providers::{Provider, ProviderError};

pub struct CodexProvider {
    pub configured_roots: Vec<PathBuf>,
}

impl CodexProvider {
    pub fn new(configured_roots: Vec<PathBuf>) -> Self {
        Self { configured_roots }
    }
}

impl Provider for CodexProvider {
    fn agent(&self) -> Agent {
        Agent::Codex
    }
    fn effective_roots(&self, _configured_roots: &[PathBuf]) -> Vec<PathBuf> {
        todo!("chapter D")
    }
    fn list_sessions(&self) -> Result<Vec<SessionSummary>, ProviderError> {
        todo!("chapter D")
    }
    fn resolve_session(&self, _id: &str) -> Result<ResolvedSession, ProviderError> {
        todo!("chapter D")
    }
    fn parse_transcript(
        &self,
        _session: &ResolvedSession,
    ) -> Result<Conversation, ProviderError> {
        todo!("chapter D")
    }
}
