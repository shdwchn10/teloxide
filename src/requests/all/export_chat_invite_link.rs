use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::ChatId,
    Bot,
};

/// Use this method to generate a new invite link for a chat; any previously
/// generated link is revoked.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights.
///
/// ## Note
/// Each administrator in a chat generates their own invite links. Bots can't
/// use invite links generated by other administrators. If you want your bot to
/// work with invite links, it will need to generate its own link using
/// [`Bot::export_chat_invite_link`] – after this the link will become available
/// to the bot via the [`Bot::get_chat`] method. If your bot needs to generate a
/// new invite link replacing its previous one, use
/// [`Bot::export_chat_invite_link`] again.
///
/// [The official docs](https://core.telegram.org/bots/api#exportchatinvitelink).
///
/// [`Bot::export_chat_invite_link`]: crate::Bot::export_chat_invite_link
/// [`Bot::get_chat`]: crate::Bot::get_chat
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct ExportChatInviteLink {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
}

#[async_trait::async_trait]
impl RequestOld for ExportChatInviteLink {
    type Output = String;

    /// Returns the new invite link as `String` on success.
    async fn send(&self) -> ResponseResult<String> {
        net::request_json(self.bot.client(), self.bot.token(), "exportChatInviteLink", &self).await
    }
}

impl ExportChatInviteLink {
    pub(crate) fn new<C>(bot: Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
