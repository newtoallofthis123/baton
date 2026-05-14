use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Agent {
    Claude,
    Codex,
}

impl Agent {
    pub fn as_str(self) -> &'static str {
        match self {
            Agent::Claude => "claude",
            Agent::Codex => "codex",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "claude" => Some(Agent::Claude),
            "codex" => Some(Agent::Codex),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub source: Agent,
    pub session_id: String,
    pub transcript_path: PathBuf,
    pub cwd: Option<PathBuf>,
    pub started_at: Option<OffsetDateTime>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub enum Block {
    HumanMessage(TextBlock),
    AgentMessage(TextBlock),
    ToolCall(ToolCallBlock),
    ToolResult(ToolResultBlock),
    SystemEvent(SystemEventBlock),
    UnknownEvent(UnknownEventBlock),
}

#[derive(Debug, Clone)]
pub struct TextBlock {
    pub text: String,
    pub source_event_index: usize,
}

#[derive(Debug, Clone)]
pub struct ToolCallBlock {
    pub name: String,
    pub input: String,
    pub source_event_index: usize,
}

#[derive(Debug, Clone)]
pub struct ToolResultBlock {
    pub output: String,
    pub truncated: Option<TruncationInfo>,
    pub source_event_index: usize,
}

#[derive(Debug, Clone)]
pub struct TruncationInfo {
    pub original_chars: usize,
    pub shown_chars: usize,
}

#[derive(Debug, Clone)]
pub struct SystemEventBlock {
    pub label: String,
    pub detail: String,
    pub source_event_index: usize,
}

#[derive(Debug, Clone)]
pub struct UnknownEventBlock {
    pub raw_type: String,
    pub raw_excerpt: String,
    pub source_event_index: usize,
}

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub agent: Agent,
    pub id: String,
    pub path: PathBuf,
    pub cwd: Option<PathBuf>,
    pub started_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSession {
    pub agent: Agent,
    pub id: String,
    pub path: PathBuf,
}
