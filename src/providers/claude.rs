use std::path::PathBuf;

use crate::model::{Agent, Conversation, ResolvedSession, SessionSummary};
use crate::providers::{Provider, ProviderError};

pub struct ClaudeProvider {
    pub configured_roots: Vec<PathBuf>,
}

impl ClaudeProvider {
    pub fn new(configured_roots: Vec<PathBuf>) -> Self {
        Self { configured_roots }
    }
}

impl Provider for ClaudeProvider {
    fn agent(&self) -> Agent {
        Agent::Claude
    }
    fn effective_roots(&self, _configured_roots: &[PathBuf]) -> Vec<PathBuf> {
        todo!("chapter C")
    }
    fn list_sessions(&self) -> Result<Vec<SessionSummary>, ProviderError> {
        todo!("chapter C")
    }
    fn resolve_session(&self, _id: &str) -> Result<ResolvedSession, ProviderError> {
        todo!("chapter C")
    }
    fn parse_transcript(
        &self,
        _session: &ResolvedSession,
    ) -> Result<Conversation, ProviderError> {
        todo!("chapter C")
    }
}
