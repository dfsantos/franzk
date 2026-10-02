use tauri::State;

use crate::domain::message::{KafkaMessage, MessageFilter};
use crate::domain::topic::{TopicSpec, TopicSummary};
use crate::error::AppResult;
use crate::state::{AppState, LOCAL_CLUSTER_ID};

/// RF-002: criação de tópico local sem fluxo de aprovação e com
/// configuração mínima (nome, partições, retenção).
#[tauri::command]
pub fn create_local_topic(state: State<AppState>, spec: TopicSpec) -> AppResult<TopicSummary> {
    state.kafka.create_topic(LOCAL_CLUSTER_ID, &spec)
}

#[tauri::command]
pub fn list_local_topics(state: State<AppState>) -> AppResult<Vec<TopicSummary>> {
    state.kafka.list_topics(LOCAL_CLUSTER_ID)
}

#[tauri::command]
pub fn produce_local_message(
    state: State<AppState>,
    topic_name: String,
    key: Option<String>,
    value: String,
) -> AppResult<KafkaMessage> {
    state
        .kafka
        .produce_message(LOCAL_CLUSTER_ID, &topic_name, key, value)
}

#[tauri::command]
pub fn consume_local_topic(
    state: State<AppState>,
    topic_name: String,
    limit: usize,
) -> AppResult<Vec<KafkaMessage>> {
    let filter = MessageFilter {
        limit,
        ..Default::default()
    };
    state
        .kafka
        .filter_messages(LOCAL_CLUSTER_ID, &topic_name, &filter)
}
