use ferrogram::prelude::*;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

// Состояние одной игры в крестики-нолики
#[derive(Clone, Debug)]
struct GameState {
    player_x: i64,          // ID создателя вызова (играет за ❌)
    player_x_name: String,
    player_o: Option<i64>,  // ID того, кто принял вызов (играет за ⭕)
    player_o_name: Option<String>,
    board: [char; 9],       // ' ' (пусто), 'X', 'O'
    turn: char,             // 'X' или 'O'
    winner: Option<char>,   // Some('X'), Some('O'), Some('D') (ничья)
}

impl GameState {
    fn new(player_x: i64, player_x_name: String) -> Self {
        Self {
            player_x,
            player_x_name,
            player_o: None,
            player_o_name: None,
            board: [' '; 9],
            turn: 'X',
            winner: None,
        }
    }

    fn check_winner(&self) -> Option<char> {
        let b = self.board;
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Горизонтали
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Вертикали
            [0, 4, 8], [2, 4, 6],             // Диагонали
        ];

        for &[i, j, k] in &lines {
            if b[i] != ' ' && b[i] == b[j] && b[j] == b[k] {
                return Some(b[i]);
            }
        }

        if b.iter().all(|&c| c != ' ') {
            return Some('D'); // Ничья (Draw)
        }

        None
    }

    fn render_keyboard(&self, game_id: &str) -> InlineKeyboardMarkup {
        let mut rows = Vec::new();
        for r in 0..3 {
            let mut row = Vec::new();
            for c in 0..3 {
                let idx = r * 3 + c;
                let symbol = match self.board[idx] {
                    'X' => "❌",
                    'O' => "⭕",
                    _ => "⬜",
                };
                let cb_data = format!("ttt:{}:{}", game_id, idx);
                row.push(InlineKeyboardButton::callback(symbol, cb_data));
            }
            rows.push(row);
        }

        // Кнопка перезапуска, если игра окончена
        if self.winner.is_some() {
            rows.push(vec![InlineKeyboardButton::callback("🔄 Новая игра", format!("ttt_new:{}", game_id))]);
        }

        InlineKeyboardMarkup::new(rows)
    }

    fn render_text(&self) -> String {
        let x_name = &self.player_x_name;
        let o_name = self.player_o_name.as_deref().unwrap_or("<i>Ожидание соперника...</i>");

        let mut text = format!(
            "⚔️ <b>Крестики-Нолики</b>\n\n❌ <b>Игрок X:</b> {}\n⭕ <b>Игрок O:</b> {}\n\n",
            x_name, o_name
        );

        match self.winner {
            Some('X') => text.push_str(&format!("🎉 <b>Победил ❌ {}!</b>", x_name)),
            Some('O') => text.push_str(&format!("🎉 <b>Победил ⭕ {}!</b>", self.player_o_name.as_deref().unwrap_or(""))),
            Some('D') => text.push_str("🤝 <b>Боевая ничья!</b>"),
            _ => {
                if self.player_o.is_none() {
                    text.push_str("⏳ <b>Нажмите на любую клетку, чтобы принять вызов и сделать ход!</b>");
                } else if self.turn == 'X' {
                    text.push_str(&format!("👉 Сейчас ход ❌ <b>{}</b>", x_name));
                } else {
                    text.push_str(&format!("👉 Сейчас ход ⭕ <b>{}</b>", self.player_o_name.as_deref().unwrap_or("")));
                }
            }
        }

        text
    }
}

