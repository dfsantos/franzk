use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::approval::ApprovalRequest;
use crate::domain::cluster::{ClusterProfile, Environment};
use crate::kafka::mock::MockKafkaClient;
use crate::kafka::KafkaClient;

/// Id de cluster fixo para o ambiente local do desenvolvedor (RF-002/RF-003).
/// Sempre existe, não exige aprovação e não aparece na lista de clusters
/// compartilhados.
pub const LOCAL_CLUSTER_ID: &str = "local";

pub struct AppState {
    pub clusters: Mutex<HashMap<String, ClusterProfile>>,
    pub approvals: Mutex<HashMap<String, ApprovalRequest>>,
    pub kafka: Box<dyn KafkaClient>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut clusters = HashMap::new();
        clusters.insert(
            LOCAL_CLUSTER_ID.to_string(),
            ClusterProfile {
                id: LOCAL_CLUSTER_ID.to_string(),
                name: "Ambiente Local".to_string(),
                environment: Environment::Local,
                bootstrap_servers: "localhost:9092".to_string(),
            },
        );
        Self {
            clusters: Mutex::new(clusters),
            approvals: Mutex::new(HashMap::new()),
            kafka: Box::new(MockKafkaClient::default()),
        }
    }
}
