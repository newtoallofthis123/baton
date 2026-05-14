use std::path::Path;

use crate::model::Agent;

#[derive(Debug)]
pub struct LaunchResult {
    pub status: std::process::ExitStatus,
}

#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("executable for `{0}` not found on PATH")]
    ExecutableNotFound(String),
    #[error("launching `{cmd}` failed: {source}")]
    Spawn {
        cmd: String,
        #[source]
        source: std::io::Error,
    },
    #[error("`{cmd}` exited with status {status}")]
    NonZeroExit {
        cmd: String,
        status: std::process::ExitStatus,
    },
}

pub trait Launcher {
    fn launch(&self, target: Agent, prompt: &str) -> Result<LaunchResult, LaunchError>;
}

pub struct ProcessLauncher;

impl Launcher for ProcessLauncher {
    fn launch(&self, target: Agent, prompt: &str) -> Result<LaunchResult, LaunchError> {
        let cmd = target.as_str();
        let mut child = std::process::Command::new(cmd)
            .arg(prompt)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    LaunchError::ExecutableNotFound(cmd.to_string())
                } else {
                    LaunchError::Spawn {
                        cmd: cmd.to_string(),
                        source: e,
                    }
                }
            })?;
        let status = child.wait().map_err(|e| LaunchError::Spawn {
            cmd: cmd.to_string(),
            source: e,
        })?;
        if !status.success() {
            return Err(LaunchError::NonZeroExit {
                cmd: cmd.to_string(),
                status,
            });
        }
        Ok(LaunchResult { status })
    }
}

pub fn catch_up_prompt(handoff_path: &Path) -> String {
    format!(
        "You are catching up from a previous Claude Code/Codex conversation.\n\n\
         Read this handoff file:\n{}\n\n\
         Use it as context and continue naturally. Tool output may be truncated; inspect the workspace directly when exact details matter.",
        handoff_path.display()
    )
}
