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
                "👋 <b>Welcome!</b>\n\nThis is an interactive bot demonstrating <b>Ferrogram</b> with inline keyboards and instant updates.",
                Some(ReplyMarkup::Inline(keyboard)),
            )?;
            Ok(())
        })
        .callback("btn_click", |bot, query| {
            bot.answer_callback_query(&query.id, Some("⚡ Lightning-fast click from Rust!"), false)?;

            if let Some(msg) = query.message {
                let updated_keyboard = InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::callback("✅ Clicked!", "btn_done"),
                ]]);
                bot.edit_message_text(
                    msg.chat.id,
                    msg.message_id,
                    "🎉 <b>Button processed!</b> Message was updated in-place with zero lag.",
                    Some(updated_keyboard),
                )?;
            }
            Ok(())
        })
        .callback("btn_stats", |bot, query| {
            bot.answer_callback_query(&query.id, Some("RAM: ~5 MB | Architecture: Pure Rust"), true)?;
            Ok(())
        })
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                bot.send_message(msg.chat.id, &format!("🦀 <b>Echo:</b> {}", text))?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
