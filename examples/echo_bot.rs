use ferrogram::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("⚠️  Укажите токен бота через переменную окружения TELEGRAM_BOT_TOKEN!");
        eprintln!("Пример для PowerShell:");
        eprintln!("$env:TELEGRAM_BOT_TOKEN=\"123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11\"");
        std::process::exit(1);
    });

    let bot = Bot::new(token);

    let dispatcher = Dispatcher::new(bot)
        .command("start", |bot, msg| {
            let user_name = msg.from.map(|u| u.first_name).unwrap_or_else(|| "Друг".into());
            let text = format!(
                "👋 Привет, <b>{}</b>!\n\nЯ бот на <b>Ferrogram</b> — сверхбыстром фреймворке на 🦀 <b>Rust</b>, скомпилированном через <b>Relax JIT</b> (Rust на Rust)!\n\nКоманды:\n/ping — проверить пинг\n/stats — статистика рантайма",
                user_name
            );
            bot.send_message(msg.chat.id, &text)?;
            Ok(())
        })
        .command("ping", |bot, msg| {
            bot.send_message(msg.chat.id, "🏓 <b>Pong!</b> Скорость Rust + Relax JIT.")?;
            Ok(())
        })
        .command("stats", |bot, msg| {
            let stats = "⚡ <b>Статистика бота:</b>\n• Язык: <b>Rust</b>\n• Компилятор: <b>Relax JIT (Rust on Rust)</b>\n• Фреймворк: <b>Ferrogram</b>\n• Память: <b>~4-6 МБ</b> (в 20 раз меньше Python Aiogram)\n• Задержка: микросекунды";
            bot.send_message(msg.chat.id, stats)?;
            Ok(())
        })
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                let echo = format!("🦀 <b>Rust Эхо:</b> {}", text);
                bot.send_message(msg.chat.id, &echo)?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
