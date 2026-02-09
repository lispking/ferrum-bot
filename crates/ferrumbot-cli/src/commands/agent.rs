use std::fs;
use std::future::Future;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use chrono::Local;
use ferrumbot_agent::AgentLoop;
use ferrumbot_config::{data_dir, load_config};
use ferrumbot_core::MessageBus;
use ferrumbot_runtime::init_tracing;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::app::AgentArgs;

pub async fn run(args: AgentArgs) -> Result<()> {
    let config = load_config(None)?;
    init_tracing(false);

    let model = config.agents.defaults.model.clone();
    let workspace = config.workspace_path();
    let bus = MessageBus::new(64);
    let agent = AgentLoop::from_config(bus, &config, None)?;

    if let Some(message) = args.message {
        let response = run_with_spinner(
            "thinking",
            agent.process_direct(&message, &args.session, "cli", "direct"),
        )
        .await?;
        println!("{response}");
        return Ok(());
    }

    let mut state = ReplState::new(args.session, model, workspace);
    let mut repl_input = ReplInput::new()?;

    print_banner(&state);

    loop {
        let input = match repl_input.read_line(&render_prompt(&state.session, state.turns + 1))? {
            ReadEvent::Line(line) => line,
            ReadEvent::Interrupted => {
                println!(
                    "{}",
                    paint("Interrupted (^C). Use /exit to quit.", "38;5;244")
                );
                continue;
            }
            ReadEvent::Eof => {
                println!("\nSession closed.");
                break;
            }
        };

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.starts_with('/') {
            match handle_command(input, &mut state, &mut repl_input)? {
                ReplAction::Continue => continue,
                ReplAction::Send(message) => {
                    send_user_turn(&agent, &mut state, message).await?;
                }
                ReplAction::Retry => {
                    retry_last_turn(&agent, &mut state).await?;
                }
                ReplAction::Exit => {
                    println!("Session closed.");
                    break;
                }
            }
            continue;
        }

        send_user_turn(&agent, &mut state, input.to_string()).await?;
    }

    repl_input.save_history()?;
    Ok(())
}

struct ReplState {
    session: String,
    model: String,
    workspace: PathBuf,
    turns: u64,
    last_user: Option<String>,
    last_response: Option<String>,
}

impl ReplState {
    fn new(session: String, model: String, workspace: PathBuf) -> Self {
        Self {
            session,
            model,
            workspace,
            turns: 0,
            last_user: None,
            last_response: None,
        }
    }
}

enum ReplAction {
    Continue,
    Send(String),
    Retry,
    Exit,
}

enum ReadEvent {
    Line(String),
    Interrupted,
    Eof,
}

enum ReplInput {
    Readline {
        editor: Box<DefaultEditor>,
        history_path: PathBuf,
    },
    Stdio,
}

impl ReplInput {
    fn new() -> Result<Self> {
        if io::stdin().is_terminal() && io::stdout().is_terminal() {
            let mut editor = DefaultEditor::new().context("failed to initialize line editor")?;
            let history_path = data_dir().join("history").join("agent.history");
            if let Some(parent) = history_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create history dir: {}", parent.display())
                })?;
            }
            if history_path.exists() {
                let _ = editor.load_history(&history_path);
            }
            return Ok(Self::Readline {
                editor: Box::new(editor),
                history_path,
            });
        }

        Ok(Self::Stdio)
    }

    fn read_line(&mut self, prompt: &str) -> Result<ReadEvent> {
        self.read_line_internal(prompt, true)
    }

    fn read_line_no_history(&mut self, prompt: &str) -> Result<ReadEvent> {
        self.read_line_internal(prompt, false)
    }

    fn save_history(&mut self) -> Result<()> {
        if let Self::Readline {
            editor,
            history_path,
        } = self
        {
            editor
                .save_history(history_path)
                .with_context(|| format!("failed to save history: {}", history_path.display()))?;
        }
        Ok(())
    }

    fn read_line_internal(&mut self, prompt: &str, add_history: bool) -> Result<ReadEvent> {
        match self {
            Self::Readline { editor, .. } => match editor.readline(prompt) {
                Ok(line) => {
                    if add_history && !line.trim().is_empty() {
                        let _ = editor.add_history_entry(line.as_str());
                    }
                    Ok(ReadEvent::Line(line))
                }
                Err(ReadlineError::Interrupted) => Ok(ReadEvent::Interrupted),
                Err(ReadlineError::Eof) => Ok(ReadEvent::Eof),
                Err(err) => Err(anyhow::anyhow!("line editor failed: {err}")),
            },
            Self::Stdio => {
                print!("{prompt}");
                io::stdout().flush()?;

                let mut line = String::new();
                let bytes = io::stdin().read_line(&mut line)?;
                if bytes == 0 {
                    return Ok(ReadEvent::Eof);
                }
                Ok(ReadEvent::Line(line))
            }
        }
    }
}

