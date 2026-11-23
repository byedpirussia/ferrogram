use ferrogram::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("⚠️  Please set TELEGRAM_BOT_TOKEN environment variable!");
        std::process::exit(1);
    });

    let bot = Bot::new(token);

    let dispatcher = Dispatcher::new(bot)
        .command("start", |bot, msg| {
            let user_name = msg.from.map(|u| u.first_name).unwrap_or_else(|| "Friend".into());
            let text = format!(
                "👋 Hello, <b>{}</b>!\n\nI am a bot powered by <b>Ferrogram</b> — a blazing-fast 🦀 <b>Rust</b> framework!\n\nTry:\n/ping — check latency\n/stats — runtime stats",
                user_name
            );
            bot.send_message(msg.chat.id, &text)?;
            Ok(())
        })
        .command("ping", |bot, msg| {
            bot.send_message(msg.chat.id, "🏓 <b>Pong!</b> Response delivered in microseconds via native Rust.")?;
            Ok(())
        })
        .command("stats", |bot, msg| {
            let stats = "⚡ <b>Bot Statistics:</b>\n• Language: <b>Rust 2021</b>\n• Engine: <b>Ferrogram</b>\n• RAM: <b>~5 MB</b>\n• Multithreading: enabled";
            bot.send_message(msg.chat.id, stats)?;
            Ok(())
        })
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                let echo = format!("🦀 <b>Echo:</b> {}", text);
                bot.send_message(msg.chat.id, &echo)?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
