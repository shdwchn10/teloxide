use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to stop updating a live location message (sent via the bot)
/// before `live_period` expires.
///
/// On success, [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#stopmessagelivelocation).
///
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct StopInlineMessageLiveLocation {
    #[serde(skip_serializing)]
    bot: Bot,
    pub inline_message_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl RequestOld for StopInlineMessageLiveLocation {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "stopMessageLiveLocation", &self)
            .await
    }
}

impl StopInlineMessageLiveLocation {
    pub(crate) fn new<I>(bot: Bot, inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self { bot, inline_message_id, reply_markup: None }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
