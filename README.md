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

## Configuration

### Initial Setup

Copy the example configuration to your home directory:

```bash
mkdir -p ~/.ferrum-bot
cp config.example.json ~/.ferrum-bot/config.json
```

Then edit `~/.ferrum-bot/config.json` with your API keys and preferences.

### Configuration Structure

```json
{
  // Agent defaults
  "agents": {
    "defaults": {
      "workspace": "~/.ferrum-bot/workspace",
      "model": "anthropic/claude-opus-4-5",
      "max_tokens": 8192,
      "temperature": 0.7,
      "max_tool_iterations": 20
    }
  },

  // Communication channels
  "channels": {
    "whatsapp": { ... },
    "telegram": { ... },
    "discord": { ... },
    "feishu": { ... }
  },

  // LLM providers
  "providers": {
    "anthropic": { "api_key": "sk-ant-api03-..." },
    "openai": { "api_key": "sk-..." },
    "openrouter": { "api_key": "sk-or-v1-..." },
    "deepseek": { "api_key": "sk-..." },
    "groq": { "api_key": "gsk_..." },
    "zhipu": { "api_key": "..." },
    "dashscope": { "api_key": "..." },
    "vllm": { "api_base": "http://localhost:8000/v1" },
    "gemini": { "api_key": "..." },
    "moonshot": { "api_key": "..." },
    "aihubmix": { "api_key": "..." }
  },

  // Gateway settings
  "gateway": {
    "host": "0.0.0.0",
    "port": 18790
  },

  // Tool restrictions
  "tools": {
    "web": { ... },
    "exec": { "timeout": 60 },
    "restrict_to_workspace": true
  }
}
```

### Provider Configuration

Supported providers and their model aliases:

| Provider | API Key Field | Model Keywords |
|----------|--------------|-----------------|
| Anthropic | `providers.anthropic.api_key` | `anthropic`, `claude` |
| OpenAI | `providers.openai.api_key` | `openai`, `gpt` |
| OpenRouter | `providers.openrouter.api_key` | `openrouter` |
| DeepSeek | `providers.deepseek.api_key` | `deepseek` |
| Groq | `providers.groq.api_key` | `groq` |
| Zhipu AI | `providers.zhipu.api_key` | `zhipu`, `glm` |
| DashScope | `providers.dashscope.api_key` | `dashscope`, `qwen` |
| VLLM | `providers.vllm.api_base` | `vllm` |
| Gemini | `providers.gemini.api_key` | `gemini` |
| Moonshot AI | `providers.moonshot.api_key` | `moonshot`, `kimi` |
| AIHubMix | `providers.aihubmix.api_key` | `aihubmix` |

### Channel Configuration

#### WhatsApp Cloud API
```json
"channels": {
  "whatsapp": {
    "enabled": true,
    "cloud_api": {
      "access_token": "YOUR_WHATSAPP_TOKEN",
      "phone_number_id": "YOUR_PHONE_NUMBER_ID",
      "verify_token": "YOUR_VERIFY_TOKEN",
      "app_secret": "YOUR_APP_SECRET"
    }
  }
}
```

#### Telegram
```json
"channels": {
  "telegram": {
    "enabled": true,
    "token": "YOUR_BOT_TOKEN"
  }
}
```

#### Discord
```json
"channels": {
  "discord": {
    "enabled": true,
    "token": "YOUR_BOT_TOKEN",
    "intents": 37377
  }
}
```

#### Feishu (飞书)
```json
"channels": {
  "feishu": {
    "enabled": true,
    "app_id": "YOUR_APP_ID",
    "app_secret": "YOUR_APP_SECRET",
    "verification_token": "YOUR_VERIFICATION_TOKEN"
  }
}
```

### Tool Configuration

```json
"tools": {
  "restrict_to_workspace": true,
  "exec": {
    "timeout": 60
  },
  "web": {
    "search": {
      "api_key": "",
      "max_results": 5
    }
  }
}
```

## Operational Defaults

- Workspace restriction is enabled by default (`tools.restrict_to_workspace = true`)
- `exec` tool includes dangerous command guards and workspace checks
- `web_fetch` blocks non-http(s), localhost, and private/local IP targets
- Web and search tools use request timeouts to avoid hanging calls

## Project Direction

`ferrum-bot` focuses on dependable local operation, explicit safety boundaries, and iterative channel/runtime expansion.

The goal is not feature bloat; it is a robust agent core you can trust and extend.
