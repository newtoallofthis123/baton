use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claudex", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    List {
        agent: String,
        #[arg(long)]
        last: bool,
        #[arg(long)]
        interactive: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Inspect {
        /// session reference like `claude:<id>` — omit when using --last or --interactive
        session: Option<String>,
        #[arg(long, value_name = "AGENT")]
        last: Option<String>,
        #[arg(long, value_name = "AGENT")]
        interactive: Option<String>,
        #[arg(long)]
        full: bool,
    },
    Handoff {
        source: Option<String>,
        target: String,
        #[arg(long, value_name = "AGENT")]
        last: Option<String>,
        #[arg(long, value_name = "AGENT")]
        interactive: Option<String>,
        #[arg(long)]
        no_launch: bool,
    },
    Settings {
        #[command(subcommand)]
        action: SettingsAction,
    },
}

#[derive(Subcommand)]
pub enum SettingsAction {
    Path,
    Show,
    Edit,
    Get { key: String },
    Set { key: String, value: String },
    AddRoot { agent: String, path: std::path::PathBuf },
    RemoveRoot { agent: String, path: std::path::PathBuf },
    ResetRoot { agent: String },
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::List { .. } => todo!("chapter E"),
        Command::Inspect { .. } => todo!("chapter E"),
        Command::Handoff { .. } => todo!("chapter E"),
        Command::Settings { .. } => todo!("chapter E"),
    }
}
