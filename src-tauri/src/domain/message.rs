use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaMessage {
    pub partition: u32,
    pub offset: i64,
    pub key: Option<String>,
    pub value: String,
    /// Epoch millis.
    pub timestamp: i64,
}

/// Filtro de inspeção de mensagens (RF-001). Todos os campos são opcionais e
/// combináveis: quando mais de um é informado, a mensagem precisa satisfazer
/// todos (AND), não apenas um deles.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageFilter {
    pub content_contains: Option<String>,
    pub key_equals: Option<String>,
    pub timestamp_from: Option<i64>,
    pub timestamp_to: Option<i64>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    100
}

impl MessageFilter {
    pub fn matches(&self, message: &KafkaMessage) -> bool {
        if let Some(needle) = &self.content_contains {
            if !message.value.contains(needle.as_str()) {
                return false;
            }
        }
        if let Some(key) = &self.key_equals {
            if message.key.as_deref() != Some(key.as_str()) {
                return false;
            }
        }
        if let Some(from) = self.timestamp_from {
            if message.timestamp < from {
                return false;
            }
        }
        if let Some(to) = self.timestamp_to {
            if message.timestamp > to {
                return false;
            }
        }
        true
    }
}
