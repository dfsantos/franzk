use tauri::State;

use crate::domain::message::{KafkaMessage, MessageFilter};
use crate::domain::topic::TopicSummary;
use crate::error::AppResult;
use crate::state::AppState;

#[tauri::command]
pub fn list_topics(state: State<AppState>, cluster_id: String) -> AppResult<Vec<TopicSummary>> {
    state.kafka.list_topics(&cluster_id)
}

/// RF-001: inspeção de mensagens com filtro combinável por conteúdo, chave
/// e intervalo de timestamp.
#[tauri::command]
pub fn filter_messages(
    state: State<AppState>,
    cluster_id: String,
    topic_name: String,
    filter: MessageFilter,
) -> AppResult<Vec<KafkaMessage>> {
    state
        .kafka
        .filter_messages(&cluster_id, &topic_name, &filter)
}