fn handle_command(
    input: &str,
    state: &mut ReplState,
    repl_input: &mut ReplInput,
) -> Result<ReplAction> {
    let mut parts = input.splitn(2, ' ');
    let cmd = parts.next().unwrap_or_default();
    let arg = parts.next().map(str::trim).unwrap_or_default();

    match cmd {
        "/help" => {
            print_help();
            Ok(ReplAction::Continue)
        }
        "/status" => {
            print_status(state);
            Ok(ReplAction::Continue)
        }
        "/clear" => {
            clear_screen()?;
            print_banner(state);
            Ok(ReplAction::Continue)
        }
        "/session" => {
            if arg.is_empty() {
                println!("Current session: {}", state.session);
            } else {
                state.session = sanitize_session(arg);
                println!("Switched to session: {}", state.session);
            }
            Ok(ReplAction::Continue)
        }
        "/new" => {
            state.session = if arg.is_empty() {
                format!("cli:{}", Local::now().format("%Y%m%d-%H%M%S"))
            } else {
                format!("cli:{}", sanitize_session_part(arg))
            };
            state.turns = 0;
            state.last_user = None;
            state.last_response = None;
            println!("Started new session: {}", state.session);
            Ok(ReplAction::Continue)
        }
        "/multi" => {
            let content = read_multiline(repl_input)?;
            if content.trim().is_empty() {
                println!("No content captured.");
                return Ok(ReplAction::Continue);
            }
            Ok(ReplAction::Send(content))
        }
        "/last" => {
            if let Some(last) = state.last_response.as_deref() {
                print_response(last, state.turns);
            } else {
                println!("No previous assistant response.");
            }
            Ok(ReplAction::Continue)
        }
        "/retry" => Ok(ReplAction::Retry),
        "/quit" | "/exit" => Ok(ReplAction::Exit),
        _ => {
            println!("Unknown command: {cmd}. Use /help.");
            Ok(ReplAction::Continue)
        }
    }
}

fn read_multiline(repl_input: &mut ReplInput) -> Result<String> {
    println!(
        "{}",
        paint(
            "Multi-line mode: finish input with a single line `/end`",
            "38;5;250"
        )
    );

    let mut lines = Vec::new();
    loop {
        let line = match repl_input.read_line_no_history(&paint("... ", "38;5;244"))? {
            ReadEvent::Line(line) => line,
            ReadEvent::Interrupted => {
                println!("{}", paint("Multi-line input interrupted.", "38;5;244"));
                return Ok(String::new());
            }
            ReadEvent::Eof => break,
        };

        let trimmed = line.trim_end();
        if trimmed == "/end" {
            break;
        }
        lines.push(trimmed.to_string());
    }

    Ok(lines.join("\n"))
}

fn sanitize_session(raw: &str) -> String {
    let sanitized = sanitize_session_part(raw);
    if sanitized.contains(':') {
        sanitized
    } else {
        format!("cli:{sanitized}")
    }
}

fn sanitize_session_part(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ':' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if cleaned.is_empty() {
        "session".to_string()
    } else {
        cleaned
    }
}

