use crate::error::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "email")]
pub mod email_notification;
#[cfg(feature = "callback")]
pub mod post_callback;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoalescenceEvent {
    pub(crate) coalescence: Option<u64>,
    pub(crate) coalescence_group: Option<String>,
    #[serde(flatten)]
    pub(crate) event: Event,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub ty: String,
    pub params: HashMap<String, serde_json::Value>,
}

#[async_trait]
pub trait EventTrait {
    fn new() -> Self
    where
        Self: Sized;

    fn get_type(&self) -> &str;

    fn validate(
        &self,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<(), String>;
    async fn trigger(
        &self,
        params: &HashMap<String, serde_json::Value>,
        facts: &serde_json::Value,
    ) -> Result<(), Error>;
}