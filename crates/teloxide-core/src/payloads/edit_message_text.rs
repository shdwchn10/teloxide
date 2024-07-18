//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    InlineKeyboardMarkup, LinkPreviewOptions, Message, MessageEntity, MessageId, ParseMode,
    Recipient,
};

impl_payload! {
    /// Use this method to edit text and [games] messages. On success, the edited Message is returned.
    ///
    /// See also: [`EditMessageTextInline`](crate::payloads::EditMessageTextInline)
    ///
    /// [games]: https://core.telegram.org/bots/api#games
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageText (EditMessageTextSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: Recipient [into],
            /// Identifier of the message to edit
            #[serde(flatten)]
            pub message_id: MessageId,
            /// New text of the message, 1-4096 characters after entities parsing
            pub text: String [into],
        }
        optional {
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in message text, which can be specified instead of _parse\_mode_
            pub entities: Vec<MessageEntity> [collect],
            /// Link preview generation options for the message
            pub link_preview_options: LinkPreviewOptions,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
