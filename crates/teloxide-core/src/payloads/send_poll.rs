//! Generated by `codegen_payloads`, do not edit by hand.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::{
    BusinessConnectionId, InputPollOption, Message, MessageEntity, ParseMode, PollType, Recipient,
    ReplyMarkup, ReplyParameters, ThreadId,
};

impl_payload! {
    /// Use this method to send a native poll. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendPoll (SendPollSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Poll question, 1-300 characters
            pub question: String [into],
            /// A JSON-serialized list of 2-10 answer options
            pub options: Vec<InputPollOption> [collect],
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message will be sent
            pub business_connection_id: BusinessConnectionId,
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Mode for parsing entities in the question. See [formatting options] for more details. Currently, only custom emoji entities are allowed
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub question_parse_mode: ParseMode,
            /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of _question\_parse\_mode_
            pub question_entities: Vec<MessageEntity> [collect],
            /// True, if the poll needs to be anonymous, defaults to True
            pub is_anonymous: bool,
            /// Poll type, “quiz” or “regular”, defaults to “regular”
            #[serde(rename = "type")]
            pub type_: PollType,
            /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
            pub allows_multiple_answers: bool,
            /// 0-based identifier of the correct answer option, required for polls in quiz mode
            pub correct_option_id: u8,
            /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
            pub explanation: String [into],
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub explanation_parse_mode: ParseMode,
            /// List of special entities that appear in the poll explanation, which can be specified instead of _parse\_mode_
            pub explanation_entities: Vec<MessageEntity> [collect],
            /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
            pub open_period: u16,
            /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
            #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
            pub close_date: DateTime<Utc> [into],
            /// Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
            pub is_closed: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// Description of the message to reply to
            pub reply_parameters: ReplyParameters,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove a reply keyboard or to force a reply from the user. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