type GamesMap = Arc<Mutex<HashMap<String, GameState>>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("⚠️ Укажите TELEGRAM_BOT_TOKEN!");
        std::process::exit(1);
    });

    let bot = Bot::new(token);
    let games: GamesMap = Arc::new(Mutex::new(HashMap::new()));

    let games_iq = games.clone();
    let games_cb = games.clone();

    let dispatcher = Dispatcher::new(bot)
        // 1. Обычная команда /start в личке бота
        .command("start", |bot, msg| {
            let text = "🎮 <b>Привет! Я бот для игры в Крестики-Нолики в инлайн-режиме!</b>\n\n\
                Ты можешь играть прямо в любом групповом чате со своими друзьями!\n\n\
                👉 Напиши в любом чате: <code>@имя_бота</code> и нажми на вызов игры!";
            bot.send_message(msg.chat.id, text)?;
            Ok(())
        })
        // 2. Инлайн-запрос (когда пользователь пишет @bot в любом чате)
        .inline_query(move |bot, iq| {
            let game_id = format!("{}_{}", iq.from.id, iq.id);
            let player_name = iq.from.first_name.clone();

            let state = GameState::new(iq.from.id, player_name.clone());
            let text = state.render_text();
            let keyboard = state.render_keyboard(&game_id);

            {
                let mut map = games_iq.lock().unwrap();
                map.insert(game_id.clone(), state);
            }

            let result = InlineQueryResult::Article(InlineQueryResultArticle {
                result_type: "article".into(),
                id: game_id,
                title: "⚔️ Бросить вызов в Крестики-Нолики".into(),
                description: Some(format!("Создать дуэль от имени {}", player_name)),
                input_message_content: InputTextMessageContent {
                    message_text: text,
                    parse_mode: Some("HTML".into()),
                },
                reply_markup: Some(keyboard),
            });

            bot.answer_inline_query(&iq.id, &[result])?;
            Ok(())
        })
        // 3. Обработка ходов по игровому полю (ttt:<game_id>:<cell>)
        .callback("ttt:", move |bot, query| {
            let data = query.data.clone().unwrap_or_default();
            let parts: Vec<&str> = data.split(':').collect();
            if parts.len() != 3 {
                return Ok(());
            }

            let game_id = parts[1];
            let cell_idx: usize = match parts[2].parse() {
                Ok(idx) if idx < 9 => idx,
                _ => return Ok(()),
            };

            let user_id = query.from.id;
            let user_name = query.from.first_name.clone();

            let mut alert_text = None;
            let mut need_update = false;
            let mut new_text = String::new();
            let mut new_keyboard = None;

            {
                let mut map = games_cb.lock().unwrap();
                if let Some(game) = map.get_mut(game_id) {
                    if game.winner.is_some() {
                        alert_text = Some("Игра уже завершена!");
                    } else if game.board[cell_idx] != ' ' {
                        alert_text = Some("Клетка уже занята!");
                    } else {
                        // Если второй игрок еще не присоединился
                        if game.player_o.is_none() {
                            if user_id == game.player_x {
                                // Первый ход делает создатель вызова (X)
                                game.board[cell_idx] = 'X';
                                game.turn = 'O';
                                game.winner = game.check_winner();
                                need_update = true;
                            } else {
                                // Соперник присоединяется к игре за O
                                game.player_o = Some(user_id);
                                game.player_o_name = Some(user_name);
                                game.board[cell_idx] = 'X'; // первый ход был X
                                game.turn = 'O';
                                game.winner = game.check_winner();
                                need_update = true;
                            }
                        } else {
                            // Оба игрока уже в игре. Проверяем чей сейчас ход
                            if game.turn == 'X' && user_id == game.player_x {
                                game.board[cell_idx] = 'X';
                                game.turn = 'O';
                                game.winner = game.check_winner();
                                need_update = true;
                            } else if game.turn == 'O' && Some(user_id) == game.player_o {
                                game.board[cell_idx] = 'O';
                                game.turn = 'X';
                                game.winner = game.check_winner();
                                need_update = true;
                            } else if user_id != game.player_x && Some(user_id) != game.player_o {
                                alert_text = Some("⚠️ Это дуэль между другими игроками!");
                            } else {
                                alert_text = Some("⏳ Сейчас не ваш ход!");
                            }
                        }
                    }

                    if need_update {
                        new_text = game.render_text();
                        new_keyboard = Some(game.render_keyboard(game_id));
                    }
                } else {
                    alert_text = Some("Игра не найдена или устарела.");
                }
            }

            if let Some(text) = alert_text {
                bot.answer_callback_query(&query.id, Some(text), false)?;
            } else {
                bot.answer_callback_query(&query.id, None, false)?;
            }

            if need_update {
                if let Some(inline_msg_id) = &query.inline_message_id {
                    bot.edit_inline_message_text(inline_msg_id, &new_text, new_keyboard)?;
                } else if let Some(msg) = query.message {
                    bot.edit_message_text(msg.chat.id, msg.message_id, &new_text, new_keyboard)?;
                }
            }

            Ok(())
        });

    dispatcher.run_polling().map_err(|e| e.into())
}
