use crate::bot::Bot;
use crate::types::{CallbackQuery, InlineQuery, Message};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type MessageHandlerFn = Arc<dyn Fn(Bot, Message) -> Result<(), String> + Send + Sync>;
pub type CallbackHandlerFn = Arc<dyn Fn(Bot, CallbackQuery) -> Result<(), String> + Send + Sync>;
pub type InlineQueryHandlerFn = Arc<dyn Fn(Bot, InlineQuery) -> Result<(), String> + Send + Sync>;

pub struct Dispatcher {
    bot: Bot,
    command_handlers: Vec<(String, MessageHandlerFn)>,
    callback_handlers: Vec<(String, CallbackHandlerFn)>,
    inline_query_handler: Option<InlineQueryHandlerFn>,
    default_message_handler: Option<MessageHandlerFn>,
    default_callback_handler: Option<CallbackHandlerFn>,
}

impl Dispatcher {
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            command_handlers: Vec::new(),
            callback_handlers: Vec::new(),
            inline_query_handler: None,
            default_message_handler: None,
            default_callback_handler: None,
        }
    }

    pub fn command<F>(mut self, command: &str, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Result<(), String> + Send + Sync + 'static,
    {
        let cmd = if command.starts_with('/') {
            command.to_string()
        } else {
            format!("/{}", command)
        };
        self.command_handlers.push((cmd, Arc::new(handler)));
        self
    }

    pub fn callback<F>(mut self, prefix_or_exact: &str, handler: F) -> Self
    where
        F: Fn(Bot, CallbackQuery) -> Result<(), String> + Send + Sync + 'static,
    {
        self.callback_handlers
            .push((prefix_or_exact.to_string(), Arc::new(handler)));
        self
    }

    pub fn inline_query<F>(mut self, handler: F) -> Self
    where
        F: Fn(Bot, InlineQuery) -> Result<(), String> + Send + Sync + 'static,
    {
        self.inline_query_handler = Some(Arc::new(handler));
        self
    }

    pub fn default_handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(Bot, Message) -> Result<(), String> + Send + Sync + 'static,
    {
        self.default_message_handler = Some(Arc::new(handler));
        self
    }

    pub fn default_callback<F>(mut self, handler: F) -> Self
    where
        F: Fn(Bot, CallbackQuery) -> Result<(), String> + Send + Sync + 'static,
    {
        self.default_callback_handler = Some(Arc::new(handler));
        self
    }

    pub fn run_polling(self) -> Result<(), String> {
        println!("==================================================");
        println!("🚀 Запуск Ferrogram Telegram Engine (Relax JIT)");
        println!("==================================================");
        let me = self.bot.get_me()?;
        println!("🤖 Бот: @{} (ID: {})", me.username.unwrap_or_default(), me.id);
        println!("🎮 Инлайн-режим активирован: можно вызывать через @bot_username в любых чатах!");
        println!("==================================================");

        let mut offset = 0;

        loop {
            match self.bot.get_updates(Some(offset), 25) {
                Ok(updates) => {
                    for update in updates {
                        offset = update.update_id + 1;

                        // 1. Обработка Inline Query (когда в чате пишут @bot_username)
                        if let Some(iq) = update.inline_query {
                            if let Some(h) = &self.inline_query_handler {
                                let bot = self.bot.clone();
                                let h = h.clone();
                                thread::spawn(move || {
                                    if let Err(e) = h(bot, iq) {
                                        eprintln!("Ошибка inline_query: {}", e);
                                    }
                                });
                            }
                            continue;
                        }

                        // 2. Обработка нажатий на инлайн-кнопки
                        if let Some(cb) = update.callback_query {
                            if let Some(data) = &cb.data {
                                let mut handled = false;
                                for (prefix, handler) in &self.callback_handlers {
                                    if data == prefix || data.starts_with(prefix) {
                                        let bot = self.bot.clone();
                                        let cb_clone = cb.clone();
                                        let h = handler.clone();
                                        thread::spawn(move || {
                                            if let Err(e) = h(bot, cb_clone) {
                                                eprintln!("Ошибка callback: {}", e);
                                            }
                                        });
                                        handled = true;
                                        break;
                                    }
                                }

                                if !handled {
                                    if let Some(def_cb) = &self.default_callback_handler {
                                        let bot = self.bot.clone();
                                        let cb_clone = cb.clone();
                                        let h = def_cb.clone();
                                        thread::spawn(move || {
                                            if let Err(e) = h(bot, cb_clone) {
                                                eprintln!("Ошибка дефолтного callback: {}", e);
                                            }
                                        });
                                    }
                                }
                            }
                            continue;
                        }

                        // 3. Обработка обычных сообщений
                        if let Some(msg) = update.message {
                            if let Some(text) = &msg.text {
                                let mut handled = false;
                                for (cmd, handler) in &self.command_handlers {
                                    if text == cmd || text.starts_with(&format!("{} ", cmd)) {
                                        let bot = self.bot.clone();
                                        let msg_clone = msg.clone();
                                        let h = handler.clone();
                                        thread::spawn(move || {
                                            if let Err(e) = h(bot, msg_clone) {
                                                eprintln!("Ошибка команды: {}", e);
                                            }
                                        });
                                        handled = true;
                                        break;
                                    }
                                }

                                if !handled {
                                    if let Some(default_h) = &self.default_message_handler {
                                        let bot = self.bot.clone();
                                        let msg_clone = msg.clone();
                                        let h = default_h.clone();
                                        thread::spawn(move || {
                                            if let Err(e) = h(bot, msg_clone) {
                                                eprintln!("Ошибка обработчика сообщений: {}", e);
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Ошибка сети/поллинга: {}. Повтор через 3с...", err);
                    thread::sleep(Duration::from_secs(3));
                }
            }
        }
    }
}
