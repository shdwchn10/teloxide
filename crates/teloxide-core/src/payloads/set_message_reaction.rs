//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{MessageId, ReactionType, Recipient, True};

impl_payload! {
    /// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetMessageReaction (SetMessageReactionSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
            pub chat_id: Recipient [into],
            /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
            #[serde(flatten)]
            pub message_id: MessageId,
        }
        optional {
            /// New list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators.
            pub reaction: Vec<ReactionType> [collect],
            /// Pass True to set the reaction with a big animation
            pub is_big: bool,
        }
    }
}
