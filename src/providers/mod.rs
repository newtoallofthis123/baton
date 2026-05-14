use crate::model::{Agent, Conversation, ResolvedSession, SessionSummary};
use std::path::PathBuf;

pub mod claude;
pub mod codex;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("transcript root not found: {0}")]
    RootNotFound(PathBuf),
    #[error("session id `{0}` not found")]
    SessionNotFound(String),
    #[error("session id `{id}` matched multiple transcripts: {matches:?}")]
    AmbiguousSession { id: String, matches: Vec<PathBuf> },
    #[error("transcript unreadable at {path}: {source}")]
    TranscriptUnreadable {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid JSONL at {path}:{line}: {source}")]
    InvalidJsonl {
        path: PathBuf,
        line: usize,
        #[source]
        source: serde_json::Error,
    },
}

pub trait Provider {
    fn agent(&self) -> Agent;
    fn effective_roots(&self, configured_roots: &[PathBuf]) -> Vec<PathBuf>;
    fn list_sessions(&self) -> Result<Vec<SessionSummary>, ProviderError>;
    fn resolve_session(&self, id: &str) -> Result<ResolvedSession, ProviderError>;
    fn parse_transcript(&self, session: &ResolvedSession) -> Result<Conversation, ProviderError>;
}

pub fn for_agent(agent: Agent, config: &crate::config::Config) -> Box<dyn Provider> {
    match agent {
        Agent::Claude => Box::new(claude::ClaudeProvider::new(config.roots.claude.clone())),
        Agent::Codex => Box::new(codex::CodexProvider::new(config.roots.codex.clone())),
    }
}
