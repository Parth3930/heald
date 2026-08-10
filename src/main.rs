use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "heald")]
#[command(about = "Canonical AI coding harness rules and memory store", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold .heald/ (project) or ~/.heald/ (global)
    Init {
        #[arg(long)]
        global: bool,
    },
    /// Compile canonical rules/skills into harness-specific format
    Sync {
        #[arg(long)]
        harness: Option<String>,
        #[arg(long)]
        all: bool,
    },
    /// Print assembled, pruned memory context
    Context {
        harness: String,
        #[arg(long)]
        budget: Option<usize>,
    },
    /// Append a new memory document
    Remember {
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: Option<String>,
        #[arg(long)]
        stdin: bool,
    },
    /// Run end-of-session consolidation
    Finalize {
        #[arg(long)]
        summary: Option<String>,
        #[arg(long)]
        stdin: bool,
    },
    /// Validate bundle integrity
    Doctor,
    /// Forget a memory document
    Forget {
        query: String,
        #[arg(short, long)]
        yes: bool,
    },
    /// Show memory docs that touched a given file
    Blame {
        path: String,
        #[arg(long)]
        json: bool,
    },
    /// Generate repo map
    Map,
}

mod cmd;
mod okf;
mod xref;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { global } => cmd::init::run(*global),
        Commands::Sync { harness, all } => cmd::sync::run(harness.as_deref(), *all),
        Commands::Context { harness, budget } => cmd::context::run(harness, *budget),
        Commands::Remember { r#type, title, body, stdin } => cmd::remember::run(r#type, title, body.as_deref(), *stdin),
        Commands::Finalize { summary, stdin } => cmd::finalize::run(summary.as_deref(), *stdin),
        Commands::Doctor => cmd::doctor::run(),
        Commands::Forget { query, yes } => cmd::forget::run(query, *yes),
        Commands::Blame { path, json } => cmd::blame::run(path, *json),
        Commands::Map => cmd::map::run(),
    }
}
