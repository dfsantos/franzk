mod commands;
mod domain;
mod error;
mod kafka;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::clusters::list_clusters,
            commands::clusters::add_cluster_profile,
            commands::clusters::request_topic_creation,
            commands::clusters::request_topic_deletion,
            commands::approvals::list_pending_approvals,
            commands::approvals::decide_approval,
            commands::messages::list_topics,
            commands::messages::filter_messages,
            commands::local_env::create_local_topic,
            commands::local_env::list_local_topics,
            commands::local_env::produce_local_message,
            commands::local_env::consume_local_topic,
            commands::monitoring::get_cluster_health,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
