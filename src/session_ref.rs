use crate::model::Agent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRef {
    pub agent: Agent,
    pub id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionRefError {
    #[error("session reference must be `<agent>:<session_id>`")]
    Malformed,
    #[error("unknown agent `{0}` (expected `claude` or `codex`)")]
    UnknownAgent(String),
    #[error("session id cannot be empty")]
    EmptyId,
}

impl SessionRef {
    pub fn parse(s: &str) -> Result<Self, SessionRefError> {
        let (a, id) = s.split_once(':').ok_or(SessionRefError::Malformed)?;
        let agent = Agent::parse(a).ok_or_else(|| SessionRefError::UnknownAgent(a.to_string()))?;
        if id.is_empty() {
            return Err(SessionRefError::EmptyId);
        }
        Ok(SessionRef {
            agent,
            id: id.to_string(),
        })
    }
}

impl std::fmt::Display for SessionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.agent.as_str(), self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_claude_ref() {
        let r = SessionRef::parse("claude:abc").unwrap();
        assert_eq!(r.agent, Agent::Claude);
        assert_eq!(r.id, "abc");
    }

    #[test]
    fn parses_codex_ref() {
        let r = SessionRef::parse("codex:abc").unwrap();
        assert_eq!(r.agent, Agent::Codex);
        assert_eq!(r.id, "abc");
    }

    #[test]
    fn rejects_missing_colon() {
        assert!(matches!(
            SessionRef::parse("claudeabc"),
            Err(SessionRefError::Malformed)
        ));
    }

    #[test]
    fn rejects_unknown_agent() {
        match SessionRef::parse("foo:abc") {
            Err(SessionRefError::UnknownAgent(a)) => assert_eq!(a, "foo"),
            other => panic!("expected UnknownAgent, got {:?}", other),
        }
    }

    #[test]
    fn rejects_empty_id() {
        assert!(matches!(
            SessionRef::parse("claude:"),
            Err(SessionRefError::EmptyId)
        ));
    }

    #[test]
    fn display_roundtrips() {
        let r = SessionRef::parse("codex:xyz").unwrap();
        assert_eq!(r.to_string(), "codex:xyz");
    }
}
