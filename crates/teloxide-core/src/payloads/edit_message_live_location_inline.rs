//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, ReplyMarkup, True};

impl_payload! {
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to [`StopMessageLiveLocation`]. On success, True is returned.
    ///
    /// See also: [`EditMessageLiveLocation`](crate::payloads::EditMessageLiveLocation)
    ///
    /// [`StopMessageLiveLocation`]: crate::payloads::StopMessageLiveLocation
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub EditMessageLiveLocationInline (EditMessageLiveLocationInlineSetters) => True {
        required {
            /// Identifier of the inline message
            pub inline_message_id: String [into],
            /// Latitude of new location
            pub latitude: f64,
            /// Longitude of new location
            pub longitude: f64,
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message to be edited was sent
            pub business_connection_id: BusinessConnectionId,
            /// The radius of uncertainty for the location, measured in meters; 0-1500
            pub horizontal_accuracy: f64,
            /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
            pub heading: u16,
            /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
            pub proximity_alert_radius: u32,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove a reply keyboard or to force a reply from the user. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
