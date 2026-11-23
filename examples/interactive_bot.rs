use ferrogram::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("⚠️ Укажите TELEGRAM_BOT_TOKEN!");
        std::process::exit(1);
    });

    let bot = Bot::new(token);

    let dispatcher = Dispatcher::new(bot)
        // Команда /start с меню инлайн-кнопок
        .command("start", |bot, msg| {
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("🔥 Нажми меня!", "btn_click"),
                    InlineKeyboardButton::callback("📊 Статистика", "btn_stats"),
                ],
                vec![
                    InlineKeyboardButton::url("🦀 Репозиторий Relax JIT", "https://github.com/makepad/relax"),
                ],
            ]);

            bot.send_message_with_markup(
                msg.chat.id,
                "👋 <b>Добро пожаловать в бота нового поколения!</b>\n\nЭтот бот написан на <b>Ferrogram</b> (Rust) и скомпилирован через <b>Relax JIT</b>.\nПопробуй нажать на кнопки ниже:",
                Some(ReplyMarkup::Inline(keyboard)),
            )?;
            Ok(())
        })
        // Обработка клика по кнопке 'btn_click'
        .callback("btn_click", |bot, query| {
            // Всплывающее уведомление
            bot.answer_callback_query(&query.id, Some("⚡ Молниеносный клик из Rust!"), false)?;

            // Редактируем сообщение прямо на лету!
            if let Some(msg) = query.message {
                let updated_keyboard = InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::callback("✅ Клик получен!", "btn_done"),
                ]]);
                bot.edit_message_text(
                    msg.chat.id,
                    msg.message_id,
                    "🎉 <b>Кнопка успешно обработана!</b>\nОтвет вернулся за считанные доли миллисекунды.",
                    Some(updated_keyboard),
                )?;
            }
            Ok(())
        })
        // Обработка кнопки 'btn_stats'
        .callback("btn_stats", |bot, query| {
            bot.answer_callback_query(&query.id, Some("RAM: ~5 MB | CPU: 0.1%"), true)?;
            Ok(())
        })
        // Команда для отправки фото
        .command("photo", |bot, msg| {
            let photo_url = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
            bot.send_photo(
                msg.chat.id,
                photo_url,
                Some("🦀 <b>Официальный логотип Rust!</b>"),
                None,
            )?;
            Ok(())
        })
        // Эхо-обработчик
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                bot.send_message(msg.chat.id, &format!("🦀 <b>Rust Эхо:</b> {}", text))?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
