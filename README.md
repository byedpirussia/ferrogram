# 🦀 Ferrogram

<p align="center">
  <b>A blazing-fast, lightweight, pure-Rust Telegram Bot framework.</b><br>
  <i>An ergonomic Aiogram alternative designed for instant startups and tiny memory footprints. Tested and verified with the Relax JIT compiler (Rust on Rust).</i>
</p>

<p align="center">
  <a href="README_RU.md">🇷🇺 Читать на русском языке (Russian version)</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/RAM-~5MB-brightgreen.svg" alt="RAM usage">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License">
  <img src="https://img.shields.io/badge/compiler-Relax%20JIT%20compatible-purple.svg" alt="Relax JIT">
</p>

---

## ⚡ Why Ferrogram?

Most modern Telegram bots are written in Python (`aiogram`, `python-telegram-bot`). While easy to start with, Python bots quickly hit bottlenecks:
- **Memory Hungry:** ~60–120 MB RAM per idle process.
- **Latency & GIL:** Single-threaded event loops cause lag during heavy computation.
- **Runtime Crashes:** Unexpected `NoneType` or `KeyError` exceptions in production.

**Ferrogram** solves this by providing:
- 🚀 **~5 MB RAM footprint** (host 50+ bots on a $3/mo server).
- ⚡ **Microsecond dispatch latency** on pure native machine code.
- 🛡️ **Zero runtime type errors** thanks to Rust's type system.
- 🧵 **Multi-threaded task execution** out of the box.
- 🦀 **100% Pure Rust:** Zero C-dependencies, builds smoothly on Windows, Linux, and macOS.
- 🔄 **Relax JIT Ready:** Verified with [Makepad Relax](https://github.com/makepad/relax) for sub-second compile checks!

---

## 📊 Ferrogram vs. Aiogram 3.x

| Metric | 🐍 Aiogram 3.x (Python) | 🦀 Ferrogram (Rust) |
| :--- | :--- | :--- |
| **Idle Memory (RAM)** | ~80 - 120 MB | **~4 - 6 MB** |
| **Response Latency** | 15 - 35 ms | **0.5 - 2 ms** |
| **Concurrency** | Single-threaded (GIL) | **True Multithreading** |
| **Type Safety** | Runtime (Pydantic) | **Compile-time strict** |
| **Binary Deployment** | Python runtime + venv + pip | **Single static executable** |

---

## 📦 Quick Start

Add `ferrogram` to your `Cargo.toml`:

```toml
[dependencies]
ferrogram = { git = "https://github.com/byedpirussia/ferrogram.git" }
```

### 🤖 Example: Interactive Bot with Inline Buttons

```rust
use ferrogram::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let bot = Bot::new(token);

    let dispatcher = Dispatcher::new(bot)
        // Handle /start command with an interactive inline keyboard
        .command("start", |bot, msg| {
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("🔥 Click Me!", "btn_click"),
                    InlineKeyboardButton::callback("📊 Stats", "btn_stats"),
                ],
                vec![
                    InlineKeyboardButton::url("🦀 GitHub", "https://github.com/byedpirussia/ferrogram"),
                ],
            ]);

            bot.send_message_with_markup(
                msg.chat.id,
                "👋 <b>Welcome to Ferrogram!</b>\nA superfast Telegram bot written in Rust.",
                Some(ReplyMarkup::Inline(keyboard)),
            )?;
            Ok(())
        })
        // Handle button clicks
        .callback("btn_click", |bot, query| {
            bot.answer_callback_query(&query.id, Some("⚡ Lightning-fast callback!"), false)?;

            if let Some(msg) = query.message {
                bot.edit_message_text(
                    msg.chat.id,
                    msg.message_id,
                    "🎉 <b>Button processed!</b> Message was updated in-place without lag.",
                    None,
                )?;
            }
            Ok(())
        })
        .callback("btn_stats", |bot, query| {
            bot.answer_callback_query(&query.id, Some("RAM: ~5 MB | CPU: 0.01%"), true)?;
            Ok(())
        })
        // Default text handler
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                bot.send_message(msg.chat.id, &format!("🦀 <b>Rust Echo:</b> {}", text))?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
```

---

## 🚀 Running Prebuilt Binaries (No Rust or Compilers Required!)

If you just want to run a fast Telegram bot without installing Rust:

1. Download the latest `ferrogram-v0.1.0-windows-x64.zip` from [Releases](https://github.com/byedpirussia/ferrogram/releases).
2. Unzip the archive.
3. Open PowerShell in that folder and run:
   ```powershell
   # 1. Provide your Telegram Bot token from @BotFather
   $env:TELEGRAM_BOT_TOKEN="your_token_here"

   # 2. Run the prebuilt interactive bot!
   .\ferrogram-interactive-bot-windows-x64.exe
   ```

---

## 🏃 Running from Source

```bash
# Set your bot token
export TELEGRAM_BOT_TOKEN="your_token_from_botfather"

# On Windows PowerShell:
# $env:TELEGRAM_BOT_TOKEN="your_token_from_botfather"

# Run the interactive bot example
cargo run --example interactive_bot
```

---

## 📜 License

Licensed under the [MIT License](LICENSE).
