use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicSpec {
    pub name: String,
    pub partitions: u32,
    pub replication_factor: u16,
    pub retention_ms: i64,
    /// Lista de identificadores (usuários/grupos) com permissão de acesso.
    /// Vazio = sem restrição adicional além do controle do cluster.
    pub acl_principals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicSummary {
    pub cluster_id: String,
    pub name: String,
    pub partitions: u32,
    pub replication_factor: u16,
    pub retention_ms: i64,
}
