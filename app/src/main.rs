use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use salsa_core::ipc::{read_message, write_message, Request, Response, DEFAULT_SOCKET_PATH};
use salsa_core::lint::lint_snippets;
use salsa_core::model::{CaseMode, ContentType, DelimiterMode, ScopeRule, Snippet};
use salsa_store::Store;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "salsa-app", version, about = "Salsa app CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Ping,
    List,
    Add(AddArgs),
    Delete(DeleteArgs),
    Lint,
    Ui,
}

#[derive(Parser)]
struct AddArgs {
    #[arg(long)]
    trigger: String,
    #[arg(long)]
    label: String,
    #[arg(long)]
    content: String,
    #[arg(long, default_value = "plain")]
    content_type: ContentKind,
    #[arg(long, default_value = "preserve")]
    case_mode: CaseModeKind,
    #[arg(long, default_value = "any")]
    delimiter: DelimiterKind,
    #[arg(long)]
    delimiter_chars: Option<String>,
    #[arg(long)]
    profile_id: Option<String>,
    #[arg(long)]
    app_bundle_id: Option<String>,
    #[arg(long)]
    window_title: Option<String>,
    #[arg(long, default_value_t = 0)]
    priority: i32,
    #[arg(long, action = clap::ArgAction::Append)]
    tag: Vec<String>,
    #[arg(long, default_value_t = true)]
    enabled: bool,
}

#[derive(Parser)]
struct DeleteArgs {
    #[arg(long)]
    id: String,
}

#[derive(Clone, ValueEnum)]
enum ContentKind {
    Plain,
    Markdown,
}

#[derive(Clone, ValueEnum)]
enum CaseModeKind {
    Smart,
    Upper,
    Lower,
    Preserve,
}

#[derive(Clone, ValueEnum)]
enum DelimiterKind {
    Any,
    Word,
    Custom,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Ui) {
        Command::Ping => ping_agent(DEFAULT_SOCKET_PATH),
        Command::List => list_snippets(),
        Command::Add(args) => add_snippet(args),
        Command::Delete(args) => delete_snippet(args),
        Command::Lint => lint_db(),
        Command::Ui => run_ui(),
    }
}

fn ping_agent(socket_path: &str) -> anyhow::Result<()> {
    let stream = UnixStream::connect(socket_path)?;
    write_message(&stream, &Request::Ping)?;
    let response: Response = read_message(&stream)?;
    match response {
        Response::Pong => println!("agent pong"),
        Response::Error { message } => println!("agent error: {message}"),
    }
    Ok(())
}

fn list_snippets() -> anyhow::Result<()> {
    let store = Store::open(default_db_path()?)?;
    let snippets = store.list_snippets()?;
    for snippet in snippets {
        println!("{} -> {}", snippet.trigger, snippet.label);
    }
    Ok(())
}

fn add_snippet(args: AddArgs) -> anyhow::Result<()> {
    let store = Store::open(default_db_path()?)?;
    let now = chrono::Utc::now();

    let content_type = match args.content_type {
        ContentKind::Plain => ContentType::PlainText,
        ContentKind::Markdown => ContentType::Markdown,
    };

    let case_mode = match args.case_mode {
        CaseModeKind::Smart => CaseMode::Smart,
        CaseModeKind::Upper => CaseMode::Upper,
        CaseModeKind::Lower => CaseMode::Lower,
        CaseModeKind::Preserve => CaseMode::Preserve,
    };

    let delimiter_mode = match args.delimiter {
        DelimiterKind::Any => DelimiterMode::Any,
        DelimiterKind::Word => DelimiterMode::WordBoundary,
        DelimiterKind::Custom => DelimiterMode::Custom(
            args.delimiter_chars
                .clone()
                .unwrap_or_else(|| "".to_string()),
        ),
    };

    let mut scope = ScopeRule {
        app_rules: Vec::new(),
        profile_id: args
            .profile_id
            .as_ref()
            .and_then(|value| Uuid::parse_str(value).ok()),
    };

    if let Some(bundle_id) = args.app_bundle_id {
        scope.app_rules.push(salsa_core::model::AppRule {
            bundle_id,
            window_title_pattern: args.window_title,
            enabled: true,
        });
    }

    let snippet = Snippet {
        id: Uuid::new_v4(),
        trigger: args.trigger,
        label: args.label,
        content: args.content,
        content_type,
        tags: args.tag,
        enabled: args.enabled,
        case_mode,
        delimiter_mode,
        scope,
        priority: args.priority,
        created_at: now,
        updated_at: now,
    };

    store.insert_snippet(&snippet)?;
    println!("added {}", snippet.trigger);
    Ok(())
}

fn delete_snippet(args: DeleteArgs) -> anyhow::Result<()> {
    let store = Store::open(default_db_path()?)?;
    let id = Uuid::parse_str(&args.id).context("invalid uuid")?;
    store.delete_snippet(id)?;
    println!("deleted {}", args.id);
    Ok(())
}

fn lint_db() -> anyhow::Result<()> {
    let store = Store::open(default_db_path()?)?;
    let snippets = store.list_snippets()?;
    let issues = lint_snippets(&snippets);
    if issues.is_empty() {
        println!("no lint issues found");
        return Ok(());
    }

    for issue in issues {
        println!("{:?} {} {:?}", issue.kind, issue.trigger, issue.snippet_ids);
    }
    Ok(())
}

fn run_ui() -> anyhow::Result<()> {
    println!("UI shell not wired yet. Use --help for CLI commands.");
    Ok(())
}

fn default_db_path() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "salsa", "Salsa")
        .ok_or_else(|| anyhow::anyhow!("unable to resolve app support dir"))?;
    let dir = proj_dirs.data_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("salsa.db"))
}
