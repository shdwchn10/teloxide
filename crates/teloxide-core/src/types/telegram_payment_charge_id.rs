use serde::{Deserialize, Serialize};

/// Identifier of a story.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct TelegramPaymentChargeId(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that `TelegramPaymentChargeId` is serialized as the underlying
    /// String
    #[test]
    fn deser() {
        let story_id = S { id: TelegramPaymentChargeId("some_id".into()) };
        let json = r#"{"id":"some_id"}"#;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct S {
            id: TelegramPaymentChargeId,
        }

        assert_eq!(serde_json::to_string(&story_id).unwrap(), json);
        assert_eq!(story_id, serde_json::from_str(json).unwrap());
    }
}
