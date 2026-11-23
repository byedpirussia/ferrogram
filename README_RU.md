# 🦀 Ferrogram (Русская версия)

<p align="center">
  <b>Легковесный и быстрый фреймворк для Telegram-ботов на чистом Rust.</b><br>
  <i>Удобная альтернатива Aiogram с мгновенным стартом и минимальным потреблением оперативной памяти. Проверен и совместим с JIT-компилятором Relax.</i>
</p>

---

## ⚡ Преимущества Ferrogram

Большинство ботов в Telegram создаются на Python (`aiogram`, `python-telegram-bot`). Это удобно для старта, но при росте нагрузки возникают сложности:
- **Расход памяти:** от 60 до 120 МБ RAM на один фоновый процесс.
- **Ограничения GIL:** однопоточный цикл событий может тормозить при тяжелых вычислениях.
- **Ошибки типизации в рантайме:** внезапные падения из-за отсутствующих полей (`KeyError` или `NoneType`).

**Ferrogram решает эти задачи:**
- 🚀 **Потребление всего ~5 МБ RAM** (на сервере с 512 МБ можно держать десятки ботов).
- ⚡ **Низкая задержка** благодаря компиляции в машинный код.
- 🛡️ **Надежность типов Rust:** проверка структуры данных еще до запуска.
- 🧵 **Многопоточная обработка:** команды выполняются параллельно.
- 🦀 **Чистый Rust:** без внешних библиотек на Си, простая сборка на Windows, Linux и macOS.

---

## 📊 Сравнение: Ferrogram и Aiogram 3.x

| Параметр | 🐍 Aiogram 3.x (Python) | 🦀 Ferrogram (Rust) |
| :--- | :--- | :--- |
| **Память в простое (RAM)** | ~80 - 120 МБ | **~4 - 6 МБ** |
| **Задержка диспетчера** | 15 - 35 мс | **0.5 - 2 мс** |
| **Модель параллелизма** | Однопоточный asyncio (GIL) | **Многопоточный пул** |
| **Контроль типов** | В рантайме (Pydantic) | **Во время компиляции** |
| **Способ развертывания** | Python, pip, виртуальное окружение | **Один бинарный файл** |

---

## 📦 Быстрый старт

Добавьте зависимость в ваш `Cargo.toml`:

```toml
[dependencies]
ferrogram = { git = "https://github.com/byedpirussia/ferrogram.git" }
```

### 🤖 Пример: Интерактивный бот с инлайн-кнопками

```rust
use ferrogram::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("Укажите переменную TELEGRAM_BOT_TOKEN");
    let bot = Bot::new(token);

    let dispatcher = Dispatcher::new(bot)
        // Обработка команды /start с отправкой инлайн-кнопок
        .command("start", |bot, msg| {
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("🔥 Нажми меня", "btn_click"),
                    InlineKeyboardButton::callback("📊 Статистика", "btn_stats"),
                ],
                vec![
                    InlineKeyboardButton::url("🦀 Репозиторий", "https://github.com/byedpirussia/ferrogram"),
                ],
            ]);

            bot.send_message_with_markup(
                msg.chat.id,
                "👋 <b>Добро пожаловать в Ferrogram!</b>\nБыстрый Telegram-бот на языке Rust.",
                Some(ReplyMarkup::Inline(keyboard)),
            )?;
            Ok(())
        })
        // Обработка нажатий на кнопки (callback query)
        .callback("btn_click", |bot, query| {
            bot.answer_callback_query(&query.id, Some("⚡ Мгновенный ответ!"), false)?;

            if let Some(msg) = query.message {
                bot.edit_message_text(
                    msg.chat.id,
                    msg.message_id,
                    "🎉 <b>Кнопка успешно обработана!</b> Сообщение обновлено на лету.",
                    None,
                )?;
            }
            Ok(())
        })
        .callback("btn_stats", |bot, query| {
            bot.answer_callback_query(&query.id, Some("RAM: ~5 MB | CPU: 0.01%"), true)?;
            Ok(())
        })
        // Обработка всех остальных текстовых сообщений
        .default_handler(|bot, msg| {
            if let Some(text) = msg.text {
                bot.send_message(msg.chat.id, &format!("🦀 <b>Эхо:</b> {}", text))?;
            }
            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
```

---

## 🏃 Запуск примеров

В PowerShell:
```powershell
$env:TELEGRAM_BOT_TOKEN="токен_вашего_бота_от_BotFather"
cargo run --example interactive_bot
```

В Linux / macOS:
```bash
export TELEGRAM_BOT_TOKEN="токен_вашего_бота_от_BotFather"
cargo run --example interactive_bot
```

---

## 📜 Лицензия

Проект распространяется под лицензией MIT.
