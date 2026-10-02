use tauri::State;

use crate::domain::monitoring::ClusterHealth;
use crate::error::AppResult;
use crate::state::AppState;

/// RF-005: lag de consumers, estado dos brokers, partições e throughput.
#[tauri::command]
pub fn get_cluster_health(state: State<AppState>, cluster_id: String) -> AppResult<ClusterHealth> {
    state.kafka.cluster_health(&cluster_id)
}