fn print_banner(state: &ReplState) {
    let logo = [
        " ___________  ________  __________  __  ___ ",
        "/ ____/  _/ / / / __ \\/ ____/ __ \\/  |/  / ",
        "/ /_   / // / / / /_/ / __/ / /_/ / /|_/ /  ",
        "/ __/ _/ // /_/ / _, _/ /___/ _, _/ /  / /   ",
        "/_/   /___/\\____/_/ |_/_____/_/ |_/_/  /_/    ",
    ];

    println!();
    for line in logo {
        println!("{}", paint(line, "1;38;5;45"));
    }
    println!(
        "{}",
        paint(
            "----------------------------------------------------------------------",
            "38;5;240"
        )
    );
    println!(
        "{} {}",
        paint("session   :", "1;37"),
        paint(&state.session, "38;5;81")
    );
    println!(
        "{} {}",
        paint("model     :", "1;37"),
        paint(&state.model, "38;5;229")
    );
    println!(
        "{} {}",
        paint("workspace :", "1;37"),
        paint(&state.workspace.display().to_string(), "38;5;152")
    );
    println!(
        "{} {}",
        paint("shortcuts :", "1;37"),
        paint(
            "/help /status /session <id> /new [name] /multi /last /retry /clear /exit",
            "38;5;250"
        )
    );
    println!(
        "{} {}",
        paint("editor    :", "1;37"),
        paint(
            "up/down history, Ctrl+R search, Ctrl+C interrupt, Ctrl+D exit",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "----------------------------------------------------------------------",
            "38;5;240"
        )
    );
    println!();
}

fn print_status(state: &ReplState) {
    println!("session: {}", state.session);
    println!("model: {}", state.model);
    println!("workspace: {}", state.workspace.display());
    println!("turns: {}", state.turns);
}

fn print_help() {
    println!("{}", paint("Available commands:", "1;37"));
    println!(
        "{}",
        paint("/help               Show this help", "38;5;250")
    );
    println!(
        "{}",
        paint(
            "/status             Show active session/model/workspace",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "/session <id>       Switch to an existing session id",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "/new [name]         Start a new session (auto timestamp if omitted)",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "/multi              Enter multi-line input mode (end with /end)",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "/last               Show the previous assistant response",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint(
            "/retry              Resend the previous user message",
            "38;5;250"
        )
    );
    println!(
        "{}",
        paint("/clear              Clear terminal screen", "38;5;250")
    );
    println!(
        "{}",
        paint("/exit or /quit      Exit interactive mode", "38;5;250")
    );
}

fn print_response(response: &str, turn: u64) {
    println!();
    println!(
        "{} {}",
        paint("assistant>", "1;36"),
        paint(&format!("[turn {turn}]"), "38;5;244")
    );
    println!("{response}");
    println!();
}

fn render_prompt(session: &str, next_turn: u64) -> String {
    format!(
        "{} {} {} ",
        paint(&format!("[{session}]"), "38;5;81"),
        paint(&format!("#{next_turn}"), "38;5;244"),
        paint("you>", "1;37")
    )
}

async fn retry_last_turn(agent: &AgentLoop, state: &mut ReplState) -> Result<()> {
    let Some(previous) = state.last_user.clone() else {
        println!("No previous user message to retry.");
        return Ok(());
    };

    send_user_turn(agent, state, previous).await
}

async fn send_user_turn(agent: &AgentLoop, state: &mut ReplState, message: String) -> Result<()> {
    let response = run_with_spinner(
        "thinking",
        agent.process_direct(&message, &state.session, "cli", "direct"),
    )
    .await?;

    state.turns += 1;
    state.last_user = Some(message);
    state.last_response = Some(response.clone());
    print_response(&response, state.turns);
    Ok(())
}

async fn run_with_spinner<F, T>(label: &str, fut: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    if !supports_ansi() {
        return fut.await;
    }

    let frames = ["|", "/", "-", "\\"];
    let mut idx = 0usize;
    let start = Instant::now();

    tokio::pin!(fut);
    loop {
        tokio::select! {
            out = &mut fut => {
                let elapsed = start.elapsed().as_millis();
                eprint!(
                    "\r{} {} {}ms{}\n",
                    paint("assistant", "1;36"),
                    paint("done in", "38;5;250"),
                    elapsed,
                    " ".repeat(16)
                );
                let _ = io::stderr().flush();
                return out;
            }
            _ = tokio::time::sleep(Duration::from_millis(90)) => {
                eprint!(
                    "\r{} {} {}",
                    paint("assistant", "1;36"),
                    paint(label, "38;5;250"),
                    paint(frames[idx % frames.len()], "38;5;81"),
                );
                let _ = io::stderr().flush();
                idx += 1;
            }
        }
    }
}

fn clear_screen() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

fn paint(text: &str, style: &str) -> String {
    if supports_ansi() {
        format!("\x1b[{style}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

fn supports_ansi() -> bool {
    io::stdout().is_terminal()
        && std::env::var_os("NO_COLOR").is_none()
        && std::env::var("TERM")
            .map(|term| term != "dumb")
            .unwrap_or(true)
}
