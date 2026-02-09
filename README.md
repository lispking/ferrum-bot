# ferrum-bot

Rust-native personal agent runtime and CLI.

`ferrum-bot` is inspired by `nanobot`: the same practical agent idea, rebuilt with a cleaner Rust architecture, stronger runtime boundaries, and a more production-minded operator experience.

## Why ferrum-bot

- Rust-first runtime for predictable behavior and maintainability
- Local-first workflow with explicit filesystem and tool boundaries
- Unified model of `agent + tools + channels + cron`
- CLI ergonomics designed for daily use, not demos

## Core Capabilities

- Interactive agent CLI (`ferrum-bot agent`)
- OpenAI-compatible provider integration
- Tool framework:
  - `file` (`read`, `write`, `edit`, `list`)
  - `exec` (guarded shell execution)
  - `web` (`search`, `fetch`)
  - `message`, `cron`, `spawn`
- Persistent sessions and scheduled jobs
- Gateway runtime and channel manager
- WhatsApp Cloud API outbound adapter
- Scaffolded adapters for Telegram, Discord, and Feishu

## System Layout

Default data root: `~/.ferrum-bot`

- Config: `~/.ferrum-bot/config.json`
- Workspace: `~/.ferrum-bot/workspace`
- Sessions: `~/.ferrum-bot/sessions/*.jsonl`
- Cron store: `~/.ferrum-bot/cron/jobs.json`
- REPL history: `~/.ferrum-bot/history/agent.history`

## Quick Start

```bash
cargo build -p ferrumbot-cli
cargo run -p ferrumbot-cli -- onboard
cargo run -p ferrumbot-cli -- status
cargo run -p ferrumbot-cli -- agent
```

Single-shot message:

```bash
cargo run -p ferrumbot-cli -- agent -m "hello ferrum-bot"
```

Gateway mode:

```bash
cargo run -p ferrumbot-cli -- gateway
```

## CLI Commands

- `ferrum-bot onboard`
- `ferrum-bot status`
- `ferrum-bot agent [-m MESSAGE] [-s SESSION]`
- `ferrum-bot gateway [-p PORT] [--verbose]`
- `ferrum-bot channels status`
- `ferrum-bot cron list|add|remove|enable|run`

## Agent REPL Experience

Inside `ferrum-bot agent`:

- `/help`
- `/status`
- `/session <id>`
- `/new [name]`
- `/multi` (finish with `/end`)
- `/last`
- `/retry`
- `/clear`
- `/exit` or `/quit`

Line editor shortcuts:

- `Up` / `Down` history navigation
- `Ctrl+R` reverse history search
- `Ctrl+C` interrupt current input
- `Ctrl+D` clean exit

## Operational Defaults

- Workspace restriction is enabled by default (`tools.restrict_to_workspace = true`)
- `exec` tool includes dangerous command guards and workspace checks
- `web_fetch` blocks non-http(s), localhost, and private/local IP targets
- Web and search tools use request timeouts to avoid hanging calls

## Configuration Notes

Set provider keys and model in:

`~/.ferrum-bot/config.json`

For WhatsApp Cloud API, configure under:

`channels.whatsapp.cloudApi`

## Project Direction

`ferrum-bot` focuses on dependable local operation, explicit safety boundaries, and iterative channel/runtime expansion.

The goal is not feature bloat; it is a robust agent core you can trust and extend.
