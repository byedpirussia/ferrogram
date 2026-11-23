use crate::types::*;
use std::sync::Arc;
use std::time::Duration;
use ureq::Agent;

#[derive(Clone)]
pub struct Bot {
    token: Arc<String>,
    api_url: Arc<String>,
    agent: Agent,
}

impl Bot {
    pub fn new(token: impl Into<String>) -> Self {
        let tok = token.into();
        let api_url = format!("https://api.telegram.org/bot{}", tok);
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(Duration::from_secs(65))
            .build();

        Self {
            token: Arc::new(tok),
            api_url: Arc::new(api_url),
            agent,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn get_me(&self) -> Result<User, String> {
        let url = format!("{}/getMe", self.api_url);
        let resp = self.agent.get(&url).call().map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<User> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok && body.result.is_some() {
            Ok(body.result.unwrap())
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn send_message(&self, chat_id: i64, text: &str) -> Result<Message, String> {
        self.send_message_with_markup(chat_id, text, None)
    }

    pub fn send_message_with_markup(
        &self,
        chat_id: i64,
        text: &str,
        reply_markup: Option<ReplyMarkup>,
    ) -> Result<Message, String> {
        let url = format!("{}/sendMessage", self.api_url);
        let payload = SendMessagePayload {
            chat_id,
            text,
            parse_mode: Some("HTML"),
            reply_markup,
        };

        let resp = self.agent.post(&url).send_json(&payload).map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<Message> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok && body.result.is_some() {
            Ok(body.result.unwrap())
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn answer_inline_query(
        &self,
        inline_query_id: &str,
        results: &[InlineQueryResult],
    ) -> Result<bool, String> {
        let url = format!("{}/answerInlineQuery", self.api_url);
        let payload = AnswerInlineQueryPayload {
            inline_query_id,
            results,
            cache_time: Some(1),
            is_personal: Some(true),
        };

        let resp = self.agent.post(&url).send_json(&payload).map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<bool> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok {
            Ok(body.result.unwrap_or(true))
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn answer_callback_query(
        &self,
        callback_query_id: &str,
        text: Option<&str>,
        show_alert: bool,
    ) -> Result<bool, String> {
        let url = format!("{}/answerCallbackQuery", self.api_url);
        let payload = AnswerCallbackQueryPayload {
            callback_query_id,
            text,
            show_alert: Some(show_alert),
        };

        let resp = self.agent.post(&url).send_json(&payload).map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<bool> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok {
            Ok(body.result.unwrap_or(true))
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn edit_message_text(
        &self,
        chat_id: i64,
        message_id: i64,
        text: &str,
        reply_markup: Option<InlineKeyboardMarkup>,
    ) -> Result<Message, String> {
        let url = format!("{}/editMessageText", self.api_url);
        let payload = EditMessageTextPayload {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
            text,
            parse_mode: Some("HTML"),
            reply_markup,
        };

        let resp = self.agent.post(&url).send_json(&payload).map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<Message> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok && body.result.is_some() {
            Ok(body.result.unwrap())
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn edit_inline_message_text(
        &self,
        inline_message_id: &str,
        text: &str,
        reply_markup: Option<InlineKeyboardMarkup>,
    ) -> Result<bool, String> {
        let url = format!("{}/editMessageText", self.api_url);
        let payload = EditMessageTextPayload {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            text,
            parse_mode: Some("HTML"),
            reply_markup,
        };

        let resp = self.agent.post(&url).send_json(&payload).map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<bool> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok {
            Ok(body.result.unwrap_or(true))
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }

    pub fn get_updates(&self, offset: Option<i64>, timeout_secs: u64) -> Result<Vec<Update>, String> {
        let mut url = format!("{}/getUpdates?timeout={}", self.api_url, timeout_secs);
        if let Some(off) = offset {
            url.push_str(&format!("&offset={}", off));
        }

        let resp = self.agent.get(&url).call().map_err(|e| format!("HTTP error: {}", e))?;
        let body: TelegramResponse<Vec<Update>> = resp.into_json().map_err(|e| format!("JSON error: {}", e))?;

        if body.ok && body.result.is_some() {
            Ok(body.result.unwrap())
        } else {
            Err(body.description.unwrap_or_else(|| "Unknown error".into()))
        }
    }
}
